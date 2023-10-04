// devela::thread
//
//! Native threads, extends [`std::thread`].
//

/* always compiled for internal use */

/* only compiled with the `thread` feature */

#[cfg(feature = "thread")]
mod sleep;

/* re-exports */

#[cfg(feature = "thread")]
pub use all::*;
#[cfg(feature = "thread")]
pub(crate) mod all {
    pub use super::sleep::*;
}
