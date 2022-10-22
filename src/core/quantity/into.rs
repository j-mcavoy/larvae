use super::{Quantity, QuantityFloat};

impl TryInto<QuantityFloat> for Quantity {
    type Error = String;
    fn try_into(self) -> Result<QuantityFloat, Self::Error> {
        if self.dimensions.length == 0. && self.dimensions.mass == 0. && self.dimensions.time == 0.
        {
            Ok(self.value)
        } else {
            Err("Not a number".into())
        }
    }
}

impl<T: Into<QuantityFloat>> From<T> for Quantity {
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_f64() {
        let expected = Quantity {
            value: 3.14,
            ..Default::default()
        };
        assert_eq!(expected, 3.14.into());
        assert_eq!(expected, Quantity::from(3.14));
    }
}
