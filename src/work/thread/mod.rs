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
#[allow(unused_imports)]
pub use {reexports::*, sleep::*};

#[cfg(feature = "work")]
pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{reexports::*, sleep::*};
}
