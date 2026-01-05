// devela::text::str
//
//! String types and related functionality.
//!
#![doc = crate::_doc!(extends: str, string)]

mod ext_str; // StrExt

#[cfg(feature = "alloc")]
mod ext_string; // StringExt

// re-exports
mod _reexport_core; // SYMLINK to /libs/base_core/src/text/str/_reexport.rs
mod _reexport_alloc; // SYMLINK to /libs/base_alloc/src/text/str/_reexport.rs
mod _reexport_std; // SYMLINK to /libs/base_std/src/text/str/_reexport.rs

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            ext_str::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::ext_string::*;
    }
    _reexports {
        pub use super::{
            _reexport_core::*,
            _reexport_alloc::*,
            _reexport_std::*,
        };
        #[doc(inline)]
        pub use devela_base_core::text::str::{
            Str, StringNonul, StringU8, StringU16, StringU32, StringUsize,
        };
        // from other modules
        pub use crate::CStr;
        #[cfg(feature = "alloc")]
        pub use crate::CString;
    }
}
