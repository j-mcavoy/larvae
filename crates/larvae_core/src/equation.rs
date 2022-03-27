pub mod grammar;
pub mod semanter;
pub mod tokenizer;
use earlgrey::EarleyParser;
pub use grammar::build_grammar;
pub use semanter::build_semanter;

pub fn build_parser() -> EarleyParser {
    EarleyParser::new(build_grammar())
}
#[cfg(test)]
mod tests {
    use super::semanter::build_semanter;
    use crate::{equation::build_parser, quantity::Quantity};

    use super::tokenizer::tokenizer;

    fn eval_test(input: &str) -> Result<Quantity, std::string::String> {
        let parser = build_parser();
        let semanter = build_semanter();
        let tokens = tokenizer(input.chars());
        semanter.eval(&parser.parse(tokens).unwrap())
    }

    #[test]
    fn test_eval_basic_arithmetic() {
        assert_eq!(eval_test("80 - 4 - 4").unwrap(), Quantity::from_value(72.));
        assert_eq!(eval_test("80-4-4").unwrap(), Quantity::from_value(72.));
        assert_eq!(eval_test("80+4+4").unwrap(), Quantity::from_value(88.));
        assert_eq!(eval_test("2+2").unwrap(), Quantity::from_value(4.));
        assert_eq!(eval_test("-2-2").unwrap(), Quantity::from_value(-4.));
    }
    #[test]
    fn test_dimensional_arithmetic() {
        let input = "1m+2m";
        assert!(eval_test(input).is_ok());
    }
}
