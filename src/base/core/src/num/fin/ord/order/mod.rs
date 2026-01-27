// devela_base_core::num::fin::ord::namespace
//
//! Algorithms and structures that depend on relative position rather than magnitude alone.
//

// mod _helpers; // _impl_order

mod namespace; // Order
// impls:
// mod gosper;
// mod hilbert;
// mod morton;
// mod peano;
// mod rowcol;

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
    _crate_internals {
        // pub(crate) use super::_helpers::*;
    }
}
