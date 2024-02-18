// devela::io::path
//
//! Paths, extends
//! `std::`[`path`][std::path].
//

/* feature-gated, non-public modules */
#[cfg(feature = "io")]
mod project;

#[cfg(feature = "io")]
pub use project::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "io")]
    pub use super::project::*;
}
