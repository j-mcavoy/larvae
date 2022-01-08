pub type StorageType = f64;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DimensionError(pub Dimensions, pub Dimensions);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub length: StorageType,
    pub mass: StorageType,
    pub time: StorageType,
    //    time: Dim,
    //    current: Dim,
    //    temprature: Dim,
    //    luminous_intensity: Dim,
    //    money: Dim,
}
impl Dimensions {
    pub fn pow(&self, x: StorageType) -> Self {
        Self {
            length: self.length + x,
            mass: self.mass + x,
            time: self.time + x,
        }
    }
    pub fn add(&self, r: &Self) -> Self {
        Self {
            length: self.length + r.length,
            mass: self.mass + r.mass,
            time: self.time + r.time,
        }
    }
    pub fn mul(&self, x: StorageType) -> Self {
        Self {
            length: self.length * x,
            mass: self.mass * x,
            time: self.time * x,
        }
    }
    pub fn _mul(&self, r: &Self) -> Self {
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
    pub fn test_mul() {
        let d = Dimensions {
            length: 1.,
            mass: 3.,
            ..Default::default()
        };

        assert_eq!(
            d._mul(&Dimensions {
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
