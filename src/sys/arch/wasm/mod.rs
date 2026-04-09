// devela::sys::arch::wasm
//
//! WASM architecture functionality.
//

mod _reexport_core;

mod namespace; // Wasm

crate::structural_mods! { // _mods, reexports
    _mods {
        pub use super::namespace::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
