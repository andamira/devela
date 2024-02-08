// devela::data::cmp
//
//! Comparing, ordering and sorting, extends `std::`[`cmp`].
//!
//! [`cmp`]: std::cmp
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

/* modules */

// always compiled, non-public
mod comparing;
mod sorting;

/* re-exports */

// always compiled, non-public
#[allow(unused)]
pub use {comparing::*, sorting::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{comparing::*, sorting::*};
}
