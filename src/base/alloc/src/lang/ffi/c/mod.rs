// devela_base_alloc::lang::ffi::c
//
#![doc = crate::_DOC_LANG_FFI_C!()]
//

mod _reexport; // SYMLINK to /src/lang/ffi/c/_reexport_alloc.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
