mod parser;
mod semanter;
mod tokenizer;
pub use parser::parser;
pub use semanter::semanter;
pub use tokenizer::tokenizer;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::quantity::*;
    use crate::core::unit::length::Length::*;
    use crate::core::{dimension::Dimensions, unit::Units};

    pub fn calc(input: &str) -> Result<Quantity, String> {
        let tokens = tokenizer(input.chars());
        let state = parser().parse(tokens)?;
        semanter().eval(&state)
    }
    #[test]
    pub fn one_equals_one() {
        assert_eq!(calc("1").unwrap(), Quantity::from(1.));
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
}
