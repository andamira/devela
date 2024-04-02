// devela::sys::path
//
//! Paths, extends
//! `std::`[`path`][std::path].
//

/* feature-gated */
#[cfg(feature = "sys")]
mod project;
#[cfg(feature = "sys")]
pub use project::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "sys")]
    pub use super::project::*;
}
