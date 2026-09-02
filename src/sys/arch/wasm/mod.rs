// devela/src/sys/arch/wasm/mod.rs
//
//! WASM architecture functionality.
//

mod _reexport_core;

mod namespace; // Wasm

crate::mods_out! { // _mods, reexports
    _mods {
        pub use super::namespace::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
