// devela::work::thread
//
//! Native threads, extends `std::`[`thread`].
//!
//! [`thread`]: std::thread
//

/* modules */

// feature-gated, private
#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "work")))]
mod sleep;

/* re-exports */

// feature-gated, private
#[cfg(feature = "work")]
#[allow(unused_imports)]
pub use sleep::*;

#[cfg(feature = "work")]
pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::sleep::*;
}
