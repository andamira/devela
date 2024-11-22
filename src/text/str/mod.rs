// devela::text::str
//
//! String types and related functionality.
//!
#![doc = crate::doc_!(extends: str, string)]

mod ext_str;
mod namespace;
mod reexports;
#[allow(unused_imports)]
pub use {ext_str::*, namespace::*, reexports::*};

#[cfg(feature = "alloc")]
crate::items! {
    mod ext_string;
    pub use ext_string::*;
}

#[cfg(feature = "_string_nonul")] // RETHINK
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_string_nonul")))]
crate::items! {
    mod nonul;
    #[allow(unused_imports)]
    pub use nonul::*;
}

#[cfg(_string_u·)]
crate::items! {
    mod string_u;
    #[allow(unused_imports)]
    pub use string_u::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ext_str::*, namespace::*, reexports::*};

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::ext_string::*;

    #[doc(inline)]
    #[cfg(feature = "_string_nonul")] // RETHINK
    pub use super::nonul::*;

    #[doc(inline)]
    #[cfg(_string_u·)]
    pub use super::string_u::*;
}
