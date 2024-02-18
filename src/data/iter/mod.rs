// devela::data::iter
//
//! Composable external iteration, extends `std::`[`iter`].
//!
//! [`iter`]: std::iter
//

/* always compiled, non-public modules */
mod reexports;

#[allow(unused_imports)]
pub use reexports::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;
}
