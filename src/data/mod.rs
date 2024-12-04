// devela::data
//
//! Data handling and manipulation.
#![doc = crate::doc_!(modules: crate; data: collections, hash, iter)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: array, collections, hash, iter, vec)]
//
// safety
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]

mod bit;
mod error;
mod id;
mod fmt;
mod sort;

pub mod collections;
pub mod hash;
pub mod iter;

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
pub mod dst;

/* structural access */

#[allow(unused_imports)]
pub use {doc_hidden::*, doc_inline::*};

mod doc_inline {
    pub use super::{bit::all::*, error::*, fmt::all::*, id::all::*, sort::*};
}
mod doc_hidden {
    #[doc(hidden)]
    #[doc(no_inline)]
    pub use super::{collections::all::*, hash::all::*, iter::all::*};

    #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
    #[cfg(all(
        not(any(feature = "safe_data", feature = "safe_mem")),
        feature = "unsafe_layout"
    ))]
    pub use super::dst::all::*;
}
pub(super) mod all {
    #[doc(inline)]
    pub use super::{doc_hidden::*, doc_inline::*};
}
