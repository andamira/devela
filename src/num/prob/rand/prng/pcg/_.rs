// devela/src/num/prob/rand/prng/pcg/_.rs
//
#![doc = concat![crate::_ABBR_PCG!(), "s."]]
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod generator; // rand_pcg!
}
crate::mods_out! { // _mods
    _mods {
        rand_pcg![pub Pcg32: (u32)];

        pub use super::{
            generator::rand_pcg,
        };
    }
}
