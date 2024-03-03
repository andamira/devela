// devela::data
//
//! Data handling and manipulation, extends
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

/* always compiled, non-public modules */

mod cmp;
mod error;

pub use {cmp::all::*, error::*};

/* always compiled, public modules */

pub mod bit;
pub mod collections;
pub mod hash;
pub mod iter;

#[doc(no_inline)]
pub use {bit::all::*, collections::all::*, hash::all::*, iter::all::*};

/* feature-gated, public modules */

#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(all(feature = "unsafe_dyn", feature = "bytemuck")))
)]
#[cfg(all(
    not(feature = "safe_data"),
    feature = "unsafe_dyn",
    feature = "bytemuck",
))]
pub mod dst;

#[cfg(all(
    not(feature = "safe_data"),
    feature = "unsafe_dyn",
    feature = "bytemuck",
))]
#[doc(no_inline)]
pub use dst::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{
        bit::all::*, cmp::all::*, collections::all::*, error::*, hash::all::*, iter::all::*,
    };

    // feature-gated
    #[cfg(all(
        not(feature = "safe_data"),
        feature = "unsafe_dyn",
        feature = "bytemuck",
    ))]
    #[doc(inline)]
    pub use super::dst::*;
}
