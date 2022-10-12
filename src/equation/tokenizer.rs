use lexers::Scanner;
use std::{fmt::Display, str::Chars};

trait LarvaeScanner {
    fn larvae_scan_unit(&mut self) -> Option<String>;
    fn scan_arrow(&mut self) -> Option<String>;
    fn scan_unknown(&mut self) -> Option<String>;
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
            .or_else(|| self.0.larvae_scan_unit())
            .or_else(|| self.0.scan_unknown())
    }
}
impl<I: Iterator<Item = char>> LarvaeScanner for Scanner<I> {
    fn larvae_scan_unit(&mut self) -> Option<String> {
        for unit in crate::core::unit::UNITS_LOOKUP.keys() {
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

    fn scan_unknown(&mut self) -> Option<String> {
        self.next().map(|c| c.to_string())
    }
}

pub fn tokenizer<I: Iterator<Item = char>>(input: I) -> Tokenizer<I> {
    Tokenizer(lexers::Scanner::new(input))
}

#[cfg(test)]
mod tests {

    use super::super::*;
    use super::*;
    use crate::core::quantity::*;
    use crate::core::unit::length::Length::*;
    use crate::core::{dimension::Dimensions, unit::Units};

    #[test]
    pub fn test_parse_dimunits() {
        token_test(
            "1kg^-2kg kg*e/e*log(10)*pi/pi*sqrt(1)!%2*1.123kilometer^2/s+100s^-1m*m+10km^2/s-0m^2/s-> m^3/m/s",
            "1 kg ^ -2 kg kg * e / e * log ( 10 ) * pi / pi * sqrt ( 1 ) ! % 2 * 1.123 kilometer ^ 2 / s +100 s ^ -1 m * m +10 km ^ 2 / s -0 m ^ 2 / s -> m ^ 3 / m / s");
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
    pub fn test_whitespace() {
        token_test("1    km  +   6.3m    ", "1 km + 6.3 m");
        token_test("1km/s^5.4 + 6.3m/s^5.4", "1 km / s ^ 5.4 + 6.3 m / s ^ 5.4");
    }
    #[test]
    pub fn test_arrow() {
        token_test("1m/s -> km/min", "1 m / s -> km / min");
        token_test("1m/s->km/min", "1 m / s -> km / min");
    }

    #[test]
    pub fn test_group() {
        token_test("5m/s+(1km/3s)", "5 m / s + ( 1 km / 3 s )");
    }
    #[test]
    pub fn test_ops() {
        token_test("1 + 2", "1 + 2");
        token_test("1 - 2", "1 - 2");
        token_test("1*2", "1 * 2");
        token_test("1/2", "1 / 2");
        token_test("1%2", "1 % 2");
        token_test("1!", "1 !");
        token_test("log(5)", "log ( 5 )");
        token_test("ln(5)", "ln ( 5 )");
        token_test("e^4", "e ^ 4");
        token_test("e^-4", "e ^ -4");
        token_test("pi - 4", "pi - 4");
        token_test("(2)(2)", "( 2 ) ( 2 )");
    }
    #[test]
    pub fn test_eval_pos_neg_num() {
        token_test("-1", "-1");
        token_test("+1", "+1");
        token_test("+1m-2m", "+1 m -2 m");
        token_test("+1m+2m", "+1 m +2 m");
        token_test("+1m+-2m", "+1 m + -2 m");
        token_test("+1m++2m", "+1 m + +2 m");
        token_test("+1m--2m", "+1 m - -2 m");
        token_test("1m+2m", "1 m +2 m");
        token_test("1m-2m", "1 m -2 m");
    }

    #[test]
    pub fn test_all_units() {
        for unit in crate::core::unit::UNITS_LOOKUP.keys() {
            token_test(
                format!("123{0}+ 3{0}/{0}*{0}", unit).as_str(),
                format!("123 {0} + 3 {0} / {0} * {0}", unit).as_str(),
            );
        }
    }
    #[test]
    pub fn test_unknown_tokens() {
        token_test("1m&^9", "1 m & ^ 9");
        token_test("1m&$#^9", "1 m & $ # ^ 9");
    }
    fn token_test(input: &str, expected: &str) {
        let tokens: Vec<String> = tokenizer(input.chars()).collect();
        let expected_out: Vec<String> =
            expected.split_whitespace().map(|s| s.to_string()).collect();
        assert_eq!(tokens, expected_out);
    }
}
