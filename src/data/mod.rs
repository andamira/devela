// devela::data
//
//! Data handling and manipulation.
#![doc = crate::code::doc_extends!(array, collections, hash, iter, vec)]
//!
//

// safety:
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]

mod array;
mod bit;
mod error;
mod sort;
#[allow(unused_imports)]
pub use {array::*, bit::*, error::*, sort::*};

pub mod collections;
pub mod hash;
pub mod id;
pub mod iter;
#[doc(no_inline)]
pub use {collections::*, hash::*, id::*, iter::*};

#[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
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
        array::*, bit::all::*, collections::all::*, error::*, hash::all::*, id::all::*,
        iter::all::*, sort::*,
    };

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_layout"))]
    #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
    pub use super::dst::*;
}
