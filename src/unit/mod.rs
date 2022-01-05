pub mod force;
pub mod length;
pub mod mass;
pub mod time;

use super::dimension::Dimensions;
use super::quantity::StorageType;
use length::Length;
use mass::Mass;
use time::Time;

pub trait Unit {
    fn conversion_factor(&self) -> StorageType;
    fn abbrev(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;
    fn dimensions(&self) -> Dimensions;
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
    //  time: Box<dyn Time>,
    //  current: Box<dyn Current>,
    //  temprature: Box<dyn Temperature>,
    //  luminous_intensity: Box<dyn LuminousIntensity>,
    //  money: Box<dyn Money>,
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
