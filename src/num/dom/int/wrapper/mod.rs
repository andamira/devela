// devela::num::dom::int::wrapper
//
//! Integer wrapper struct.
//

mod namespace; // Int

mod impl_base;
mod impl_combinatorics;
mod impl_div;
mod impl_elem;
mod impl_factors;
#[cfg(feature = "alloc")]
mod impl_factors_alloc;
mod impl_modulo;
mod impl_prime;
mod impl_root;
// mod impl_stats; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
