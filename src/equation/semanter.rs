use crate::core::{
    quantity::{Quantity, QuantityFloat},
    unit::UNITS_LOOKUP,
};
use log::debug;

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
        "ufact -> ufact !",
                                (TryInto::<QuantityFloat>::try_into(n[0]).unwrap() + 1.0).into(),
        "quantity -> [n]",      n[0],
        "quantity -> [n] units",
                                Quantity::new(n[0].value, n[1].dimensions, n[1].units),
        "units -> unit",        n[0],
        "units -> units unit",  n[0] * n[1],
        "group -> quantity",    n[0],
        "group -> ( expr )",    n[1],
        "equation -> expr",     n[0],
        "equation -> expr [->] units",
                                n[0].set_units(&n[2].units)
    }
    ev

    //    debug_action!(ev, "num -> _num", n, n[0]);
    //    debug_action!(ev, "_num -> _num", n, n[0]);
    //    debug_action!(ev, "+num -> +num", n, n[0]);
    //    debug_action!(ev, "-num -> -num", n, n[0]);
    //
    //    debug_action!(ev, "unit -> unit", n, n[0]);
    //
    //    debug_action!(ev, "units -> unit [^] num", n, {
    //        let mut q = n[0];
    //        q.dimensions = n[0].dimensions.pow(n[2].value);
    //        q
    //    });
    //    debug_action!(ev, "units -> unit", n, n[0]);
    //    debug_action!(ev, "units -> units units", n, n[0].mul(&n[1]));
    //    debug_action!(ev, "units -> units [*] units", n, n[0].mul(&n[2]));
    //    debug_action!(ev, "units -> units [/] units", n, n[0].div(&n[2]));
    //
    //    debug_action!(ev, "expr -> quantity", n, n[0]);
    //    debug_action!(ev, "expr -> num", n, n[0]);
    //    debug_action!(ev, "expr -> group", n, n[0]);
    //    debug_action!(ev, "expr -> expr group", n, n[0].mul(&n[1]));
    //    debug_action!(ev, "expr -> expr quantity", n, n[0].mul(&n[1]));
    //
    //    debug_action!(ev, "expr -> expr [*] quantity", n, n[0].mul(&n[2]));
    //    debug_action!(ev, "expr -> expr [/] quantity", n, n[0].div(&n[2]));
    //    debug_action!(ev, "expr -> expr [%] quantity", n, {
    //        let mut q = n[0];
    //        q.value %= n[2].value;
    //        q
    //    });
    //    debug_action!(ev, "expr -> expr [+] quantity", n, n[0].add(&n[2]).unwrap());
    //    debug_action!(ev, "expr -> expr [-] quantity", n, n[0].sub(&n[2]).unwrap());
    //    debug_action!(ev, "expr -> expr [+] quantity", n, n[0].add(&n[2]).unwrap());
    //    debug_action!(ev, "expr -> expr [-] quantity", n, n[0].sub(&n[2]).unwrap());
    //    debug_action!(ev, "num -> num [!]", n, {
    //        let mut q = n[0];
    //        q.value = gamma(q.value + 1.);
    //        q
    //    });
    //
    //    debug_action!(ev, "quantity -> num", n, n[0]);
    //    debug_action!(ev, "quantity -> num units", n, {
    //        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    //    });
    //
    //    debug_action!(ev, "+quantity -> +num", n, n[0]);
    //    debug_action!(ev, "+quantity -> +num units", n, {
    //        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    //    });
    //
    //    debug_action!(ev, "-quantity -> -num", n, n[0]);
    //    debug_action!(ev, "-quantity -> -num units", n, {
    //        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    //    });
    //    debug_action!(ev, "quantity -> expr units", n, {
    //        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    //    });
    //
    //    debug_action!(ev, "quantity -> group [^] num", n, n[0].pow(n[2].value));
    //    debug_action!(ev, "quantity -> num [^] num", n, n[0].pow(n[2].value));
    //    debug_action!(ev, "quantity -> num [^] +num", n, n[0].pow(n[2].value));
    //    debug_action!(ev, "quantity -> num [^] -num", n, n[0].pow(n[2].value));
    //    debug_action!(ev, "group -> group [^] num", n, n[0].pow(n[2].value));
    //    debug_action!(ev, "expr -> num [^] num", n, n[0].pow(n[2].value));
    //    debug_action!(ev, "expr -> num [^] +num", n, n[0].pow(n[2].value));
    //    debug_action!(ev, "expr -> num [^] -num", n, n[0].pow(n[2].value));
    //    debug_action!(ev, "quantity -> sqrt group", n, {
    //        let mut q = n[1];
    //        q.value = q.value.sqrt();
    //        q
    //    });
    //    debug_action!(ev, "quantity -> log group", n, {
    //        let mut q = n[1];
    //        q.value = q.value.log10();
    //        q
    //    });
    //    debug_action!(ev, "quantity -> ln group", n, {
    //        let mut q = n[1];
    //        q.value = q.value.ln();
    //        q
    //    });
    //    debug_action!(ev, "quantity -> e", n, n[0]);
    //    debug_action!(ev, "quantity -> pi", n, n[0]);
    //
    //    debug_action!(ev, "group -> ( expr )", n, n[1]);
    //
    //    debug_action!(ev, "equation -> expr", n, n[0]);
}

#[link(name = "m")]
extern "C" {
    fn tgamma(x: f64) -> f64;
}
fn gamma(x: f64) -> f64 {
    unsafe { tgamma(x) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::dimension::Dimensions, core::quantity::Quantity, equation::parser::parser};

    fn init() {
        env_logger::init();
    }
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