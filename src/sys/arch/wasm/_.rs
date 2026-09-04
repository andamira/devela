// devela/src/sys/arch/wasm/_.rs
//
//! WASM architecture functionality.
//

crate::mods_in! {
    mod _reexport_core;

    mod namespace; // Wasm
}
crate::mods_out! { // _mods, reexports
    _mods {
        pub use super::namespace::Wasm;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
