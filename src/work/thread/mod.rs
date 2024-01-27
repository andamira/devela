// devela::work::thread
//
//! Native threads, extends `std::`[`thread`].
//!
//! [`thread`]: std::thread
//

/* modules */

// non-public
mod reexports;
mod sleep;

/* re-exports */

// non-public
pub use {reexports::*, sleep::*};

#[cfg(feature = "work")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{reexports::*, sleep::*};
}
