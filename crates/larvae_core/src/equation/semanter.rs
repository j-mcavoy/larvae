use crate::{quantity::Quantity, unit::UNITS_LOOKUP};
use log::debug;

macro_rules! debug_action {
    ($ev:ident, $action:literal, $n:ident, $exp:expr) => {
        $ev.action($action, |$n| {
            debug!($action);
            $exp
        });
    };
}
pub fn build_semanter<'a>() -> earlgrey::EarleyForest<'a, Quantity> {
    let mut ev = earlgrey::EarleyForest::new(symbol_match);

    debug_action!(ev, "num -> num", n, n[0]);
    debug_action!(ev, "+num -> +num", n, n[0]);
    debug_action!(ev, "-num -> -num", n, n[0]);

    debug_action!(ev, "unit -> unit", n, n[0]);

    debug_action!(ev, "units -> unit [^] num", n, {
        let mut q = n[0];
        q.dimensions = n[0].dimensions.pow(n[2].value);
        q
    });
    debug_action!(ev, "units -> unit", n, n[0]);
    debug_action!(ev, "units -> units units", n, n[0].mul(&n[1]));
    debug_action!(ev, "units -> units [*] units", n, n[0].mul(&n[2]));
    debug_action!(ev, "units -> units [/] units", n, n[0].mul(&n[2].inv()));

    debug_action!(ev, "expr -> quantity", n, n[0]);
    debug_action!(ev, "expr -> expr quantity", n, n[0].mul(&n[1]));
    debug_action!(ev, "expr -> expr +quantity", n, n[0].add(&n[1]).unwrap());
    debug_action!(ev, "expr -> expr -quantity", n, n[0].sub(&n[1]).unwrap());

    debug_action!(ev, "expr -> expr [+] quantity", n, n[0].add(&n[2]).unwrap());
    debug_action!(ev, "expr -> expr [-] quantity", n, n[0].sub(&n[2]).unwrap());
    debug_action!(ev, "expr -> expr [*] quantity", n, n[0].mul(&n[2]));
    debug_action!(ev, "expr -> expr [/] quantity", n, n[0].mul(&n[2].inv()));
    debug_action!(ev, "expr -> expr [%] quantity", n, {
        let mut q = n[0];
        q.value %= n[2].value;
        q
    });
    debug_action!(ev, "num -> num [!]", n, {
        let mut q = n[0];
        q.value = gamma(q.value + 1.);
        q
    });

    debug_action!(ev, "quantity -> num", n, n[0]);
    debug_action!(ev, "quantity -> num units", n, {
        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    });

    debug_action!(ev, "+quantity -> +num", n, n[0]);
    debug_action!(ev, "+quantity -> +num units", n, {
        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    });

    debug_action!(ev, "-quantity -> -num", n, n[0]);
    debug_action!(ev, "-quantity -> -num units", n, {
        Quantity::new(n[0].value, n[1].dimensions, n[1].units)
    });

    debug_action!(ev, "quantity -> group [^] num", n, n[0].pow(n[2].value));
    debug_action!(ev, "quantity -> num [^] num", n, n[0].pow(n[2].value));
    debug_action!(ev, "quantity -> sqrt group", n, {
        let mut q = n[1];
        q.value = q.value.sqrt();
        q
    });
    debug_action!(ev, "quantity -> log group", n, {
        let mut q = n[1];
        q.value = q.value.log10();
        q
    });
    debug_action!(ev, "quantity -> ln group", n, {
        let mut q = n[1];
        q.value = q.value.ln();
        q
    });
    debug_action!(ev, "quantity -> e", n, n[0]);
    debug_action!(ev, "quantity -> pi", n, n[0]);

    debug_action!(ev, "group -> ( expr )", n, n[1]);

    debug_action!(ev, "equation -> expr", n, n[0]);
    debug_action!(ev, "equation -> expr [->] units", n, {
        n[0].set_units(&n[2].units)
    });
    ev
}

fn symbol_match(symbol: &str, token: &str) -> Quantity {
    let out = match symbol {
        "+num" => Quantity::from_value(token.parse().unwrap()),
        "-num" => Quantity::from_value(token.parse().unwrap()),
        "num" => Quantity::from_value(token.parse().unwrap()),
        "e" => Quantity::from_value(std::f64::consts::E),
        "pi" => Quantity::from_value(std::f64::consts::PI),
        "unit" => *UNITS_LOOKUP.get(token).expect("invalid unit"),
        _ => Quantity::default(),
    };
    debug!("{:?}", out);
    out
}

fn gamma(x: f64) -> f64 {
    #[link(name = "m")]
    extern "C" {
        fn tgamma(x: f64) -> f64;
    }
    unsafe { tgamma(x) }
}
