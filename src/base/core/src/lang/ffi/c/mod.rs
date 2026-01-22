// devela_base_core::lang::ffi::c
//
#![doc = crate::_DOC_LANG_FFI_C!()]
//

mod _reexport; // SYMLINK to /src/lang/ffi/c/_reexport_core.rs

mod libc; // c_mode_t, c_off_t

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            libc::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
