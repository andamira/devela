// devela::data::cmp
//
//! Comparing and ordering values, extends `std::`[`cmp`].
//!
//! [`cmp`]: std::cmp
//!
//! This module defines many constant functions for comparing primitives, and the
//! [`pclamp`], [`pmax`] and [`pmin`] functions for comparing partially ordered values.
//

/* always compiled */

mod compare;
mod reexports;

#[allow(unused)]
pub use {compare::*, reexports::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{compare::*, reexports::*};
}
