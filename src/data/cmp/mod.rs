// devela::data::cmp
//
//! Comparing, ordering and sorting, extends `std::`[`cmp`].
//!
//! [`cmp`]: std::cmp
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

/* always compiled, non-public modules */
mod comparing;
mod sorting;

#[allow(unused)]
pub use {comparing::*, sorting::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{comparing::*, sorting::*};
}
