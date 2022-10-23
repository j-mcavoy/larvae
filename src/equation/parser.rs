use earlgrey::EarleyParser;
use earlgrey::{Grammar, GrammarBuilder};

use std::str::FromStr;

use crate::core::UNITS_LOOKUP;

pub fn parser() -> EarleyParser {
    EarleyParser::new(grammar())
}
fn grammar() -> Grammar {
    GrammarBuilder::default()
        .nonterm("equation")
        .nonterm("expr")
        .nonterm("term")
        .nonterm("factor")
        .nonterm("power")
        .nonterm("ufact")
        .nonterm("group")
        .nonterm("func")
        .nonterm("args")
        .nonterm("units")
        .nonterm("quantity")
        .terminal("[n]", |n| f64::from_str(n).is_ok())
        .terminal("+", |n| n == "+")
        .terminal("-", |n| n == "-")
        .terminal("*", |n| n == "*")
        .terminal("/", |n| n == "/")
        .terminal("%", |n| n == "%")
        .terminal("^", |n| n == "^")
        .terminal("!", |n| n == "!")
        .terminal("(", |n| n == "(")
        .terminal(")", |n| n == ")")
        .terminal("ln", |n| n == "ln")
        .terminal("log", |n| n == "log")
        .terminal("sqrt", |n| n == "sqrt")
        .terminal("[->]", |n| n == "->")
        .terminal("unit", |n| UNITS_LOOKUP.contains_key(n))
        .rule("equation", &["expr"])
        .rule("equation", &["expr", "[->]", "units"])
        .rule("expr", &["term"])
        .rule("expr", &["expr", "+", "term"])
        .rule("expr", &["expr", "-", "term"])
        .rule("term", &["factor"])
        .rule("term", &["term", "*", "factor"])
        .rule("term", &["term", "/", "factor"])
        .rule("term", &["term", "%", "factor"])
        .rule("factor", &["power"])
        .rule("factor", &["-", "factor"])
        .rule("power", &["ufact"])
        .rule("power", &["ufact", "^", "factor"])
        .rule("ufact", &["group"])
        .rule("ufact", &["ufact", "!"])
        .rule("group", &["quantity"])
        .rule("group", &["(", "expr", ")"])
        .rule("group", &["log", "group"])
        .rule("group", &["ln", "group"])
        .rule("group", &["sqrt", "group"])
        .rule("quantity", &["[n]"])
        .rule("quantity", &["[n]", "units"])
        .rule("units", &["unit"])
        .rule("units", &["units", "units"])
        .rule("units", &["unit", "^", "[n]"])
        .rule("units", &["units", "*", "units"])
        .rule("units", &["units", "/", "units"])
        .into_grammar("equation")
        .expect("Bad Gramar")

    /*
    // non-terminals
    .nonterm("equation")
    .nonterm("expr")
    .nonterm("term")
    .nonterm("factor")
    .nonterm("power")
    .nonterm("ufact")
    .nonterm("group")
    // operations
    .terminal("+", |n| n == "+")
    .terminal("^", |n| n == "^")
    .terminal("/", |n| n == "/")
    .terminal("*", |n| n == "*")
    .terminal("-", |n| n == "-")
    .terminal("%", |n| n == "%")
    .terminal("!", |n| n == "!")
    .terminal("(", |n| n == "(")
    .terminal(")", |n| n == ")")
    //// functions
    .terminal("sqrt", |n| n == "sqrt")
    .terminal("num", |n| f64::from_str(n).is_ok())
    // rules
    .rule("equation", &["expr"])
    .rule("equation", &["expr", "->", "units"])
    //
    .rule("expr", &["term"])
    .rule("expr", &["expr", "+", "term"])
    .rule("expr", &["expr", "-", "term"])
    .rule("term", &["factor"])
    .rule("term", &["term", "*", "factor"])
    .rule("term", &["term", "/", "factor"])
    .rule("term", &["term", "%", "factor"])
    .rule("factor", &["power"])
    .rule("factor", &["-", "factor"])
    .rule("power", &["ufact"])
    .rule("power", &["ufact", "^", "factor"])
    .rule("ufact", &["group"])
    .rule("ufact", &["ufact", "!"])
    .rule("group", &["quantity"])
    .rule("group", &["(", "expr", ")"])
    //.rule("expr", &["expr", "quantity"])
    //.rule("expr", &["quantity"])
    .rule("num", &["num", "num"])
    .rule("num", &["num", "^", "num"])
    .rule("quantity", &["num"])
    .rule("quantity", &["num", "units"])
    .rule("quantity", &["expr", "units"])
    .rule("num", &["e"])
    .rule("num", &["pi"])
    .rule("unit", &["unit"])
    .rule("units", &["unit"])
    .rule("units", &["unit", "^", "num"])
    .rule("units", &["units", "/", "units"])
    .rule("units", &["units", "units"])
    .rule("units", &["units", "*", "units"])
    .into_grammar("equation")
    .expect("Bad Grammar")
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    fn init() {
        env_logger::init();
    }

    fn parse_test(input: &str) -> bool {
        match parser().parse(input.split_whitespace()) {
            Ok(_) => true,
            Err(x) => {
                debug!("{:?}", x);
                false
            }
        }
    }

    #[test]
    fn grammar_ok() {
        grammar();
    }

    #[test]
    fn nums() {
        assert!(parse_test("1"));
        assert!(parse_test("-1"));
        assert!(parse_test("-9.19402349"));
    }
    #[test]
    fn pos_nums() {
        assert!(parse_test("+1"));
    }
    #[test]
    fn neg_nums() {
        assert!(parse_test("-1"));
    }

    #[test]
    fn single_quantity() {
        assert!(parse_test("1 m"));
        assert!(parse_test("-1 m"));
        assert!(parse_test("-1 m"));
        assert!(parse_test("-1 m / s"));
        assert!(parse_test("-1 m / s ^ 2"));
    }

    #[test]
    fn factorial() {
        assert!(parse_test("5 !"));
    }

    #[test]
    fn log() {
        assert!(parse_test("log ( 100 )"));
        assert!(parse_test("log 100"));
    }

    #[test]
    fn ln() {
        assert!(parse_test("ln ( 100 )"));
        assert!(parse_test("ln 100"));
    }

    #[test]
    fn sqrt() {
        assert!(parse_test("sqrt ( 100 )"));
        assert!(parse_test("sqrt 100"));
    }

    #[test]
    fn basic_arithmetic() {
        assert!(parse_test("2 + 2"));
        assert!(parse_test("1. + 1 - 2 * 10.34 / 0.5"));
        assert!(parse_test("80 - 4 - 4"));
    }
    #[test]
    fn dimensional_analysis() {
        init();
        assert!(parse_test("1. m / s ^ 2 + 1 m / s ^ 2 - 2 m / s ^ 2"));
        assert!(parse_test("10 ^ 1 s"));
        assert!(parse_test("10 ^ -1 s"));
    }
}
