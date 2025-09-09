// devela::num::int::wrapper
//
//! Integer wrapper struct.
//

mod namespace; // Int

crate::items! {
    mod impl_base;
    mod impl_combinatorics;
    mod impl_core;
    mod impl_div;
    mod impl_factors;
    mod impl_modulo;
    mod impl_prime;
    mod impl_root;

    // WIPZONE
    // mod impl_stats;
}

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
