use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq)]
struct ParseError;
#[derive(Debug, Clone, Copy, PartialEq)]
struct MathErr;

type Val = f64;
type Dim = i32;

#[derive(Debug, Clone, Copy, PartialEq)]
struct DimensionError(Dimensions, Dimensions);

type MathResult = Result<(), DimensionError>;
struct Quantity {
    value: Val,
    dimensions: Dimensions,
    units: Units,
}
impl Quantity {
    fn convert_units(&mut self, units: &Units) {
        self.value *= self
            .units
            .length
            .conversion_factor()
            .powi(self.dimensions.length)
            / units
                .length
                .conversion_factor()
                .powi(self.dimensions.length)
            * self
                .units
                .mass
                .conversion_factor()
                .powi(self.dimensions.mass)
            / units.mass.conversion_factor().powi(self.dimensions.mass);
    }

    fn neg(&mut self) {
        self.value = -self.value;
    }

    fn inv(&mut self) {
        self.dimensions.inv();
    }

    fn sub(&mut self, r: &mut Self) -> MathResult {
        r.inv();
        self.add(r)
    }

    fn mul(&mut self, r: &mut Self) {
        r.convert_units(&self.units);
        self.value *= r.value;
        self.dimensions.mul(r.dimensions);
    }
    fn add(&mut self, r: &mut Self) -> MathResult {
        if self.dimensions == r.dimensions {
            r.convert_units(&self.units);
            self.value += r.value;
            Ok(())
        } else {
            Err(DimensionError(self.dimensions, r.dimensions))
        }
    }
}
impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.value,
            if self.dimensions.length != 0 {
                format!(
                    "{}{}",
                    self.units.length.abbrev(),
                    if self.dimensions.length == 1 {
                        "".to_string()
                    } else {
                        format!("^{}", self.dimensions.length)
                    }
                )
            } else {
                "".to_string()
            },
            if self.dimensions.mass != 0 {
                format!(
                    "{}{}",
                    self.units.mass.abbrev(),
                    if self.dimensions.mass == 1 {
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
// base units stored in si

// i.e. m/s^2 has length: 1, time: 2
#[derive(Debug, Clone, Copy, PartialEq)]
struct Dimensions {
    length: Dim,
    mass: Dim,
    //    time: Dim,
    //    current: Dim,
    //    temprature: Dim,
    //    luminous_intensity: Dim,
    //    money: Dim,
}
impl Dimensions {
    pub fn inv(&mut self) {
        self.length = -self.length;
        self.mass = -self.mass;
    }
    pub fn mul(&mut self, r: Self) {
        self.length += r.length;
        self.mass += r.mass;
    }
}
struct Units {
    length: Box<dyn Length>,
    mass: Box<dyn Mass>,
    //  time: Box<dyn Time>,
    //  current: Box<dyn Current>,
    //  temprature: Box<dyn Temperature>,
    //  luminous_intensity: Box<dyn LuminousIntensity>,
    //  money: Box<dyn Money>,
}

trait Unit {
    fn conversion_factor(&self) -> Val {
        1.
    }
    fn abbrev(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;
}
impl std::fmt::Display for dyn Unit {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.symbol())
    }
}
trait Length: Unit {}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Meter;
impl Length for Meter {}
impl Unit for Meter {
    fn abbrev(&self) -> &'static str {
        "m"
    }
    fn name(&self) -> &'static str {
        "meter"
    }
    fn symbol(&self) -> &'static str {
        "m"
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct KiloMeter;
impl Length for KiloMeter {}
impl Unit for KiloMeter {
    fn abbrev(&self) -> &'static str {
        "km"
    }
    fn name(&self) -> &'static str {
        "kilometer"
    }
    fn symbol(&self) -> &'static str {
        "km"
    }
    fn conversion_factor(&self) -> Val {
        1000.
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Inch;
#[derive(Debug, Clone, Copy, PartialEq)]
struct Foot;

// TODO: change dyn impl to enums for each dimension

trait Mass: Unit {}
#[derive(Debug, Clone, Copy, PartialEq)]
struct KiloGram;
impl Unit for KiloGram {
    fn abbrev(&self) -> &'static str {
        "kg"
    }
    fn name(&self) -> &'static str {
        "kilogram"
    }
    fn symbol(&self) -> &'static str {
        "kg"
    }
}
impl Mass for KiloGram {}

fn main() {}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    pub fn test_add() {
        let mut m = Quantity {
            units: Units {
                length: Box::new(Meter),
                mass: Box::new(KiloGram),
            },
            value: 2.,
            dimensions: Dimensions { length: 2, mass: 0 },
        };
        let mut n = Quantity {
            units: Units {
                length: Box::new(KiloMeter),
                mass: Box::new(KiloGram),
            },
            value: -20.,
            dimensions: Dimensions { length: 2, mass: 0 },
        };
        m.add(&mut n).unwrap();
        assert_eq!(m.value, -19999998.);

        let mut m = Quantity {
            units: Units {
                length: Box::new(Meter),
                mass: Box::new(KiloGram),
            },
            value: 2.,
            dimensions: Dimensions {
                length: -2,
                mass: 0,
            },
        };
        let mut n = Quantity {
            units: Units {
                length: Box::new(KiloMeter),
                mass: Box::new(KiloGram),
            },
            value: -20.,
            dimensions: Dimensions {
                length: -2,
                mass: 0,
            },
        };
        m.add(&mut n).unwrap();
        assert_eq!(m.value, 1.99998);
    }

    #[test]
    #[should_panic]
    pub fn test_incompatible_dimensions() {
        let mut m = Quantity {
            units: Units {
                length: Box::new(Meter),
                mass: Box::new(KiloGram),
            },
            value: 2.,
            dimensions: Dimensions { length: 3, mass: 1 },
        };
        let mut n = Quantity {
            units: Units {
                length: Box::new(KiloMeter),
                mass: Box::new(KiloGram),
            },
            value: -20.,
            dimensions: Dimensions { length: 1, mass: 3 },
        };
        m.add(&mut n).unwrap();
    }
    #[test]
    pub fn test_multiply() {
        let mut m = Quantity {
            units: Units {
                length: Box::new(Meter),
                mass: Box::new(KiloGram),
            },
            value: 5.,
            dimensions: Dimensions { length: 1, mass: 0 },
        };
        let mut n = Quantity {
            units: Units {
                length: Box::new(KiloMeter),
                mass: Box::new(KiloGram),
            },
            value: 5.,
            dimensions: Dimensions { length: 1, mass: 0 },
        };
        m.mul(&mut n);
        assert_eq!(m.value, 25000.);
        assert_eq!(m.dimensions.length, 2);
    }
}
