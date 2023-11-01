// devela::ops
//
//! Operations, comparison, conversion, extends
//! `std::{`[`cmp`][std::cmp],
//! [`convert`][std::convert],
//! [`ops`][std::ops]`}`.
//

/* contains always compiled items */

pub mod cmp;
mod fp;

#[allow(unused)]
#[cfg(not(feature = "ops"))]
pub(crate) use {cmp::*, fp::*};

/* feature-gated */

#[cfg(feature = "ops")]
pub mod convert;

#[cfg(feature = "ops")]
mod fns;

// re-export private sub-modules
#[cfg(feature = "ops")]
pub use {fns::*, fp::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "ops")]
pub use {cmp::all::*, convert::all::*};

#[cfg(feature = "ops")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{cmp::all::*, convert::all::*, fns::*, fp::*};
}
