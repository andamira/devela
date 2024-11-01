// devela::data
//
//! Data handling and manipulation.
#![doc = crate::doc_!(extends: array, collections, hash, iter, vec)]
#![doc = crate::doc_!(modules: crate; data: collections, hash, id, iter)]
#![doc = crate::doc_!(newline)]
//!
//

// safety:
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]

mod bit;
mod error;
mod fmt;
mod sort;
#[allow(unused_imports)]
pub use {bit::*, error::*, fmt::all::*, sort::*};

pub mod collections;
pub mod hash;
pub mod id;
pub mod iter;
#[doc(no_inline)]
pub use {collections::*, hash::*, id::*, iter::*};

#[cfg(all(not(any(feature = "safe_data", feature = "safe_mem")), feature = "unsafe_layout"))]
#[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))] // FIX
crate::items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    pub mod dst;
    #[doc(no_inline)]
    pub use dst::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{
        bit::all::*, collections::all::*, error::*, fmt::*, hash::all::*, id::all::*, iter::all::*,
        sort::*,
    };

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(all(
        not(any(feature = "safe_data", feature = "safe_mem")),
        feature = "unsafe_layout"
    ))]
    #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
    pub use super::dst::*;
}
