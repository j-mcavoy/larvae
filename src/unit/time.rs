use super::*;

use Time::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Time {
    Second,
}
impl Unit for Time {
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            time: 1.,
            ..Default::default()
        }
    }
    fn abbrev(&self) -> &'static str {
        match self {
            Second => "s",
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Second => "second",
        }
    }
    fn symbol(&self) -> &'static str {
        match self {
            Second => "s",
        }
    }
    fn conversion_factor(&self) -> StorageType {
        match self {
            Second => 1.,
        }
    }
}
