// devela_base_core::sys::arch
//
#![doc = crate::_DOC_SYS_ARCH!()]
//

mod _reexport; // SYMLINK from /src/sys/arch/_reexport_core.rs

mod wasm;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::wasm::_all::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
