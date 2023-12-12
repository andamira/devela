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

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "path")]
    pub use super::project::*;
}
