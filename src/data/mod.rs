// devela::data
//
//! Data handling and manipulation, extends
//! `std::{`[`array`], [`collections`], [`hash`],
//! [`iter`], [`vec`]`}`.
//!
//! [`array`]: mod@std::array
//! [`collections`]: std::collections
//! [`hash`]: std::hash
//! [`iter`]: std::iter
//! [`vec`]: mod@std::vec
//

// warnings:
#![cfg_attr(not(feature = "data"), allow(unused_imports))]
// safety:
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]

/* always compiled */

mod error;
mod sort;

pub use {error::*, sort::Sort};

pub mod bit;
pub mod collections;
pub mod hash;
pub mod iter;

#[doc(no_inline)]
pub use {bit::all::*, collections::all::*, hash::all::*, iter::all::*};

/* feature-gated */

#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_dyn")))]
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
pub mod dst;

#[doc(no_inline)]
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
pub use dst::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{bit::all::*, collections::all::*, error::*, hash::all::*, iter::all::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_dyn"))]
    pub use super::dst::*;
}
