// devela::data::hash
//
//! Generic hashing support, extends `std::`[`hash`].
//!
//! [`hash`]: std::hash
//

/* modules */

// feature-gated, non-public
#[cfg(feature = "data")]
mod reexports;

/* re-exports */

// feature-gated, non-public
#[cfg(feature = "data")]
pub use reexports::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "data")]
    pub use super::reexports::*;
}
