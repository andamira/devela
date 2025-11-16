// devela::lang::ffi::c
//
#![doc = crate::_DOC_LANG_FFI_C!()]
//!
//! Provides C-compatible types, constants, and symbols used when calling
//! C or POSIX APIs. These definitions mirror the C standard and POSIX
//! specifications, forming the low-level interface for FFI with libc and
//! other C-based libraries.
//

crate::mod_path!(_c "../../../../libs/base_core/src/lang/ffi/c/reexports.rs");
crate::mod_path!(alloc _a "../../../../libs/base_alloc/src/lang/ffi/c/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use devela_base_core::lang::ffi::c::{c_mode_t, c_off_t};

        pub use super::_c::*;
        #[cfg(feature = "alloc")]
        pub use super::_a::*;
    }
}
