// devela::num::cmp
//
//! Comparing and ordering, extends
//! `std::`[`cmp`][std::cmp].
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

/* contains always compiled items */

mod always_fns;
mod float;

#[allow(unused)]
#[cfg(not(feature = "num"))]
pub(crate) use {always_fns::*, float::*};

/* feature-gated */

#[cfg(feature = "num")]
mod fns;

// re-export private sub-modules
#[cfg(feature = "num")]
pub use {always_fns::*, float::*, fns::*};

#[cfg(feature = "num")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always_fns::*, float::*, fns::*};
}
