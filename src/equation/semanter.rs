use crate::core::{
    quantity::{Quantity, QuantityFloat},
    unit::UNITS_LOOKUP,
};
use log::debug;
use spfunc::gamma::gamma;

fn symbol_match(symbol: &str, token: &str) -> Quantity {
    let out = match symbol {
        "[n]" => Quantity::from(token.parse::<QuantityFloat>().unwrap()),
        "e" => Quantity::from(std::f64::consts::E),
        "pi" => Quantity::from(std::f64::consts::PI),
        "unit" => *UNITS_LOOKUP.get(token).expect("invalid unit"),
        _ => 0.into(),
    };
    debug!("{:?}", out);
    out
}

macro_rules! debug_action {
    ($ev:ident, $n:ident, $($action:literal, $exp:expr),+ ) => {
       $( $ev.action($action, |$n| {
            debug!($action);
            $exp
        }); )+
    };
}

pub fn semanter<'a>() -> earlgrey::EarleyForest<'a, Quantity> {
    let mut ev = earlgrey::EarleyForest::new(symbol_match);
    debug_action! {
        ev, n,
        "expr -> term",          n[0],
        "expr -> expr + term",   n[0] + n[2],
        "expr -> expr - term",   n[0] - n[2],
        "term -> factor",        n[0],
        "term -> term * factor", n[0] * n[2],
        "term -> term / factor", n[0] / n[2],
        "term -> term % factor",
                                (
                                    TryInto::<QuantityFloat>::try_into(n[0]).unwrap()
                                    %
                                    TryInto::<QuantityFloat>::try_into(n[2]).unwrap()
                                ).into(),

        "factor -> power",       n[0],
        "factor -> - factor",    n[1].neg(),
        "power -> ufact",        n[0],
        "power -> ufact ^ factor",
                                 n[0].pow(n[2].try_into().unwrap()),

        "ufact -> group",        n[0],
        "ufact -> ufact !",      gamma::<f64>(TryInto::<QuantityFloat>::try_into(n[0]).unwrap() + 1.).into(),
        "quantity -> [n]",       n[0],
        "quantity -> [n] units",
                                Quantity::new(n[0].value, n[1].dimensions, n[1].units),
        "units -> unit",        n[0],
        "units -> units units",  n[0] * n[1],
        "units -> units * units",
                                n[0] * n[2],
        "units -> units / units",
                                n[0] / n[2],

        "units -> unit ^ [n]",
                                n[0].pow(n[2].try_into().expect("Quantity is not scalar")),
        "units -> units ^ unit",
                                n[0].pow(n[2].try_into().expect("Quantity is not scalar")),
        "group -> quantity",    n[0],
        "group -> ( expr )",    n[1],
        "group -> sqrt group",   TryInto::<QuantityFloat>::try_into(n[1]).expect("Quantity is not scalar").sqrt().into(),
        "group -> log group",   TryInto::<QuantityFloat>::try_into(n[1]).expect("Quantity is not scalar").log10().into(),
        "group -> ln group",    TryInto::<QuantityFloat>::try_into(n[1]).expect("Quantity is not scalar").ln().into(),
        "equation -> expr",     n[0],
        "equation -> expr [->] units",
                                n[0].set_units(&n[2].units)
    }
    ev
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::dimension::Dimensions, core::quantity::Quantity, equation::parser::parser};

    #[inline]
    fn eval(input: &str) -> Quantity {
        debug!("input: {}", input);
        semanter()
            .eval(&parser().parse(input.split_whitespace()).unwrap())
            .unwrap()
    }

    #[test]
    fn basic_arithmetic() {
        assert_eq!(eval("1 + 2 + 2"), 5.into());
        assert_eq!(eval("5 - 3 - 2"), 0.into());
        assert_eq!(eval("5 / 1 / 2"), 2.5.into());
        assert_eq!(eval("3 * 5 * 2"), 30.into());
        assert_eq!(eval("1 / 0.5"), 2.into());
        assert_eq!(eval("2 * 1 / 3"), (2. * 1. / 3.).into());
    }

    #[test]
    fn factorial() {
        assert_eq!(eval("5 !"), 119.99999999999976.into());
    }

    #[test]
    fn log() {
        assert_eq!(eval("log ( 100 )"), 2.into());
        assert_eq!(eval("log 100"), 2.into());
    }

    #[test]
    fn ln() {
        assert_eq!(eval("ln ( 100 )"), 4.605170185988092.into());
        assert_eq!(eval("ln 100"), 4.605170185988092.into());
    }

    #[test]
    fn sqrt() {
        assert_eq!(eval("sqrt ( 100 )"), 10.into());
        assert_eq!(eval("sqrt 100"), 10.into());
    }

    #[test]
    fn order_of_operations() {
        assert_eq!(eval("2 * 3 + 1"), 7.into());
        assert_eq!(eval("1 + 2 * 3"), 7.into());
        assert_eq!(eval("1.0 + 1 - 2 * 10 / 0.5"), Quantity::from(-38));
    }

    #[test]
    fn dimensional_arith() {
        let input = "1 m + 2 m";
        assert_eq!(
            eval(input),
            Quantity {
                value: 3.,
                dimensions: Dimensions {
                    length: 1.,
                    ..Default::default()
                },
                ..Default::default()
            },
        );
    }
}
