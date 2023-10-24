// devela::cmp
//
//! Comparing and ordering, extends
//! `std::`[`cmp`][std::cmp].
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

/* contains always compiled items */

mod always_fns;

#[allow(unused)]
#[cfg(not(feature = "cmp"))]
pub(crate) use always_fns::*;

/* feature-gated */

#[cfg(feature = "cmp")]
mod float;
#[cfg(feature = "cmp")]
mod fns;

// re-export private sub-modules
#[cfg(feature = "cmp")]
pub use {always_fns::*, float::*, fns::*};

#[cfg(feature = "cmp")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always_fns::*, float::*, fns::*};
}
