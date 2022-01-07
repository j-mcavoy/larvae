use std::fmt::Display;
use std::ops::Add;

use super::dimension::*;
use super::unit::*;

pub type StorageType = f64;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quantity {
    value: StorageType,
    dimensions: Dimensions,
    units: Units,
}
impl Add<Quantity> for Quantity {
    type Output = Quantity;
    fn add(self, rhs: Quantity) -> Self::Output {
        self.add(rhs)
    }
}
impl Quantity {
    pub fn new(value: StorageType, dimensions: Dimensions, units: Units) -> Self {
        Quantity {
            value,
            dimensions,
            units,
        }
    }
    pub fn from_value(value: StorageType) -> Self {
        Quantity {
            value,
            ..Default::default()
        }
    }
    pub fn from_units(units: Units) -> Self {
        Quantity {
            units,
            ..Default::default()
        }
    }
    pub fn convert_units(&self, units: &Units) -> Self {
        Self {
            units: *units,
            value: self.value * Self::conversion_factor(&self.units, &self.dimensions)
                / Self::conversion_factor(units, &self.dimensions),
            dimensions: self.dimensions,
        }
    }

    pub fn neg(&self) -> Self {
        Self {
            value: -self.value,
            ..*self
        }
    }

    fn conversion_factor(units: &Units, dimensions: &Dimensions) -> StorageType {
        let l = units.length.conversion_factor().powf(dimensions.length);
        let m = units.mass.conversion_factor().powf(dimensions.mass);
        let t = units.time.conversion_factor().powf(dimensions.time);

        l * m * t
    }

    pub fn pow(&self, x: StorageType) -> Self {
        Self {
            units: self.units,
            dimensions: self.dimensions.pow(x),
            value: self.value.powf(x),
        }
    }
    fn inv(&self) -> Self {
        self.clone().pow(-1.)
    }

    pub fn sub(&self, r: &Self) -> Result<Self, DimensionError> {
        self.add(&r.neg())
    }

    pub fn mul(&self, r: &Self) -> Self {
        let r_converted = r.convert_units(&self.units.clone());
        Self {
            value: self.value * r_converted.value,
            dimensions: self.dimensions.mul(&r.dimensions),
            ..*self
        }
    }
    pub fn add(&self, r: &Self) -> Result<Self, DimensionError> {
        if self.dimensions != r.dimensions {
            Err(DimensionError(self.dimensions, r.dimensions))
        } else {
            let r_converted = r.clone().convert_units(&self.units.clone());
            Ok(Self {
                value: self.value + r_converted.value,
                dimensions: self.dimensions,
                units: self.units,
            })
        }
    }
}
impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.value,
            if self.dimensions.length != 0. {
                format!(
                    "{}{}",
                    self.units.length.abbrev(),
                    if self.dimensions.length == 1. {
                        "".to_string()
                    } else {
                        format!("^{}", self.dimensions.length)
                    }
                )
            } else {
                "".to_string()
            },
            if self.dimensions.mass != 0. {
                format!(
                    "{}{}",
                    self.units.mass.abbrev(),
                    if self.dimensions.mass == 1.0 {
                        "".to_string()
                    } else {
                        format!("^{}", self.dimensions.mass)
                    }
                )
            } else {
                "".to_string()
            }
        )
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::unit::length::Length::*;
    use crate::unit::mass::Mass::*;
    use crate::unit::Units;

    #[test]
    pub fn test_conversion_factor() {
        let m = Quantity {
            units: Units::SI(),
            value: 2000.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
        };

        assert_eq!(1., Quantity::conversion_factor(&m.units, &m.dimensions));
        let m = Quantity {
            units: Units {
                length: KiloMeter,
                ..Units::SI()
            },
            value: 2000.,
            dimensions: Dimensions {
                length: 3.,
                ..Default::default()
            },
        };

        assert_eq!(1e9, Quantity::conversion_factor(&m.units, &m.dimensions));
    }
    #[test]
    pub fn test_convert_units() {
        let m = Quantity {
            units: Units::SI(),
            value: 2000.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
        };
        let converted = m.convert_units(&Units::SI());

        assert_eq!(m, converted);

        let converted = m.convert_units(&Units {
            length: KiloMeter,
            ..Units::SI()
        });

        assert_eq!(
            Quantity {
                units: Units {
                    length: KiloMeter,
                    ..Units::SI()
                },
                dimensions: Dimensions {
                    length: 1.,
                    ..Default::default()
                },
                value: 2.
            },
            converted
        )
    }
    #[test]
    pub fn test_add() {
        let m = Quantity {
            units: Units::SI(),
            value: 2.,
            dimensions: Dimensions {
                length: 2.,
                ..Default::default()
            },
        };
        let n = Quantity {
            units: Units {
                length: KiloMeter,
                ..Units::SI()
            },
            value: -20.,
            dimensions: Dimensions {
                length: 2.,
                ..Default::default()
            },
        };
        let out = m.add(&n).unwrap();
        assert_eq!(
            Quantity {
                value: -19999998.,
                units: Units::SI(),
                dimensions: Dimensions {
                    length: 2.,
                    ..Default::default()
                },
            },
            out
        );
    }

    #[test]
    #[should_panic]
    pub fn test_incompatible_dimensions() {
        let m = Quantity {
            units: Units::SI(),
            value: 2.,
            dimensions: Dimensions {
                length: 3.,
                mass: 1.,
                ..Default::default()
            },
        };
        let mut n = Quantity {
            units: Units {
                length: KiloMeter,
                ..Units::SI()
            },
            value: -20.,
            dimensions: Dimensions {
                length: 1.,
                mass: 3.,
                ..Default::default()
            },
        };
        m.add(&mut n).unwrap();
    }
    #[test]
    pub fn test_multiply() {
        let m = Quantity {
            units: Units::SI(),
            value: 5.,
            dimensions: Dimensions {
                length: 2.,
                ..Default::default()
            },
        };
        let mut n = Quantity {
            units: Units::SI(),
            value: 5.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
        };
        let out = m.mul(&mut n);
        assert_eq!(
            Quantity {
                value: 25.,
                dimensions: Dimensions {
                    length: 3.,
                    ..Default::default()
                },
                units: Units::SI()
            },
            out
        );
    }
}
