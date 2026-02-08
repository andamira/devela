// devela_base_alloc::num::dom::int::wrapper
//
//! Integer wrapper struct.
//

mod namespace; // IntAlloc

mod impl_factors;
// mod impl_stats; // TODO

crate::structural_mods! {
    _mods {
        pub use super::namespace::IntAlloc;
    }
}
