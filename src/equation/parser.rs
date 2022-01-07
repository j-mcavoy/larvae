use std::cell::RefCell;

use crate::{quantity::Quantity, unit::length::Length};

static UNITS_L: &'static [&str] = &["m"];

pub fn build_grammar() -> earlgrey::Grammar {
    use std::str::FromStr;
    earlgrey::GrammarBuilder::default()
        .nonterm("expr")
        .nonterm("quantity")
        .nonterm("units")
        .terminal("unit_l", |n| UNITS_L.contains(&n))
        .terminal("value", |n| f64::from_str(n).is_ok())
        .terminal("^", |n| n == "^")
        .terminal("+", |n| n == "+")
        .terminal("-", |n| n == "-")
        .rule("expr", &["quantity"])
        .rule("expr", &["expr", "+", "quantity"])
        .rule("expr", &["expr", "-", "quantity"])
        .rule("quantity", &["value", "units"])
        .rule("units", &["unit_l"])
        .rule("value", &["value"])
        .into_grammar("expr")
        .expect("Bad Gramar")
}

pub struct Tokenizer<I: Iterator<Item = char>>(lexers::Scanner<I>);

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.scan_whitespace();
        self.0
            .scan_math_op()
            .or_else(|| self.0.scan_number())
            .or_else(|| self.0.scan_identifier())
    }
}

pub fn tokenizer<I: Iterator<Item = char>>(input: I) -> Tokenizer<I> {
    Tokenizer(lexers::Scanner::new(input))
}

pub fn gamma(x: f64) -> f64 {
    #[link(name = "m")]
    extern "C" {
        fn tgamma(x: f64) -> f64;
    }
    unsafe { tgamma(x) }
}

pub fn semanter<'a>() -> earlgrey::EarleyForest<'a, Quantity> {
    use std::str::FromStr;
    let mut ev = earlgrey::EarleyForest::new(|symbol, token| match symbol {
        "value" => Quantity::from_value(token.parse().unwrap()),
        "unit_l" => Quantity::from_units(Units {}),
        //"quantity" => Quantity::from_value(token.parse().unwrap()),
        _ => Quantity::default(),
    });
    ev.action("expr -> quantity", |n| n[0]);
    ev.action("quantity -> value + unit", |n| n[0] + n[2]);
    //ev.action("expr -> expr - quantity", |n| n[0] - n[2]);
    //ev.action("expr -> expr - quantity", |n| n[0] - n[2]);
    //ev.action("expr -> expr - quantity", |n| n[0] - n[2]);
    ev
}
