// devela::text::str
//
//! String types and related functionality.
//!
#![doc = crate::doc_!(extends: str, string)]

mod reexports;
mod macros; // join!, str!
mod namespace; // Str

#[cfg(feature = "str")]
mod ext_str; // ExtStr

#[cfg(all(feature = "str", feature = "alloc"))]
mod ext_string;
#[cfg(feature = "_str_nonul")] // RETHINK
#[cfg_attr(nightly_doc, doc(cfg(feature = "_str_nonul")))]
mod nonul;
#[cfg(_str_u··)]
mod string_u;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::reexports::*;
        pub use super::macros::*;
        pub use super::namespace::*;

        #[cfg(feature = "str")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "str")))]
        pub use super::ext_str::*;
        #[cfg(all(feature = "str", feature = "alloc"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "str")))]
        pub use super::ext_string::*;

        #[cfg(feature = "_str_nonul")] // RETHINK
        pub use super::nonul::*;
        #[cfg(_str_u··)]
        pub use super::string_u::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
