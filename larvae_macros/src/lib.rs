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
struct UnitSystems(Vec<UnitSystem>);
struct UnitSystem {
    dimension: LitStr,
    units: Vec<Unit>,
    dim_l: LitFloat,
    dim_m: LitFloat,
    dim_t: LitFloat,
}

const METRIC_PREFIXES: &[(&str, &str, f64)] = &[
    ("yotta", "Y", 24.),
    ("zetta", "Z", 21.),
    ("exa", "E", 18.),
    ("peta", "P", 15.),
    ("tera", "T", 12.),
    ("giga", "G", 9.),
    ("mega", "M", 6.),
    ("kilo", "k", 3.),
    ("hecto", "h", 2.),
    ("deca", "da", 1.),
    ("deci", "d", -1.),
    ("centi", "c", -2.),
    ("milli", "m", -3.),
    ("micro", "μ", -6.),
    ("nano", "n", -9.),
    ("pico", "p", -12.),
    ("femto", "f", -15.),
    ("atto", "a", -18.),
    ("zepto", "z", -21.),
    ("yocto", "y", -24.),
];
//($dim:ident :
//$($unit:ident $abbrev:ident $symbol:ident $d_l:tt $d_m:tt $d_t:tt),+
//)
impl Parse for UnitSystem {
    fn parse(input: ParseStream) -> Result<Self> {
        let dimension = input.parse()?;
        let dim_l: LitFloat = input.parse()?;
        let dim_m: LitFloat = input.parse()?;
        let dim_t: LitFloat = input.parse()?;
        input.parse::<Token![:]>()?;
        let mut units = vec![];

        while !input.is_empty() {
            let name: LitStr = input.parse()?;
            let abbrev: LitStr = input.parse()?;
            let symbol: LitStr = input.parse()?;
            let conversion_factor = input.parse()?;
            units.push(Unit {
                name: name.clone(),
                abbrev,
                symbol: symbol.clone(),
                conversion_factor,
            });
            if input.parse::<Token![!]>().is_ok() {
                // metric flag
                for (pre, sym, pow) in METRIC_PREFIXES {
                    let name: LitStr =
                        LitStr::new(&(pre.to_string() + &name.value()), dim_l.span());
                    let symbol: LitStr =
                        LitStr::new(&(sym.to_string() + &symbol.value()), dim_l.span());

                    let conversion_factor =
                        LitFloat::new(&format! {"{:.1}", 10.0f64.powf(*pow)}, dim_l.span());
                    units.push(Unit {
                        name,
                        abbrev: symbol.clone(),
                        symbol,
                        conversion_factor,
                    });
                }
            }
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

impl Parse for UnitSystems {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut unit_systems = vec![];
        while !input.is_empty() {
            let dimension = input.parse()?;
            let dim_l: LitFloat = input.parse()?;
            let dim_m: LitFloat = input.parse()?;
            let dim_t: LitFloat = input.parse()?;
            input.parse::<Token![:]>()?;
            let mut units = vec![];

            while input.parse::<Token![,]>().is_err() {
                let name: LitStr = input.parse()?;
                let abbrev: LitStr = input.parse()?;
                let symbol: LitStr = input.parse()?;
                let conversion_factor = input.parse()?;
                units.push(Unit {
                    name: name.clone(),
                    abbrev,
                    symbol: symbol.clone(),
                    conversion_factor,
                });
                if input.parse::<Token![!]>().is_ok() {
                    // metric flag
                    for (pre, sym, pow) in METRIC_PREFIXES {
                        let name: LitStr =
                            LitStr::new(&(pre.to_string() + &name.value()), dim_l.span());
                        let symbol: LitStr =
                            LitStr::new(&(sym.to_string() + &symbol.value()), dim_l.span());

                        let conversion_factor =
                            LitFloat::new(&format! {"{:.1}", 10.0f64.powf(*pow)}, dim_l.span());
                        units.push(Unit {
                            name,
                            abbrev: symbol.clone(),
                            symbol,
                            conversion_factor,
                        });
                    }
                }
            }
            unit_systems.push(UnitSystem {
                dimension,
                units,
                dim_l,
                dim_m,
                dim_t,
            });
        }
        Ok(UnitSystems(unit_systems))
    }
}

fn str2ident(s: LitStr) -> Ident {
    Ident::new(&s.value(), Span::call_site())
}

const BASE_DIMENSIONS: &[&str] = &["length", "mass", "time"];

#[proc_macro]
pub fn dimensions(input: TokenStream) -> TokenStream {
    let uss = parse_macro_input!(input as UnitSystems);
    let mut output = quote! {}.into();
    println!("{}", stringify!(input));
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

        output = quote! (
            #output

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
                        #(#names_ident => #names),*
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
    }
    output.into()
}
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
                    #(#names_ident => #names),*
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
