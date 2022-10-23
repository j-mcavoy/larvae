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

    fn calc(input: &str) -> Result<Quantity, String> {
        let tokens = tokenizer(input.chars());
        let state = parser().parse(tokens)?;
        semanter().eval(&state)
    }
    #[test]
    fn one_equals_one() {
        assert_eq!(Ok(1.into()), calc("1"));
    }

    #[test]
    fn conversion() {
        assert_eq!(
            Ok(Quantity {
                value: 1e-3,
                dimensions: Dimensions {
                    length: 1.,
                    ..Default::default()
                },
                units: Units {
                    length: kilometer,
                    ..Default::default()
                },
            }),
            calc("1 m -> km")
        );
    }

    #[test]
    fn dim_analysis() {
        assert_eq!(
            Ok(Quantity {
                value: 1.,
                dimensions: Dimensions {
                    length: 1.,
                    time: -2.,
                    ..Default::default()
                },
                units: Units {
                    ..Default::default()
                },
            }),
            calc("1m/s^2")
        );
    }
}
