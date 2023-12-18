// devela::data::cmp
//
//! Comparing, ordering and sorting, extends `std::`[`cmp`].
//!
//! [`cmp`]: std::cmp
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

/* contains always compiled items */

mod comparing;
mod sorting;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub use {comparing::*, sorting::*};

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {comparing::*, sorting::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{comparing::*, sorting::*};
}
