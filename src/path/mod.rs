// devela::path
//
//! Paths, extends
//! `std::`[`path`][std::path].
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "path")]
mod project;

// re-export private sub-modules
#[cfg(feature = "path")]
pub use project::*;

#[cfg(feature = "path")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::project::*;
}
