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
#[allow(unused_imports)]
pub use {bit::*, error::*, fmt::all::*, id::all::*, sort::*};

pub mod collections;
pub mod hash;
pub mod iter;
#[doc(no_inline)]
pub use {collections::all::*, hash::all::*, iter::all::*};

#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))] // FIX
crate::items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub mod dst;
    #[doc(no_inline)]
    pub use dst::all::*;
}

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::{
        bit::all::*, collections::all::*, error::*, fmt::*, hash::all::*, id::all::*, iter::all::*,
        sort::*,
    };

    #[doc(inline)]
    #[cfg(all(
        not(any(feature = "safe_data", feature = "safe_mem")),
        feature = "unsafe_layout"
    ))]
    #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
    pub use super::dst::*;
}
