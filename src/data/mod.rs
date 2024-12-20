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

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use {_pub_mods::*, _mods::*};
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{bit::_all::*, fmt::_all::*, id::_all::*, sort::_all::*};
        #[cfg(feature = "data")]
        pub use super::error::*;
    }
    mod _pub_mods {
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{collections::_all::*, hash::_all::*, iter::_all::*};

        #[cfg_attr(not(feature = "__force_miri_dst"), cfg(not(miri)))]
        #[cfg(all(
            not(any(feature = "safe_data", feature = "safe_mem")),
            feature = "unsafe_layout"
        ))]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::dst::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::{_pub_mods::*, _mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{collections::_always::*, hash::_always::*, iter::_always::*};
    }
}
