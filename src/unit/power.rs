use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Power {
    Meter,
    KiloMeter,
}
impl Unit for Power {
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
}
