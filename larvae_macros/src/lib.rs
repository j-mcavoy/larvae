use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, LitFloat, LitStr, Token};

struct Unit {
    name: LitStr,
    abbrev: LitStr,
    symbol: LitStr,
    conversion_factor: LitFloat,
}
struct UnitSystem {
    dimension: LitStr,
    units: Vec<Unit>,
    dim_l: LitFloat,
    dim_m: LitFloat,
    dim_t: LitFloat,
}

//($dim:ident :
//$($unit:ident $abbrev:ident $symbol:ident $d_l:tt $d_m:tt $d_t:tt),+
//)
impl Parse for UnitSystem {
    fn parse(input: ParseStream) -> Result<Self> {
        let dimension = input.parse()?;
        let dim_l = input.parse()?;
        let dim_m = input.parse()?;
        let dim_t = input.parse()?;
        input.parse::<Token![:]>()?;
        let mut units = vec![];
        while !input.is_empty() {
            let name = input.parse()?;
            let abbrev = input.parse()?;
            let symbol = input.parse()?;
            let conversion_factor = input.parse()?;
            units.push(Unit {
                name,
                abbrev,
                symbol,
                conversion_factor,
            });
        }
        Ok(UnitSystem {
            dimension,
            units,
            dim_l,
            dim_m,
            dim_t,
        })
    }
}

fn str2ident(s: LitStr) -> Ident {
    Ident::new(&s.value(), Span::call_site())
}

const BASE_DIMENSIONS: &[&str] = &["length", "mass", "time"];
#[proc_macro]
pub fn dimension(input: TokenStream) -> TokenStream {
    let us = parse_macro_input!(input as UnitSystem);

    println!("{}", stringify!(input));
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

    let output = quote! (
        pub mod #dim_ident {
        use super::*;
        use #dim_enum::*;

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
                    #(#names_ident => #symbols),*
                }
            }
            fn symbol(&self) -> &'static str {
                match self {
                    #(#names_ident => #symbols),*
                }
            }
            fn conversion_factor(&self) -> StorageType {
                match self {
                    #(#names_ident => #conversion_factors),*
                }
            }

            #q_units
        }
        }
    );
    output.into()
}
