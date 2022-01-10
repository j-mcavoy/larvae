#[macro_use]
extern crate larvae_macros;
use larvae_macros::dimension;

use quote::quote;

dimension! {
    "length" 1. 0. 0.:
    "meter" "m" "m" 1.
    "kilometer" "km" "km" 1e3
}
dimension! {
    "mass" 1. 0. 0.:
    "gram" "g" "g" 1.
    "kilogram" "kg" "kg" 1e3
}

#[cfg(test)]
mod test {}
