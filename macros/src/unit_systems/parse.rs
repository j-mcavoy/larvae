use super::{Unit, UnitSystem, UnitSystems};
use syn::parse::{Parse, ParseStream, Result};
use syn::{LitFloat, LitStr, Token};
// name, abbrev, symbol, 10^x
const METRIC_PREFIXES: &[(&str, &str, &str, f64)] = &[
    ("yotta", "Y", "Y", 24.),
    ("zetta", "Z", "Z", 21.),
    ("exa", "E", "E", 18.),
    ("peta", "P", "P", 15.),
    ("tera", "T", "T", 12.),
    ("giga", "G", "G", 9.),
    ("mega", "M", "M", 6.),
    ("kilo", "k", "k", 3.),
    ("hecto", "h", "h", 2.),
    ("deca", "da", "da", 1.),
    ("deci", "d", "d", -1.),
    ("centi", "c", "c", -2.),
    ("milli", "m", "m", -3.),
    ("micro", "u", "μ", -6.),
    ("nano", "n", "n", -9.),
    ("pico", "p", "p", -12.),
    ("femto", "f", "f", -15.),
    ("atto", "a", "a", -18.),
    ("zepto", "z", "z", -21.),
    ("yocto", "y", "y", -24.),
];

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
                    abbrev: abbrev.clone(),
                    symbol: symbol.clone(),
                    conversion_factor,
                });
                if input.parse::<Token![!]>().is_ok() {
                    // metric flag
                    for (pre, abr, sym, pow) in METRIC_PREFIXES {
                        let name: LitStr =
                            LitStr::new(&(pre.to_string() + &name.value()), dim_l.span());
                        let symbol: LitStr =
                            LitStr::new(&(sym.to_string() + &symbol.value()), dim_l.span());
                        let abbrev: LitStr =
                            LitStr::new(&(abr.to_string() + &abbrev.value()), dim_l.span());

                        let conversion_factor =
                            LitFloat::new(&format! {"1e{}", *pow}, dim_l.span());
                        units.push(Unit {
                            name,
                            abbrev,
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
