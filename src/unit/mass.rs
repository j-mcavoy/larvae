use super::*;

use Mass::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mass {
    KiloGram,
}
impl Unit for Mass {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            mass: 1.,
            ..Default::default()
        }
    }
    fn abbrev(&self) -> &'static str {
        match self {
            _KiloGram => "kg",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            _KiloGram => "kilogram",
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            _KiloGram => "kg",
        }
    }
    fn conversion_factor(&self) -> StorageType {
        match self {
            _KiloGram => 1.,
        }
    }
    fn quantity(&self) -> Quantity {
        Quantity {
            value: self.conversion_factor(),
            dimensions: self.dimensions(),
            units: Units {
                mass: *self,
                ..Default::default()
            },
        }
    }
}
