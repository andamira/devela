// devela_base_core::num::dom::int::int::wrapper
//
//! Integer wrapper struct.
//

mod namespace; // Int

mod impl_base;
mod impl_combinatorics;
mod impl_div;
mod impl_elem;
mod impl_factors;
mod impl_modulo;
mod impl_prime;
mod impl_root;

// WIPZONE
// mod _wip_impl_stats;

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
