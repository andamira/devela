// devela::math::ops
//
//! Operations, extends
//! `std::`[`ops`].
//!
//! [`ops`]: std::ops
//

/* contains always compiled items */

mod always_fns;
mod float;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub(crate) use {always_fns::*, float::*};

/* feature-gated */

#[cfg(feature = "math")]
mod fns;

// re-export private sub-modules
pub use always_fns::*;
#[cfg(feature = "math")]
pub use {float::*, fns::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always_fns::*, float::*};

    #[doc(inline)]
    #[cfg(feature = "math")]
    pub use super::fns::*;
}
