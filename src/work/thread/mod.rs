// devela::work::thread
//
//! Native threads, extends `std::`[`thread`].
//!
//! [`thread`]: std::thread
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "work")]
mod sleep;

// re-export private sub-modules
#[cfg(feature = "work")]
pub use sleep::*;

#[cfg(feature = "work")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::sleep::*;
}
