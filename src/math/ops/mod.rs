// devela::math::ops
//
//! Operations, extends
//! `std::`[`ops`][std::ops].
//

/* contains always compiled items */

mod always_fns;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub(crate) use always_fns::*;

/* feature-gated */

mod float;

#[cfg(feature = "math")]
mod fns;

#[doc(inline)]
pub use float::*;

// re-export private sub-modules
pub use always_fns::*;
#[cfg(feature = "math")]
pub use fns::*;

#[cfg(feature = "math")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always_fns::*, fns::*};

    #[doc(inline)]
    pub use super::float::*;
}
