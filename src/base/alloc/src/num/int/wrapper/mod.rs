// devela_base_alloc::num::int::wrapper
//
//! Integer wrapper struct.
//

mod namespace; // IntAlloc

mod impl_factors;

// WIPZONE
// mod impl_stats;

crate::structural_mods! {
    _mods {
        pub use super::namespace::IntAlloc;
    }
}
