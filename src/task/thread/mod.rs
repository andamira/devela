// devela::task::thread
//
//! Native threads, extends
//! `std::`[`thread`][std::thread].
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "task")]
mod sleep;

// re-export private sub-modules
#[cfg(feature = "task")]
pub use sleep::*;

#[cfg(feature = "task")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::sleep::*;
}
