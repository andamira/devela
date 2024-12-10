// devela::text::str
//
//! String types and related functionality.
//!
#![doc = crate::doc_!(extends: str, string)]

mod reexports;

#[cfg(feature = "str")]
mod ext_str;
#[cfg(feature = "str")]
mod namespace;

#[cfg(all(feature = "str", feature = "alloc"))]
mod ext_string;
#[cfg(feature = "_string_nonul")] // RETHINK
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_string_nonul")))]
mod nonul;
#[cfg(_string_u·)]
mod string_u;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        pub use super::reexports::*;
        #[cfg(feature = "str")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "str")))]
        pub use super::{ext_str::*, namespace::*};
        #[cfg(all(feature = "str", feature = "alloc"))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "str")))]
        pub use super::ext_string::*;
        #[cfg(feature = "_string_nonul")] // RETHINK
        pub use super::nonul::*;
        #[cfg(_string_u·)]
        pub use super::string_u::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::reexports::*;
    }
}
