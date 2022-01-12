use std::fmt::Display;

use super::dimension::*;
use super::unit::*;

pub type StorageType = f64;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Quantity {
    pub value: StorageType,
    pub dimensions: Dimensions,
    pub units: Units,
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
    pub fn set_units(&self, units: &Units) -> Self {
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
    pub fn inv(&self) -> Self {
        self.clone().pow(-1.)
    }

    pub fn sub(&self, r: &Self) -> Result<Self, DimensionError> {
        self.add(&r.neg())
    }

    pub fn mul(&self, r: &Self) -> Self {
        let r_converted = r.set_units(&self.units.clone());
        Self {
            value: self.value * r_converted.value,
            dimensions: self.dimensions._mul(&r.dimensions),
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
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

macro_rules! dim_display {
    ($f:ident, $sel:ident, $($dim:ident),+) => {
        {
            let out = write!($f,
               concat! (
                   "{} ",
                    $(replace_expr!(
                            ($dim)
                            "{} "
                        )
                    ),*
               ),
                $sel.value,
                $(if $sel.dimensions.$dim != 0. {
                        if $sel.dimensions.$dim == 1.0 {
                            format!("{}", $sel.units.$dim.symbol())
                        } else if $sel.dimensions.$dim == ($sel.dimensions.$dim as i64) as f64 {
                            format!(
                                "{}{}",
                                $sel.units.$dim.symbol(),
                                num_to_superscript($sel.dimensions.$dim as i64)
                            )
                        } else {
                            format!("{}^({})", $sel.units.$dim.symbol(), $sel.dimensions.$dim)
                        }
                    } else {
                        "".to_string()
                    }
                ),+
            );
            out
        }
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dim_display! {f, self, length, mass, time}
    }
}

fn num_to_superscript(n: i64) -> String {
    let mut out = "".to_string();
    let mut num = n;
    if n.is_negative() {
        num *= -1;
    }
    loop {
        out += if num % 10 == 0 {
            "⁰"
        } else if num % 10 == 1 {
            "¹"
        } else if num % 10 == 2 {
            "²"
        } else if num % 10 == 3 {
            "³"
        } else if num % 10 == 4 {
            "⁴"
        } else if num % 10 == 5 {
            "⁵"
        } else if num % 10 == 6 {
            "⁶"
        } else if num % 10 == 7 {
            "⁷"
        } else if num % 10 == 8 {
            "⁸"
        } else if num % 10 == 9 {
            "⁹"
        } else {
            ""
        };
        num = num / 10;
        if num == 0 {
            if n.is_negative() {
                out += "⁻";
            }
            break;
        }
    }
    out.chars().rev().collect::<String>()
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::unit::length::Length::*;
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
    pub fn test_convert_units() {
        let m = Quantity {
            units: Units::SI(),
            value: 2000.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
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
                length: kilometer,
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
        let _ = m.add(&n).unwrap();
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
