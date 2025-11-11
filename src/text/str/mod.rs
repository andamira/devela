// devela::text::str
//
//! String types and related functionality.
//!
#![doc = crate::_doc!(extends: str, string)]

mod ext_str; // StrExt
mod reexports;

#[cfg(feature = "alloc")]
mod ext_string; // StringExt

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            ext_str::*,
            reexports::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::ext_string::*;
    }
}
