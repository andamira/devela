// devela::exec::thread
//
//! Native threads, extends `std::`[`thread`].
//!
//! [`thread`]: std::thread
//

/* always compiled */

mod reexports;
mod sleep;

#[allow(unused_imports)]
pub use {reexports::*, sleep::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{reexports::*, sleep::*};
}
