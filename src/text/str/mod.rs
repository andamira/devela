// devela::text::str
//
//! String types and related functionality.
//!
#![doc = crate::_doc!(extends: str, string)]

mod ext_str; // StrExt

#[cfg(feature = "alloc")]
mod ext_string; // StringExt

// re-exports
mod reexports_core; // SYMLINK to /libs/base_core/src/text/str/reexports.rs
mod reexports_alloc; // SYMLINK to /libs/base_alloc/src/text/str/reexports.rs
mod reexports_std; // SYMLINK to /libs/base_std/src/text/str/reexports.rs

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ext_str::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::ext_string::*;

        /* re-exports */

        pub use super::{
            reexports_core::*,
            reexports_alloc::*,
            reexports_std::*,
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
