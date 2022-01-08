use super::*;
use Length::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Meter,
    KiloMeter,
}
impl Unit for Length {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            length: 1.,
            ..Default::default()
        }
    }
    fn abbrev(&self) -> &'static str {
        match self {
            Meter => "m",
            KiloMeter => "km",
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Meter => "meter",
            KiloMeter => "kilometer",
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            Meter => "m",
            KiloMeter => "km",
        }
    }
    fn conversion_factor(&self) -> StorageType {
        match self {
            Meter => 1.,
            KiloMeter => 1e3,
        }
    }
    fn quantity(&self) -> Quantity {
        Quantity {
            value: self.conversion_factor(),
            dimensions: self.dimensions(),
            units: Units {
                length: *self,
                ..Default::default()
            },
        }
    }
}
