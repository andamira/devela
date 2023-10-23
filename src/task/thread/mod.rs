// devela::task::thread
//
//! Native threads, extends `std::`[`thread`][std::thread].
//

/* always compiled for internal use */

/* only compiled with the `thread` feature */

#[cfg(feature = "task")]
mod sleep;

/* re-exports */

#[cfg(feature = "task")]
pub use all::*;
#[cfg(feature = "task")]
pub(crate) mod all {
    pub use super::sleep::*;
}
