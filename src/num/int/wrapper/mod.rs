// devela::num::int::wrapper
//
//! Integer wrapper struct.
//

mod namespace; // Int

#[cfg(_int··)]
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

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::namespace::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
