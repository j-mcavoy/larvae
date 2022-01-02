use uom::num_traits::Num;
use uom::si::quantities::*;
use uom::si::{self, Quantity};
use uom::typenum::int::Z0;
use uom::typenum::marker_traits::Integer;

type Val = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ParseError;
#[derive(Debug, Clone, Copy, PartialEq)]
struct MathErr;

macro_rules! parse_quantity_conditional {
    ($var:ident, $($quantity:ident)+) => {
        $(if let Ok(q) = $var.parse::<$quantity<Val>>() {
            Ok(q)
        }
        else
        )+
        {
        Err(ParseError)
        }
    }
}
fn parse_quantity<
    L: Integer,
    M: Integer,
    T: Integer,
    I: Integer,
    Th: Integer,
    N: Integer,
    J: Integer,
    Kind: dyn uom::Kind,
>(
    x: &str,
) -> Result<
    Quantity<
        si::Dimension<L = L, M = M, T = T, I = I, Th = Th, N = N, J = J, Kind = Kind>,
        dyn si::Units<
            Val,
            length = si::length::meter,
            mass = si::mass::kilogram,
            time = si::time::second,
            electric_current = si::electric_current::ampere,
            thermodynamic_temperature = si::thermodynamic_temperature::kelvin,
            amount_of_substance = si::amount_of_substance::mole,
            luminous_intensity = si::luminous_intensity::candela,
        >,
        Val,
    >,
    ParseError,
> {
    parse_quantity_conditional!(x,
            Acceleration
            AmountOfSubstance
            Angle
            AngularAcceleration
            AngularJerk
            AngularVelocity
            Area
            AvailableEnergy
            Capacitance
            CatalyticActivity
            CatalyticActivityConcentration
            Curvature
            ElectricCharge
            ElectricCurrent
            ElectricPotential
            ElectricalConductance
            ElectricalResistance
            Energy
            Force
            Frequency
            HeatCapacity
            HeatFluxDensity
            HeatTransfer
            Inductance
            Information
            InformationRate
            Jerk
            Length
            Luminance
            LuminousIntensity
            MagneticFlux
            MagneticFluxDensity
            Mass
            MassConcentration
            MassDensity
            MassRate
            MolarConcentration
            MolarEnergy
            MolarMass
            Momentum
            Power
            Pressure
            RadiantExposure
            Ratio
            SolidAngle
            SpecificHeatCapacity
            TemperatureInterval
            ThermalConductivity
            ThermodynamicTemperature
            Time
            Torque
            Velocity
            Volume
            VolumeRate)
}

fn main() {
    println!("{:?}", parse_quantity("77 m"));
}

//fn add(lq: Q, rq: Q) -> Result<Q, MathErr> {
//    use Q::*;
//
//    match (lq, rq) {
//        (Mass(r), Mass(l)) => Ok(Mass(r + l)),
//        (_, _) => Err(MathErr),
//    }
//}

//fn multiply(lq: Q, rq: Q) -> Result<Q, MathErr> {
//    use Q::*;
//
//    match (lq, rq) {
//        (Mass(r), Mass(l)) => {
//
//        let x = }
//        (_, _) => Err(MathErr),
//    }
//}

/*
mod test {
    use super::*;

    use uom::si::acceleration::meter_per_second_squared;
    use uom::si::acceleration::Acceleration as acceleration;
    use uom::si::mass::kilogram;
    use uom::si::mass::Mass as mass;

    #[test]
    pub fn test_parse_quantity() {
        assert_eq!(
            parse_quantity("123 kg").unwrap(),
            Q::Mass(mass::new::<kilogram>(123.))
        );
    }
    #[test]
    pub fn test_add() {
        let result = add(
            Q::Mass(mass::new::<kilogram>(100.)),
            Q::Mass(mass::new::<kilogram>(23.)),
        )
        .unwrap();
        assert_eq!(result, Q::Mass(mass::new::<kilogram>(123.)));
        let result = add(
            Q::Acceleration(acceleration::new::<meter_per_second_squared>(100.)),
            Q::Mass(mass::new::<kilogram>(23.)),
        );
        assert_eq!(result, Err(MathErr));
    }
}
*/
