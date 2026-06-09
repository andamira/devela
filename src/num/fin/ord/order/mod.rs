// devela::num::fin::ord::namespace
//
//! Algorithms and structures that depend on relative position rather than magnitude alone.
//

mod namespace; // Order
// impls:
// mod gosper;
// mod hilbert;
// mod morton;
// mod peano;
mod rowcol;

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
    }
}
