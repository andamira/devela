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
mod id;
mod fmt;
mod sort;

#[cfg(feature = "data")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
mod error;

pub mod collections;
pub mod hash;
pub mod iter;

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
pub mod dst;

crate::items! { // structural access: doc_inline, doc_hidden, all, always
    #[allow(unused)]
    pub use {doc_hidden::*, doc_inline::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{bit::all::*, fmt::all::*, id::all::*, sort::all::*};
        #[cfg(feature = "data")]
        pub use super::error::*;
    }
    mod doc_hidden {
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{collections::all::*, hash::all::*, iter::all::*};

        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::dst::all::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::{doc_hidden::*, doc_inline::*};
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{collections::always::*, hash::always::*, iter::always::*};
    }
}
