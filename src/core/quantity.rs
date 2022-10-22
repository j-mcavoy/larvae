use crate::core::dimension::*;
use crate::core::unit::*;

pub type QuantityFloat = f64;

mod display;
pub mod into;
mod ops;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quantity {
    pub value: QuantityFloat,
    pub dimensions: Dimensions,
    pub units: Units,
}
impl Quantity {
    pub fn new(value: QuantityFloat, dimensions: Dimensions, units: Units) -> Self {
        Quantity {
            value,
            dimensions,
            units,
        }
    }
    pub fn set_units(&self, units: &Units) -> Self {
        Self {
            units: *units,
            value: self.value * Self::conversion_factor(&self.units, &self.dimensions)
                / Self::conversion_factor(units, &self.dimensions),
            dimensions: self.dimensions,
        }
    }

    fn conversion_factor(units: &Units, dimensions: &Dimensions) -> QuantityFloat {
        let l = units.length.conversion_factor().powf(dimensions.length);
        let m = units.mass.conversion_factor().powf(dimensions.mass);
        let t = units.time.conversion_factor().powf(dimensions.time);

        l * m * t
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::core::unit::length::Length::*;
    use crate::core::unit::Units;

    #[test]
    pub fn test_conversion_factor() {
        let m = Quantity {
            value: 2000.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(1., Quantity::conversion_factor(&m.units, &m.dimensions));
        let m = Quantity {
            units: Units {
                length: kilometer,
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
    pub fn convert_units() {
        let m = Quantity {
            value: 2000.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
            ..Default::default()
        };
        let converted = m.set_units(&Units::SI());

        assert_eq!(m, converted);

        let converted = m.set_units(&Units {
            length: kilometer,
            ..Units::SI()
        });

        assert_eq!(
            Quantity {
                units: Units {
                    length: kilometer,
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
    #[should_panic]
    pub fn incompatible_dimensions() {
        let m = Quantity {
            value: 2.,
            dimensions: Dimensions {
                length: 3.,
                mass: 1.,
                ..Default::default()
            },
            ..Default::default()
        };
        let n = Quantity {
            units: Units {
                length: kilometer,
                ..Units::SI()
            },
            value: -20.,
            dimensions: Dimensions {
                length: 1.,
                mass: 3.,
                ..Default::default()
            },
        };
        let _ = m + n;
    }

    #[test]
    pub fn quantity_display() {
        let q = Quantity {
            value: 25.,
            dimensions: Dimensions {
                length: 1.,
                mass: 2.,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(q.to_string(), "25 kg²·m");
        let q = Quantity {
            value: 25.,
            dimensions: Dimensions {
                length: 1.,
                mass: 2.,
                time: -3.,
                ..Default::default()
            },
            ..Default::default()
        };

        assert_eq!(q.to_string(), "25 kg²·m/s³");

        let q = Quantity {
            value: 25.,
            dimensions: Dimensions {
                length: -1.,
                mass: -2.,
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(q.to_string(), "25 1/kg²·m");
    }
}
