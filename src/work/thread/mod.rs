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
#[allow(unused_imports)]
pub use sleep::*;

#[cfg(feature = "work")]
pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::sleep::*;
}
