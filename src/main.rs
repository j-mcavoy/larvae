use std::cell::RefCell;

mod dimension;
mod equation;
mod quantity;
mod unit;

use equation::parser::*;

use crate::{
    dimension::Dimensions,
    quantity::{Quantity, StorageType},
    unit::{Unit, Units},
};

const UNITS: &'static [&str] = &["m", "s", "kg"];

fn main() {
    let input = "1m";
    let parser = earlgrey::EarleyParser::new(build_grammar());
    let evaler = semanter();
    match parser.parse(&mut tokenizer(input.chars())) {
        Err(e) => println!("Parse err: {:?}", e),
        Ok(state) => {
            let val = evaler.eval(&state);
            println!("{:?}", val);
        }
    }
}
