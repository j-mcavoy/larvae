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
            Newton => "N",
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Newton => "newton",
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            Newton => "N",
        }
    }
    fn conversion_factor(&self) -> StorageType {
        match self {
            Newton => 1.,
        }
    }
}
impl CompoundUnit for Force {}
