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

fn main() {}
