use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Force {
    Newton,
}
impl Unit for Force {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            mass: 1.,
            length: 1.,
            time: -2.,
            ..Default::default()
        }
    }
    fn abbrev(&self) -> &'static str {
        match self {
            _Newton => "N",
        }
    }
    fn name(&self) -> &'static str {
        match self {
            _Newton => "newton",
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            _Newton => "N",
        }
    }
    fn conversion_factor(&self) -> StorageType {
        match self {
            _Newton => 1.,
        }
    }
    fn quantity(&self) -> Quantity {
        Quantity {
            value: self.conversion_factor(),
            dimensions: self.dimensions(),
            units: Units::default(),
        }
    }
}
impl CompoundUnit for Force {}
