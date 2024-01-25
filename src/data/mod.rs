// devela::data
//
//! Data structures, extends
//! `std::{`[`any`], [`array`], [`cmp`], [`collections`], [`hash`],
//! [`iter`], [`vec`]`}`.
//!
//! [`any`]: core::any
//! [`array`]: mod@core::array
//! [`cmp`]: core::cmp
//! [`collections`]: alloc::collections
//! [`hash`]: std::hash
//! [`iter`]: core::iter
//! [`vec`]: mod@alloc::vec
//

#![cfg_attr(not(feature = "data"), allow(unused_imports))]

/* modules */

// always compiled, non-public
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
mod error;

// always compiled, public
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub mod bit;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub mod cmp;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub mod collections;
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub mod hash;

// feature-gated, public
#[cfg(feature = "data")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub mod any;
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "unsafe_data", feature = "dep")))
)]
#[cfg(all(
    feature = "data",
    feature = "unsafe_data",
    any(feature = "bytemuck", feature = "dep")
))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub mod dst;

/* re-exports */

// always,compiled, non-public
pub use error::*;

// always compiled, public
#[doc(no_inline)]
pub use {bit::all::*, cmp::all::*, collections::all::*, hash::all::*};

// feature-gated, public
#[doc(no_inline)]
#[cfg(feature = "data")]
pub use any::all::*;
#[doc(no_inline)]
#[cfg(all(
    feature = "data",
    feature = "unsafe_data",
    any(feature = "bytemuck", feature = "dep")
))]
pub use dst::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{bit::all::*, cmp::all::*, collections::all::*, error::*, hash::all::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "data")]
    pub use super::any::all::*;
    #[doc(inline)]
    #[cfg(all(
        feature = "data",
        feature = "unsafe_data",
        any(feature = "bytemuck", feature = "dep")
    ))]
    pub use super::dst::*;
}
