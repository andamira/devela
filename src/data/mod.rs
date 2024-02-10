// devela::data
//
//! Data structures, extends
//! `std::{`[`array`], [`cmp`], [`collections`], [`hash`],
//! [`iter`], [`vec`]`}`.
//!
//! [`array`]: mod@std::array
//! [`cmp`]: std::cmp
//! [`collections`]: std::collections
//! [`hash`]: std::hash
//! [`iter`]: std::iter
//! [`vec`]: mod@std::vec
//

// warnings:
#![cfg_attr(not(feature = "data"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]

/* modules */

// always compiled, non-public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
mod cmp;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
mod error;

// always compiled, public
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
pub mod bit;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
pub mod collections;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
pub mod hash;
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
pub mod iter;

// feature-gated, public
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(all(feature = "unsafe_dyn", feature = "dep")))
)]
#[cfg(all(
    feature = "data",
    feature = "unsafe_dyn",
    any(feature = "bytemuck", feature = "dep"),
    not(feature = "safe_data")
))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "data")))]
pub mod dst;

/* re-exports */

// always,compiled, non-public
pub use {cmp::all::*, error::*};

// always compiled, public
#[doc(no_inline)]
pub use {bit::all::*, collections::all::*, hash::all::*, iter::all::*};

// feature-gated, public
#[doc(no_inline)]
#[cfg(all(
    feature = "data",
    feature = "unsafe_dyn",
    any(feature = "bytemuck", feature = "dep"),
    not(feature = "safe_data"),
))]
pub use dst::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        bit::all::*, cmp::all::*, collections::all::*, error::*, hash::all::*, iter::all::*,
    };

    // feature-gated
    #[doc(inline)]
    #[cfg(all(
        feature = "data",
        feature = "unsafe_dyn",
        any(feature = "bytemuck", feature = "dep"),
        not(feature = "safe_data"),
    ))]
    pub use super::dst::*;
}
