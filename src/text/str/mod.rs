// devela::text::str
//
//! String types and related functionality.
//!
#![doc = crate::doc_!(extends: str, string)]

mod reexports;

#[cfg(feature = "str")]
mod ext_str; // ExtStr
#[cfg(feature = "str")]
mod namespace; // Str

#[cfg(feature = "dep_const_str")]
mod r#macro; // str!

#[cfg(all(feature = "str", feature = "alloc"))]
mod ext_string;
#[cfg(feature = "_string_nonul")] // RETHINK
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_string_nonul")))]
mod nonul;
#[cfg(_string_u··)]
mod string_u;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::reexports::*;

        #[cfg(feature = "str")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "str")))]
        pub use super::{ext_str::*, namespace::*};
        #[cfg(all(feature = "str", feature = "alloc"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "str")))]
        pub use super::ext_string::*;

        #[cfg(feature = "_string_nonul")] // RETHINK
        pub use super::nonul::*;
        #[cfg(_string_u··)]
        pub use super::string_u::*;

        #[doc(inline)] #[cfg(feature = "dep_const_str")]
        pub use super::r#macro::str;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
