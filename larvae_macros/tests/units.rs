#[macro_use]
extern crate larvae_macros;
use larvae_macros::dimension;
use larvae_macros::metric;

use quote::quote;

dimension! {
    ^"length" 1. 0. 0.:
    "meter" "m" "m" 1.!
    ^"mass" 1. 0. 0.:
    "gram" "g" "g" 1.!
    ^"time" 0. 0. 1.:
    "second" "s" "sec" 1.!
    "minute" "min" "min" 1.
    ^"force" 1. 1. -1.:
    "newton" "N" "N" 1.!
}

#[cfg(test)]
mod test {}
