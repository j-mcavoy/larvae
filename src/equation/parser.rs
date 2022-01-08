use std::cell::RefCell;

use nom::number::complete::double;
use regex::{Regex, RegexBuilder};

use crate::{
    dimension::{Dimensions, StorageType},
    quantity::Quantity,
    unit::{length::Length, Unit, Units, UNITS_LOOKUP},
};

static UNITS: &'static [&str] = &["m"];

pub fn build_grammar() -> earlgrey::Grammar {
    use std::str::FromStr;
    earlgrey::GrammarBuilder::default()
        .nonterm("expr")
        .nonterm("quantity")
        .nonterm("units")
        .terminal("[+]", |n| n == "+")
        .terminal("[^]", |n| n == "^")
        .terminal("[/]", |n| n == "/")
        .terminal("[*]", |n| n == "*")
        .terminal("[-]", |n| n == "-")
        .terminal("[->]", |n| n == "->")
        .terminal("num", |n| f64::from_str(n).is_ok())
        .terminal("unit", |n| UNITS_LOOKUP.contains_key(n))
        .rule("expr", &["expr", "[->]", "units"])
        .rule("expr", &["expr", "[+]", "quantity"])
        .rule("expr", &["expr", "[-]", "quantity"])
        .rule("expr", &["expr", "[*]", "quantity"])
        .rule("expr", &["expr", "[/]", "quantity"])
        .rule("expr", &["quantity"])
        .rule("num", &["num"])
        .rule("quantity", &["num", "units"])
        .rule("quantity", &["num"])
        .rule("unit", &["unit"])
        .rule("units", &["unit", "[^]", "num"])
        .rule("units", &["units", "[/]", "units"])
        .rule("units", &["units", "units"])
        .rule("units", &["units", "[*]", "units"])
        .rule("units", &["unit"])
        .into_grammar("expr")
        .expect("Bad Gramar")
}
pub fn semanter<'a>() -> earlgrey::EarleyForest<'a, Quantity> {
    let mut ev = earlgrey::EarleyForest::new(symbol_match);
    ev.action("num -> num", |n| n[0]);
    ev.action("unit -> unit", |n| n[0]);
    ev.action("units -> unit [^] num", |n| {
        let mut q = n[0];
        q.dimensions = n[0].dimensions.pow(n[2].value);
        q
    });
    ev.action("units -> unit", |n| n[0]);
    ev.action("units -> units units", |n| n[0].mul(&n[1]));
    ev.action("units -> units [*] units", |n| n[0].mul(&n[2]));
    ev.action("units -> units [/] units", |n| n[0].mul(&n[1].inv()));
    ev.action("quantity -> num units", |n| {
        println!("{:?}", n);
        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    });
    ev.action("quantity -> num", |n| n[0]);
    ev.action("expr -> expr [+] quantity", |n| n[0].add(&n[2]).unwrap());
    ev.action("expr -> expr [-] quantity", |n| n[0].sub(&n[2]).unwrap());
    ev.action("expr -> expr [*] quantity", |n| n[0].mul(&n[2]));
    ev.action("expr -> expr [/] quantity", |n| n[0].mul(&n[2].inv()));
    ev.action("expr -> quantity", |n| n[0]);
    ev.action("expr -> expr [->] units", |n| {
        n[0].convert_units(&n[1].units)
    });
    ev
}

fn symbol_match(symbol: &str, token: &str) -> Quantity {
    match symbol {
        "num" => Quantity::from_value(token.parse().unwrap()),
        "unit" => {
            let mut q = Quantity::default();
            if let Some(q) = UNITS_LOOKUP.get(token) {
                *q
            } else {
                Quantity::default()
            }
        }
        _ => Quantity::default(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_parse_dimunits() {
        let input =
            "1 * 1.123 km ^ 2 + 100 m * m + 10 km ^ 2 - 0 m ^ 2 -> m ^ 3 / m".split_whitespace();
        println!("{:?}", input);
        let trees = earlgrey::EarleyParser::new(build_grammar())
            .parse(input)
            .unwrap();
        let evaler = semanter();
        let result = evaler.eval(&trees).unwrap();
        println!("{:?}", trees);
        assert_eq!(
            Quantity::new(
                11123100.,
                Dimensions {
                    length: 2.,
                    ..Default::default()
                },
                Units {
                    length: Length::Meter,
                    ..Default::default()
                }
            ),
            result
        );
    }
}
