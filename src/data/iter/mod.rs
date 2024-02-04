// devela::data::iter
//
//! Composable external iteration, extends `std::`[`iter`].
//!
//! [`iter`]: std::iter
//

/* modules */

// always compiled, non-public
mod reexports;

/* re-exports */

// always compiled, non-public
#[allow(unused_imports)]
pub use reexports::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;
}
