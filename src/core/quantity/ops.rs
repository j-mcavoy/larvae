use crate::core::DimensionError;
use std::ops::{Add, Div, Mul, Sub};

use super::{Quantity, QuantityFloat};

impl Quantity {
    pub fn neg(&self) -> Self {
        Self {
            value: -self.value,
            ..*self
        }
    }

    pub fn add(&self, r: &Self) -> Result<Self, DimensionError> {
        if self.dimensions != r.dimensions {
            Err(DimensionError(self.dimensions, r.dimensions))
        } else {
            let r_converted = r.clone().set_units(&self.units.clone());
            Ok(Self {
                value: self.value + r_converted.value,
                dimensions: self.dimensions,
                units: self.units,
            })
        }
    }
    pub fn sub(&self, r: &Self) -> Result<Self, DimensionError> {
        self.add(&r.neg())
    }

    pub fn mul(&self, r: &Self) -> Self {
        let r_converted = r.set_units(&self.units.clone());
        Self {
            value: self.value * r_converted.value,
            dimensions: self.dimensions.mul(&r.dimensions),
            ..*self
        }
    }

    pub fn div(&self, r: &Self) -> Self {
        self.mul(&r.inv())
    }

    pub fn pow(&self, x: QuantityFloat) -> Self {
        Self {
            units: self.units,
            dimensions: self.dimensions.pow(x),
            value: self.value.powf(x),
        }
    }
    pub fn inv(&self) -> Self {
        self.clone().pow(-1.)
    }
}

impl Add<Quantity> for Quantity {
    type Output = Quantity;

    fn add(self, rhs: Quantity) -> Self::Output {
        Quantity::add(&self, &rhs).unwrap()
    }
}

impl Sub<Quantity> for Quantity {
    type Output = Quantity;

    fn sub(self, rhs: Quantity) -> Self::Output {
        Quantity::sub(&self, &rhs).unwrap()
    }
}

impl Mul<Quantity> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: Quantity) -> Self::Output {
        Quantity::mul(&self, &rhs)
    }
}

impl Div<Quantity> for Quantity {
    type Output = Quantity;

    fn div(self, rhs: Quantity) -> Self::Output {
        Quantity::div(&self, &rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::dimension::*;
    use crate::core::unit::length::Length::*;
    use crate::core::unit::Units;

    #[test]
    pub fn add() {
        let m = Quantity {
            value: 2.,
            dimensions: Dimensions {
                length: 2.,
                ..Default::default()
            },
            ..Default::default()
        };
        let n = Quantity {
            value: -20.,
            dimensions: Dimensions {
                length: 2.,
                ..Default::default()
            },
            units: Units {
                length: kilometer,
                ..Units::SI()
            },
        };
        let out = m + n;
        assert_eq!(
            Quantity {
                value: -19999998.,
                dimensions: Dimensions {
                    length: 2.,
                    ..Default::default()
                },
                ..Default::default()
            },
            out
        );
    }

    #[test]
    pub fn div() {
        let m = Quantity {
            value: 30.,
            dimensions: Dimensions {
                length: 2.,
                ..Default::default()
            },
            ..Default::default()
        };
        let n = Quantity {
            value: 5.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
            ..Default::default()
        };
        let out = m.div(n);
        let out2 = m / n;
        assert_eq!(out, out2);
        assert_eq!(
            Quantity {
                value: 6.,
                dimensions: Dimensions {
                    length: 1.,
                    ..Default::default()
                },
                ..Default::default()
            },
            out
        );
    }

    #[test]
    pub fn mul() {
        let m = Quantity {
            value: 5.,
            dimensions: Dimensions {
                length: 2.,
                ..Default::default()
            },
            ..Default::default()
        };
        let n = Quantity {
            value: 5.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
            ..Default::default()
        };
        let out = m.mul(n);
        let out2 = m * n;
        assert_eq!(out, out2);
        assert_eq!(
            Quantity {
                value: 25.,
                dimensions: Dimensions {
                    length: 3.,
                    ..Default::default()
                },
                ..Default::default()
            },
            out
        );
    }
}
