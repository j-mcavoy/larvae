use super::dimension::*;
use super::unit::*;
use float_pretty_print::PrettyPrintFloat;
use std::fmt::Display;

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

macro_rules! sym_dim {
    ($f:ident, $map: ident, $sel:ident, $($dim:ident),+) => {
        let mut $map: Vec<(&str, f64)> = vec![
            $(($sel.units.$dim.symbol(),  $sel.dimensions.$dim)),+
        ];
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        sym_dim!(f, sym_dim, self, length, mass, time);
        sym_dim.sort_by(|l, r| r.1.partial_cmp(&l.1).unwrap());

        let mut pos_dims = vec![];
        let mut neg_dims = vec![];
        for (sym, dim) in sym_dim {
            if dim == 0. {
                continue;
            }
            if dim.is_sign_positive() {
                pos_dims.push((sym, dim))
            } else if dim.is_sign_negative() {
                neg_dims.insert(0, (sym, -dim))
            }
        }
        let mut pos_units = vec![];
        for (sym, dim) in pos_dims.clone() {
            pos_units.push(if dim == 1.0 {
                sym.to_string()
            } else if dim == (dim as i64) as f64 {
                format!("{}{}", sym, num_to_superscript(dim as i64))
            } else {
                format!("{}^({})", sym, dim)
            });
        }
        let mut neg_units = vec![];
        for (sym, dim) in neg_dims.clone() {
            neg_units.push(if dim == 1.0 {
                sym.to_string()
            } else if dim == (dim as i64) as f64 {
                format!("{}{}", sym, num_to_superscript(dim as i64))
            } else {
                format!("{}^({})", sym, dim)
            });
        }
        let units: String = match (pos_dims.len(), neg_dims.len()) {
            (0, 0) => String::new(),
            (_, 0) => pos_units.join("·"),
            (0, _) => format!("1/{}", neg_units.join("·")),
            (_, _) => format!("{}/{}", pos_units.join("·"), neg_units.join("·")),
        };

        let value = self.value;
        if value == (value as i64) as f64 {
            write!(f, "{} {}", value, units)
        } else {
            write!(f, "{:.10} {}", PrettyPrintFloat(value), units)
        }
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
        num /= 10;
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
    pub fn test_convert_units() {
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
    pub fn test_add() {
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
        let out = m.add(&n).unwrap();
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
    #[should_panic]
    pub fn test_incompatible_dimensions() {
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
        let _ = m.add(&n).unwrap();
    }
    #[test]
    pub fn test_multiply() {
        let m = Quantity {
            value: 5.,
            dimensions: Dimensions {
                length: 2.,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut n = Quantity {
            value: 5.,
            dimensions: Dimensions {
                length: 1.,
                ..Default::default()
            },
            ..Default::default()
        };
        let out = m.mul(&mut n);
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

    #[test]
    pub fn test_quantity_display() {
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
