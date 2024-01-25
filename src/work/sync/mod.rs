// devela::work::sync
//
//! Synchronization, extends `std::`[`sync`].
//!
//! [`sync`]: std::sync
//

/* modules */

// feature-gated, public
#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "work")))]
pub mod atomic;

/* re-exports */

// feature-gated, public
#[doc(no_inline)]
#[cfg(feature = "work")]
pub use atomic::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "work")]
    pub use super::atomic::*;
}
