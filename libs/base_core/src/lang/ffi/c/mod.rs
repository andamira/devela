// devela_base_core::lang::ffi::c
//
#![doc = crate::_DOC_LANG_FFI_C!()]
//

mod libc; // c_mode_t, c_off_t
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            libc::*,
            reexports::*,
        };
    }
}
