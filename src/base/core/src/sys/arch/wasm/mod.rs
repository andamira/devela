// devela_base_core::sys::arch::wasm
//
//! Wasm architecture.
//

mod _reexport; // SYMLINK from /src/sys/arch/wasm/_reexport_core.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
