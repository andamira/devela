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

/* re-exports */

#[cfg(feature = "cmp")]
pub use all::*;
#[cfg(feature = "cmp")]
pub(crate) mod all {
    pub use super::{always_fns::*, float::*, fns::*};
}
