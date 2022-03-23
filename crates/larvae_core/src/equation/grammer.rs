use crate::unit::UNITS_LOOKUP;
use earlgrey::{Grammar, GrammarBuilder};
use std::str::FromStr;

pub fn build_grammar() -> Grammar {
    GrammarBuilder::default()
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
        .terminal("ln", |n| n == "ln")
        .terminal("log", |n| n == "log")
        .terminal("+num", |n| n.starts_with('+') && f64::from_str(n).is_ok())
        .terminal("-num", |n| n.starts_with('-') && f64::from_str(n).is_ok())
        .terminal("num", |n| {
            !n.starts_with('+') && !n.starts_with('-') && f64::from_str(n).is_ok()
        })
        .terminal("sqrt", |n| n == "sqrt")
        .terminal("unit", |n| UNITS_LOOKUP.contains_key(n))
        // rules
        .rule("equation", &["expr", "[->]", "units"])
        .rule("equation", &["expr"])
        .rule("group", &["(", "expr", ")"])
        .rule("group", &["num", "[!]"])
        .rule("expr", &["expr", "quantity"])
        .rule("expr", &["expr", "[+]", "quantity"])
        .rule("expr", &["expr", "[-]", "quantity"])
        .rule("expr", &["expr", "[*]", "quantity"])
        .rule("expr", &["expr", "[/]", "quantity"])
        .rule("expr", &["expr", "[%]", "quantity"])
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
        .rule("quantity", &["ln", "group"])
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_arithmetic() {
        let grammar = build_grammar();
        let input = "1. + 1 - 2 * 10.34 / .5".split_whitespace();
        let parsed = earlgrey::EarleyParser::new(grammar).parse(input);
        assert!(parsed.is_ok());
    }
    #[test]
    fn test_dimensional_analysis() {
        let grammar = build_grammar();
        let input = "1. m / s ^ 2 + 1 m / s ^ 2 - 2 m / s ^ 2".split_whitespace();
        let parsed = earlgrey::EarleyParser::new(grammar).parse(input);
        assert!(parsed.is_ok());
    }
}
