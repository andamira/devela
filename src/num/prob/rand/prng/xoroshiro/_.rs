// devela/src/num/prob/rand/prng/xoroshiro/_.rs
//
//! Pseudo-random number generators based on [Xoroxhiro].
//!
//! [Xoroshiro]: https://en.wikipedia.org/wiki/Xorshift#xoroshiro
//

crate::mods_in! {
    mod u128;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            u128::Xoroshiro128pp,
        };
    }
}
