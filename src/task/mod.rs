// devela::task
//
//! Asynchronous tasks, extends [`core::task`].
//

/* always compiled for internal use */

/* only compiled with the `task` feature */

/* re-exports */

#[cfg(feature = "task")]
mod reexports;

#[cfg(feature = "task")]
pub use all::*;
#[cfg(feature = "task")]
pub(crate) mod all {
    pub use super::reexports::*;
}
