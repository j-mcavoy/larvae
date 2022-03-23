pub mod grammer;
pub mod semanter;
pub mod tokenizer;
use earlgrey::EarleyParser;
use grammer::build_grammar;

pub fn build_parser() -> EarleyParser {
    EarleyParser::new(build_grammar())
}
#[cfg(test)]
mod tests {
    use super::semanter::build_semanter;
    use crate::equation::build_parser;

    use super::tokenizer::tokenizer;

    #[test]
    fn test_no_missing_actions() {
        let parser = build_parser();
        let semanter = build_semanter();
        let input = "1m+2m";
        let tokens = tokenizer(input.chars());
        assert!(semanter.eval(&parser.parse(tokens).unwrap()).is_ok());
    }
}
