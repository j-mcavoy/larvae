pub mod force;
pub mod length;
pub mod mass;
pub mod time;


use std::collections::{HashMap};
use std::fmt::Display;


use crate::quantity::Quantity;

use super::dimension::Dimensions;
use super::quantity::StorageType;
use lazy_static::lazy_static;
use length::Length;
use mass::Mass;
use time::Time;

lazy_static! {
    pub static ref UNITS_LOOKUP: HashMap<&'static str, Quantity> = {
        let mut m = HashMap::new();
        m.insert(Length::Meter.abbrev(), Length::Meter.quantity());
        m.insert(Length::Meter.name(), Length::Meter.quantity());
        m.insert(Length::KiloMeter.abbrev(), Length::KiloMeter.quantity());
        m.insert(Length::KiloMeter.name(), Length::KiloMeter.quantity());
        m.insert(Time::Second.name(), Time::Second.quantity());
        m.insert(Time::Second.abbrev(), Time::Second.quantity());
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
            length: Length::Meter,
            mass: Mass::KiloGram,
            time: Time::Second,
        }
    }
}

impl Default for Units {
    fn default() -> Self {
        Self::SI()
    }
}
