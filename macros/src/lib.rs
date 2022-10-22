mod unit_systems;
use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Ident, LitFloat, LitStr};
use unit_systems::{UnitSystems, BASE_DIMENSIONS};

fn str2ident(s: LitStr) -> Ident {
    Ident::new(&s.value(), Span::call_site())
}
#[proc_macro]
pub fn dimensions(input: TokenStream) -> TokenStream {
    let uss = parse_macro_input!(input as UnitSystems);
    let mut mod_output = quote! {};
    let mut map_output = quote! {};
    for us in uss.0 {
        let dim_ident = str2ident(us.dimension.clone());
        let dim_enum = Ident::new(
            &us.dimension.value().to_case(Case::UpperCamel),
            dim_ident.span(),
        );
        let dim_l = us.dim_l;
        let dim_m = us.dim_m;
        let dim_t = us.dim_t;
        let names_ident: Vec<Ident> = us.units.iter().map(|u| str2ident(u.name.clone())).collect();
        let names: Vec<LitStr> = us.units.iter().map(|u| u.name.clone()).collect();
        let symbols: Vec<LitStr> = us.units.iter().map(|u| u.symbol.clone()).collect();
        let abbrevs: Vec<LitStr> = us.units.iter().map(|u| u.abbrev.clone()).collect();

        for n in names_ident.clone() {
            map_output = quote! {
                #map_output
                m.insert(#dim_ident::#dim_enum::#n.name(), #dim_enum::#n.quantity());
                m.insert(#dim_ident::#dim_enum::#n.abbrev(), #dim_enum::#n.quantity());
                m.insert(#dim_ident::#dim_enum::#n.symbol(), #dim_enum::#n.quantity());
            }
        }
        let conversion_factors: Vec<LitFloat> = us
            .units
            .iter()
            .map(|u| u.conversion_factor.clone())
            .collect();

        let q_units = if BASE_DIMENSIONS.contains(&dim_ident.to_string().as_str()) {
            quote! {
                fn quantity(&self) -> Quantity {
                    Quantity {
                        value: self.conversion_factor(),
                        dimensions: self.dimensions(),
                        units: Units {
                            #dim_ident: *self,
                            ..Default::default()
                        },
                    }
                }
            }
        } else {
            quote! {
                fn quantity(&self) -> Quantity {
                    Quantity {
                        value: self.conversion_factor(),
                        dimensions: self.dimensions(),
                        units: Default::default(),
                    }
                }
            }
        };

        mod_output = quote! (
        #mod_output

        use #dim_ident::#dim_enum;
        pub mod #dim_ident {
            use super::*;
            use #dim_enum::*;

            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum #dim_enum {
                #(#names_ident),*
            }
            impl Unit for #dim_enum {
                fn dimensions(&self) -> Dimensions {
                    Dimensions {
                        length: #dim_l,
                        mass: #dim_m,
                        time: #dim_t,
                    }
                }
                fn abbrev(&self) -> &'static str {
                    match self {
                        #(#names_ident => #abbrevs),*
                    }
                }
                fn name(&self) -> &'static str {
                    match self {
                        #(#names_ident => #names),*
                    }
                }
                fn symbol(&self) -> &'static str {
                    match self {
                        #(#names_ident => #symbols),*
                    }
                }
                fn conversion_factor(&self) -> QuantityFloat {
                    match self {
                        #(#names_ident => #conversion_factors),*
                    }
                }

                #q_units
            }
        }
        );
    }

    quote! {
        lazy_static! {
            pub static ref UNITS_LOOKUP: HashMap<&'static str, Quantity> = {
                let mut m = HashMap::new();
                #map_output
                m
            };

        }
        #mod_output

    }
    .into()
}
