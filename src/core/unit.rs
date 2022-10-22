use super::dimension::Dimensions;
use super::quantity::Quantity;
use super::quantity::QuantityFloat;
use larvae_macros::dimensions;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::Display;

dimensions! {
    "length" 1. 0. 0. :
    "meter" "m" "m" 1.!,
    "mass" 0. 1. 0. :
    "gram" "g" "g" 1.!,
    "time" 0. 0. 1. :
    "second" "s" "s" 1.!
    "minute" "min" "min" 60.
    "hour" "h" "hr" 3600.,
    "force" 1. 1. -2. :
    "newton" "N" "N" 1.!,
}

pub trait Unit {
    fn conversion_factor(&self) -> QuantityFloat;
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
    #[allow(non_snake_case)]
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
