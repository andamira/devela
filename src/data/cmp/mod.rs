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
mod compare;
mod sort;

#[allow(unused)]
pub use {compare::*, sort::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{compare::*, sort::*};
}
