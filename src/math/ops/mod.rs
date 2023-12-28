// devela::math::ops
//
//! Operations, extends
//! `std::`[`ops`].
//!
//! [`ops`]: std::ops
//

/* contains always compiled items */

mod always_fns;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub use always_fns::*;

/* feature-gated */

#[cfg(feature = "math")]
mod fns;

// re-export private sub-modules
#[allow(unused)]
pub use always_fns::*;
#[cfg(feature = "math")]
pub use fns::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[cfg(feature = "math")]
    pub use super::fns::*;
}
