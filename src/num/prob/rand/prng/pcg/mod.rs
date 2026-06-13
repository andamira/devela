// devela/src/num/prob/rand/prng/pcg/mod.rs
//
#![doc = concat![crate::_ABBR_PCG!(), "s."]]
//

mod generator; // rand_pcg!

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        rand_pcg![pub Pcg32: (u32)];

        pub use super::{
            generator::*,
        };
    }
}
