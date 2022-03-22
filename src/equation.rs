use crate::{quantity::Quantity, unit::UNITS_LOOKUP};
use earlgrey::EarleyParser;
use lexers::Scanner;
use std::{fmt::Display, str::Chars};

fn build_grammar() -> earlgrey::Grammar {
    use std::str::FromStr;
    earlgrey::GrammarBuilder::default()
        // non-terminals
        .nonterm("equation")
        .nonterm("expr")
        .nonterm("group")
        .nonterm("quantity")
        .nonterm("+quantity")
        .nonterm("-quantity")
        .nonterm("units")
        // operations
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
        // constants
        .terminal("e", |n| n == "e")
        .terminal("pi", |n| n == "pi")
        // functions
        .terminal("log", |n| n == "log")
        .terminal("+num", |n| n.starts_with("+") && f64::from_str(n).is_ok())
        .terminal("-num", |n| n.starts_with("-") && f64::from_str(n).is_ok())
        .terminal("num", |n| f64::from_str(n).is_ok())
        .terminal("sqrt", |n| n == "sqrt")
        .terminal("unit", |n| UNITS_LOOKUP.contains_key(n))
        // rules
        .rule("equation", &["expr", "[->]", "units"])
        .rule("equation", &["expr"])
        .rule("group", &["(", "expr", ")"])
        .rule("expr", &["expr", "[+]", "quantity"])
        .rule("expr", &["expr", "[-]", "quantity"])
        .rule("expr", &["expr", "[*]", "quantity"])
        .rule("expr", &["expr", "[/]", "quantity"])
        .rule("expr", &["expr", "[%]", "quantity"])
        .rule("expr", &["expr", "[!]"])
        .rule("expr", &["quantity"])
        .rule("expr", &["expr", "+quantity"])
        .rule("expr", &["expr", "-quantity"])
        .rule("+quantity", &["+num", "units"])
        .rule("+quantity", &["+num"])
        .rule("-quantity", &["-num", "units"])
        .rule("-quantity", &["-num"])
        .rule("quantity", &["num", "units"])
        .rule("quantity", &["num"])
        .rule("quantity", &["group", "[^]", "num"])
        .rule("quantity", &["num", "[^]", "num"])
        .rule("quantity", &["log", "group"])
        .rule("quantity", &["sqrt", "group"])
        .rule("quantity", &["e"])
        .rule("quantity", &["pi"])
        .rule("unit", &["unit"])
        .rule("units", &["unit", "[^]", "num"])
        .rule("units", &["units", "[/]", "units"])
        .rule("units", &["units", "units"])
        .rule("units", &["units", "[*]", "units"])
        .rule("units", &["unit"])
        .rule("num", &["num"])
        .rule("+num", &["+num"])
        .rule("-num", &["-num"])
        .into_grammar("equation")
        .expect("Bad Gramar")
}

pub fn semanter<'a>() -> earlgrey::EarleyForest<'a, Quantity> {
    let mut ev = earlgrey::EarleyForest::new(symbol_match);

    ev.action("num -> num", |n| n[0]);
    ev.action("num -> +num", |n| n[0]);
    ev.action("num -> -num", |n| n[0]);

    ev.action("+num -> +num", |n| n[0]);
    ev.action("-num -> -num", |n| n[0]);

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

    ev.action("expr -> quantity", |n| n[0]);
    ev.action("expr -> expr +quantity", |n| n[0].add(&n[1]).unwrap());
    ev.action("expr -> expr -quantity", |n| n[0].sub(&n[1]).unwrap());

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

    ev.action("quantity -> num", |n| n[0]);
    ev.action("quantity -> num units", |n| {
        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    });

    ev.action("+quantity -> +num", |n| n[0]);
    ev.action("+quantity -> +num units", |n| {
        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    });

    ev.action("-quantity -> -num", |n| n[0]);
    ev.action("-quantity -> -num units", |n| {
        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    });

    ev.action("quantity -> group [^] num", |n| n[0].pow(n[2].value));
    ev.action("quantity -> num [^] num", |n| n[0].pow(n[2].value));
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
    ev.action("quantity -> e", |n| n[0]);
    ev.action("quantity -> pi", |n| n[0]);

    ev.action("group -> ( expr )", |n| n[1]);

    ev.action("equation -> expr", |n| n[0]);
    ev.action("equation -> expr [->] units", |n| {
        n[0].set_units(&n[2].units)
    });
    ev
}

