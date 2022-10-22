type DimensionFloat = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DimensionError(pub Dimensions, pub Dimensions);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub length: DimensionFloat,
    pub mass: DimensionFloat,
    pub time: DimensionFloat,
    //    current: Dim,
    //    temprature: Dim,
    //    luminous_intensity: Dim,
    //    money: Dim,
}
impl Dimensions {
    pub fn pow(&self, x: DimensionFloat) -> Self {
        Self {
            length: self.length * x,
            mass: self.mass * x,
            time: self.time * x,
        }
    }
    pub fn mul(&self, r: &Self) -> Self {
        Self {
            length: self.length + r.length,
            mass: self.mass + r.mass,
            time: self.time + r.time,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_pow() {
        let d = Dimensions {
            length: 1.,
            mass: 2.,
            ..Default::default()
        };

        assert_eq!(
            d.pow(2.),
            Dimensions {
                length: 2.,
                mass: 4.,
                ..Default::default()
            }
        )
    }

    #[test]
    pub fn test_mul() {
        let d = Dimensions {
            length: 1.,
            mass: 3.,
            ..Default::default()
        };

        assert_eq!(
            d.mul(&Dimensions {
                length: -1.,
                mass: 2.,
                ..Default::default()
            }),
            Dimensions {
                length: 0.,
                mass: 5.,
                ..Default::default()
            }
        )
    }
}
