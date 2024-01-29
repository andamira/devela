// devela::io::path
//
//! Paths, extends
//! `std::`[`path`][std::path].
//

/* modules */

// feature-gated, non-public
#[cfg(feature = "io")]
mod project;

/* re-exports */

// feature-gated, non-public
#[cfg(feature = "io")]
pub use project::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "io")]
    pub use super::project::*;
}
