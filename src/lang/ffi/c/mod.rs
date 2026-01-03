// devela::lang::ffi::c
//
#![doc = crate::_DOC_LANG_FFI_C!()]
//!
//! Provides C-compatible types, constants, and symbols used when calling
//! C or POSIX APIs. These definitions mirror the C standard and POSIX
//! specifications, forming the low-level interface for FFI with libc and
//! other C-based libraries.
//

mod _reexport_core; // SYMLINK to /libs/base_core/src/lang/ffi/c/_reexport.rs
#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /libs/base_alloc/src/lang/ffi/c/_reexport.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;

        pub use devela_base_core::lang::ffi::c::{
            c_mode_t, c_off_t,
        };
    }
}
