use crate::unit::UNITS_LOOKUP;
use earlgrey::EarleyParser;
use earlgrey::{Grammar, GrammarBuilder};
use std::str::FromStr;

pub fn parser() -> EarleyParser {
    EarleyParser::new(grammar())
}

fn grammar() -> Grammar {
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
        .rule("expr", &["expr", "group"])
        .rule("expr", &["group"])
        .rule("expr", &["num"])
        .rule("+quantity", &["+num", "units"])
        .rule("+quantity", &["+num"])
        .rule("-quantity", &["-num", "units"])
        .rule("-quantity", &["-num"])
        .rule("quantity", &["num", "units"])
        .rule("quantity", &["num"])
        .rule("quantity", &["group", "[^]", "num"])
        .rule("quantity", &["num", "[^]", "num"])
        .rule("quantity", &["num", "[^]", "+num"])
        .rule("quantity", &["num", "[^]", "-num"])
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
        .rule("num", &["num", "[^]", "num"])
        .rule("+num", &["+num"])
        .rule("-num", &["-num"])
        .into_grammar("equation")
        .expect("Bad Gramar")
}

#[cfg(test)]
mod tests {
    use super::*;
    use earlgrey::EarleyParser;
    fn parse_test(input: &str) -> bool {
        let grammar = grammar();
        EarleyParser::new(grammar)
            .parse(input.split_whitespace())
            .is_ok()
    }
    #[test]
    fn test_basic_arithmetic() {
        assert!(parse_test("1. + 1 - 2 * 10.34 / .5"));
        assert!(parse_test("80 - 4 - 4"));
        assert!(parse_test("80 -4 -4"));
        assert!(parse_test("( 2 ) ( 2 )"));
    }
    #[test]
    fn test_dimensional_analysis() {
        assert!(parse_test("1. m / s ^ 2 + 1 m / s ^ 2 - 2 m / s ^ 2"));
        assert!(parse_test("10 ^ -1 s"));
    }
}
