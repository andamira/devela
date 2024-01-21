// devela::num::ops
//
//! Operations, extends
//! `std::`[`ops`].
//!
//! [`ops`]: std::ops
//

/* contains always compiled items */

mod always_fns;

#[allow(unused)]
#[cfg(not(feature = "num"))]
pub use always_fns::*;

/* feature-gated */

// re-export private sub-modules
#[allow(unused)]
pub use always_fns::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;
}
