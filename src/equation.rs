use std::str::SplitWhitespace;

use crate::{
    quantity::Quantity,
    unit::{Unit, UNITS_LOOKUP},
};

static UNITS: &'static [&str] = &["m"];

pub fn build_grammar() -> earlgrey::Grammar {
    use std::str::FromStr;
    earlgrey::GrammarBuilder::default()
        .nonterm("expr")
        .nonterm("quantity")
        .nonterm("units")
        .nonterm("group")
        .nonterm("equation")
        .terminal("[+]", |n| n == "+")
        .terminal("[^]", |n| n == "^")
        .terminal("[/]", |n| n == "/")
        .terminal("[*]", |n| n == "*")
        .terminal("[-]", |n| n == "-")
        .terminal("[%]", |n| n == "%")
        .terminal("[!]", |n| n == "!")
        .terminal("[->]", |n| n == "->")
        .terminal("(", |n| n == "(")
        .terminal(")", |n| n == ")")
        .terminal("sqrt", |n| n == "sqrt")
        .terminal("log", |n| n == "log")
        .terminal("pi", |n| n == "pi")
        .terminal("num", |n| f64::from_str(n).is_ok())
        .terminal("unit", |n| UNITS_LOOKUP.contains_key(n))
        .rule("group", &["(", "expr", ")"])
        .rule("quantity", &["sqrt", "group"])
        .rule("quantity", &["log", "group"])
        .rule("equation", &["expr", "[->]", "units"])
        .rule("equation", &["expr"])
        .rule("expr", &["expr", "[+]", "quantity"])
        .rule("expr", &["expr", "[-]", "quantity"])
        .rule("expr", &["expr", "[*]", "quantity"])
        .rule("expr", &["expr", "[/]", "quantity"])
        .rule("expr", &["expr", "[%]", "quantity"])
        .rule("expr", &["expr", "[!]"])
        .rule("expr", &["quantity"])
        .rule("quantity", &["pi"])
        .rule("num", &["num"])
        .rule("quantity", &["num", "units"])
        .rule("quantity", &["num"])
        .rule("unit", &["unit"])
        .rule("units", &["unit", "[^]", "num"])
        .rule("units", &["units", "[/]", "units"])
        .rule("units", &["units", "units"])
        .rule("units", &["units", "[*]", "units"])
        .rule("units", &["unit"])
        .into_grammar("equation")
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
    ev.action("units -> units [/] units", |n| n[0].mul(&n[2].inv()));
    ev.action("quantity -> num units", |n| {
        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    });
    ev.action("quantity -> num", |n| n[0]);
    ev.action("expr -> expr [+] quantity", |n| n[0].add(&n[2]).unwrap());
    ev.action("expr -> expr [-] quantity", |n| n[0].sub(&n[2]).unwrap());
    ev.action("expr -> expr [*] quantity", |n| n[0].mul(&n[2]));
    ev.action("expr -> expr [/] quantity", |n| n[0].mul(&n[2].inv()));
    ev.action("expr -> expr [%] quantity", |n| {
        let mut q = n[0];
        q.value %= n[2].value;
        q
    });
    ev.action("expr -> expr [!]", |n| {
        let mut q = n[0];
        q.value = gamma(q.value);
        q
    });
    ev.action("group -> ( expr )", |n| n[1]);
    ev.action("quantity -> sqrt group", |n| {
        let mut q = n[1];
        q.value = q.value.sqrt();
        q
    });
    ev.action("quantity -> log group", |n| {
        let mut q = n[1];
        q.value = q.value.log10();
        q
    });
    ev.action("expr -> quantity", |n| n[0]);
    ev.action("quantity -> pi", |n| n[0]);
    ev.action("equation -> expr", |n| n[0]);
    ev.action("equation -> expr [->] units", |n| {
        n[0].convert_units(&n[1].units)
    });
    ev
}

fn symbol_match(symbol: &str, token: &str) -> Quantity {
    match symbol {
        "num" => Quantity::from_value(token.parse().unwrap()),
        "pi" => Quantity::from_value(std::f64::consts::PI),
        "unit" => {
            if let Some(q) = UNITS_LOOKUP.get(token) {
                *q
            } else {
                println!("invalid unit");
                Quantity::default()
            }
        }
        _ => Quantity::default(),
    }
}

fn gamma(x: f64) -> f64 {
    #[link(name = "m")]
    extern "C" {
        fn tgamma(x: f64) -> f64;
    }
    unsafe { tgamma(x) }
}

pub fn tokenizer(input: &str) -> SplitWhitespace {
    input.split_whitespace()
}

#[cfg(test)]
mod test {
    use crate::{
        dimension::Dimensions,
        unit::{length::Length, Units},
    };

    use super::*;
    #[test]
    pub fn test_parse_dimunits() {
        let input =
            "log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s + 100 s ^ -1 m * m + 10 km ^ 2 / s - 0 m ^ 2 / s -> m ^ 3 / m / s"
                .split_whitespace();
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
                    time: -1.,
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
