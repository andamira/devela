// devela::lang::prog::ffi::c
//
#![doc = crate::_DOC_LANG_PROG_FFI_C!()]
//!
//! Provides C-compatible types, constants, and symbols used when calling
//! C or POSIX APIs. These definitions mirror the C standard and POSIX
//! specifications, forming the low-level interface for FFI with libc and
//! other C-based libraries.
//

mod _reexport_core; // SYMLINK to /src/base/core/src/lang/prog/ffi/c/_reexport.rs
#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /src/base/alloc/src/lang/prog/ffi/c/_reexport.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;

        pub use devela_base_core::lang::prog::ffi::c::{
            c_mode_t, c_off_t,
        };
    }
}
