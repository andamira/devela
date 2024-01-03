// devela::io::path
//
//! Paths, extends
//! `std::`[`path`][std::path].
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "io")]
mod project;

// re-export private sub-modules
#[cfg(feature = "io")]
#[allow(unused_imports)]
pub use project::*;

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "io")]
    #[allow(unused_imports)]
    pub use super::project::*;
}
