use super::Quantity;
use crate::core::unit::*;
use float_pretty_print::PrettyPrintFloat;
use std::fmt::Display;

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
mod tests {
    use super::*;
    use crate::core::dimension::*;

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
