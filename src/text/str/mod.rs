// devela::text::str
//
//! Strings.
#![doc = crate::doc_!(modules: crate; str)]
// #![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: str, string)]

use crate::items;

mod namespace;
mod reexports;
#[allow(unused_imports)]
pub use {namespace::*, reexports::*};

#[cfg(feature = "_string_nonul")] // RETHINK
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_string_nonul")))]
items! {
    mod nonul;
    #[allow(unused_imports)]
    pub use nonul::*;
}

#[cfg(_string_u·)]
items! {
    mod string_u;
    #[allow(unused_imports)]
    pub use string_u::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{namespace::*, reexports::*};

    #[doc(inline)]
    #[cfg(feature = "_string_nonul")] // RETHINK
    pub use super::nonul::*;

    #[doc(inline)]
    #[cfg(_string_u·)]
    pub use super::string_u::*;
}
