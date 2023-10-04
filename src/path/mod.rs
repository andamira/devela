// devela::path
//
//! Paths, extends [`std::path`].
//

/* always compiled for internal use */

/* only compiled with the `path` feature */

#[cfg(feature = "path")]
mod project;

/* re-exports */

#[cfg(feature = "path")]
pub use all::*;
#[cfg(feature = "path")]
pub(crate) mod all {
    pub use super::project::*;
}
