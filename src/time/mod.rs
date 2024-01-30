// devela::time
//
//! Temporal quantification, extends
//! `std::`[`time`].
//!
//! [`time`]: std::time
//

// safety:
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

/* modules */

// feature-gated, private
#[cfg(feature = "time")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
mod reexports;

/* re-exports */

// feature-gated, private
#[cfg(feature = "time")]
pub use reexports::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use super::reexports::*;
}
