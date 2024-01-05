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

#[cfg(feature = "num")]
mod counting;
#[cfg(feature = "num")]
mod factors;
#[cfg(feature = "num")]
mod fns;

// re-export private sub-modules
#[allow(unused)]
pub use always_fns::*;
#[cfg(feature = "num")]
pub use {counting::*, factors::*, fns::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use super::{counting::*, factors::*, fns::*};
}
