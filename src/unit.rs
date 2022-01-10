//pub mod force;
//pub mod length;
//pub mod mass;
//pub mod time;

use std::collections::HashMap;
use std::fmt::Display;

use crate::quantity::Quantity;

use super::dimension::Dimensions;
use super::quantity::StorageType;
use lazy_static::lazy_static;
use length::Length;
use mass::Mass;
use time::Time;

use larvae_macros::dimension;

dimension! {
    "length" 1. 0. 0. :
        "meter" "m" "m" 1.
        "kilometer" "km" "km" 1e3
}
dimension! {
    "mass" 1. 0. 0. :
        "gram" "g" "g" 1.
        "kilogram" "kg" "kg" 1e3
}
dimension! {
    "time" 0. 0. 1. :
        "second" "s" "s" 1.
        "minute" "m" "min" 60.
        "hour" "h" "hr" 3600.
}

// TODO: write a macro for generating units & dimensions
lazy_static! {
    pub static ref UNITS_LOOKUP: HashMap<&'static str, Quantity> = {
        let mut m = HashMap::new();
        m.insert(Length::meter.abbrev(), Length::meter.quantity());
        m.insert(Length::meter.name(), Length::meter.quantity());
        m.insert(Length::kilometer.abbrev(), Length::kilometer.quantity());
        m.insert(Length::kilometer.name(), Length::kilometer.quantity());
        m.insert(Time::second.name(), Time::second.quantity());
        m.insert(Time::second.abbrev(), Time::second.quantity());
        m.insert(Mass::kilogram.name(), Mass::kilogram.quantity());
        m.insert(Mass::kilogram.abbrev(), Mass::kilogram.quantity());
        m
    };
}

pub trait Unit {
    fn conversion_factor(&self) -> StorageType;
    fn abbrev(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;
    fn dimensions(&self) -> Dimensions;
    fn quantity(&self) -> Quantity;
}
impl std::fmt::Display for dyn Unit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.symbol())
    }
}
pub trait CompoundUnit: Unit {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Units {
    pub length: Length,
    pub mass: Mass,
    pub time: Time,
}
impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Units {
    pub const fn SI() -> Self {
        Self {
            length: Length::meter,
            mass: Mass::kilogram,
            time: Time::second,
        }
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::SI()
    }
}
