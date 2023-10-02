// devela::cmp
//
//! Comparing and ordering, extends [`core::cmp`].
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

/* always compiled for internal use */

mod always_fns;
#[cfg(not(feature = "cmp"))]
pub(crate) use always_fns::*;

/* only compiled with the `cmp` feature */

#[cfg(feature = "cmp")]
mod float;
#[cfg(feature = "cmp")]
mod fns;

#[cfg(feature = "cmp")]
pub use {always_fns::*, float::*, fns::*};
