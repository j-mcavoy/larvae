use std::num::ParseFloatError;

use regex::Regex;

use super::quantity::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ParseError;

//impl FromResidual<Option<Infallible>> for ParseError {
//    fn from_residual(residual: Option<Infallible>) -> Self {
//        ParseError {}
//    }
//}
impl From<ParseFloatError> for ParseError {
    fn from(_: ParseFloatError) -> Self {
        ParseError {}
    }
}
impl From<regex::Error> for ParseError {
    fn from(_: regex::Error) -> Self {
        ParseError {}
    }
}

// s: "1.234 m"
// s: "1.234m"
// s: "1m"
// s: "1 m"
// s: "1e3 m"
// s: "1e3m"
fn parse_quantity(s: String) -> Result<Quantity, ParseError> {
    let re = Regex::new(r"\d+\.?|e?\d*").unwrap();
    let mut matches = re.find_iter(&s);
    if let Some(value) = matches.next() {
        if let Ok(_) = value.as_str().parse::<StorageType>() {
            Ok(Quantity::default())
        } else {
            Err(ParseError)
        }
    } else {
        Err(ParseError)
    }
}
