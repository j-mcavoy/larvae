use syn::{LitFloat, LitStr};

mod parse;

pub struct Unit {
    pub name: LitStr,
    pub abbrev: LitStr,
    pub symbol: LitStr,
    pub conversion_factor: LitFloat,
}
pub struct UnitSystems(pub Vec<UnitSystem>);

pub struct UnitSystem {
    pub dimension: LitStr,
    pub units: Vec<Unit>,
    pub dim_l: LitFloat,
    pub dim_m: LitFloat,
    pub dim_t: LitFloat,
}

pub const BASE_DIMENSIONS: &[&str] = &["length", "mass", "time"];