fn symbol_match(symbol: &str, token: &str) -> Quantity {
    match symbol {
        "+num" => Quantity::from_value(token.parse().unwrap()),
        "-num" => Quantity::from_value(token.parse().unwrap()),
        "num" => Quantity::from_value(token.parse().unwrap()),
        "e" => Quantity::from_value(std::f64::consts::E),
        "pi" => Quantity::from_value(std::f64::consts::PI),
        "unit" => *UNITS_LOOKUP.get(token).expect("invalid unit"),
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

pub struct Tokenizer<I: Iterator<Item = char>>(lexers::Scanner<I>);

impl<'a> Display for Tokenizer<Chars<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

impl<I: Iterator<Item = char>> Iterator for Tokenizer<I> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        self.0.scan_whitespace();
        self.0
            .scan_number()
            .or_else(|| self.0.scan_arrow())
            .or_else(|| self.0.scan_math_op())
            .or_else(|| self.0.scan_identifier())
            .or_else(|| {
                if let Some(unit) = self.0.larvae_scan_unit() {
                    Some(unit)
                } else {
                    None
                }
            })
    }
}

trait LarvaeScanner {
    fn larvae_scan_unit(&mut self) -> Option<String>;
    fn scan_arrow(&mut self) -> Option<String>;
}

impl<I: Iterator<Item = char>> LarvaeScanner for Scanner<I> {
    fn larvae_scan_unit(&mut self) -> Option<String> {
        for unit in crate::unit::UNITS_LOOKUP.keys() {
            let backtrack = self.buffer_pos();
            if self.accept_all(unit.chars()) {
                return Some(unit.to_string());
            } else {
                self.set_buffer_pos(backtrack);
            }
        }
        None
    }
    fn scan_arrow(&mut self) -> Option<String> {
        let arrow = "->".to_string();
        let backtack = self.buffer_pos();
        if self.accept_all(arrow.chars()) {
            return Some(arrow);
        } else {
            self.set_buffer_pos(backtack);
        }
        None
    }
}

pub fn tokenizer<I: Iterator<Item = char>>(input: I) -> Tokenizer<I> {
    Tokenizer(lexers::Scanner::new(input))
}

pub fn parser() -> EarleyParser {
    EarleyParser::new(build_grammar())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::length::Length::*;
    use crate::{dimension::Dimensions, unit::Units};

    #[test]
    pub fn test_tokenizer() {
        let input = "-123.456 m + 1e4 m -> m";
        let expected: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
        assert_eq!(expected, tokenizer(input.chars()).collect::<Vec<String>>());
    }
    #[test]
    pub fn test_parse_dimunits() {
        let input =
            "1kg^-2kg kg*e/e*log(10)*pi/pi*sqrt(1)!%2*1.123kilometer^2/s+100s^-1m*m+10km^2/s-0m^2/s-> m^3/m/s";
        let parsed: Vec<String> = tokenizer(input.chars()).collect();
        let expected: Vec<String> =
            "1 kg ^ -2 kg kg * e / e * log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s +100 s ^ -1 m * m +10 km ^ 2 / s -0 m ^ 2 / s -> m ^ 3 / m / s".split_whitespace().map(|s| s.to_string()).collect();
        assert_eq!(parsed, expected);
    }

    #[test]
    pub fn test_conversion() {
        let input = "1 m -> km";
        let tokens = tokenizer(input.chars());
        let state = parser().parse(tokens).unwrap();
        let out = semanter().eval(&state);
        let expected = Quantity {
            value: 1e-3,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
            units: Units {
                length: kilometer,
                ..Default::default()
            },
        };
        assert_eq!(out, Ok(expected));
    }

    #[test]
    pub fn test_eval_pos_neg_num() {
        let input = "1+2";
        let tokens = tokenizer(input.chars());
        let state = parser().parse(tokens).unwrap();
        let out = semanter().eval(&state);
        let expected = Quantity::from_value(3.);
        assert_eq!(out, Ok(expected));

        let input = "1m+2m";
        let tokens = tokenizer(input.chars());
        let state = parser().parse(tokens).unwrap();
        let out = semanter().eval(&state);
        let expected = Quantity {
            value: 3.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(out, Ok(expected));
    }
}
