// devela_base_core::lang::prog::ffi::c
//
#![doc = crate::_DOC_LANG_PROG_FFI_C!()]
//

mod _reexport; // SYMLINK from /src/lang/prog/ffi/c/_reexport_core.rs

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
