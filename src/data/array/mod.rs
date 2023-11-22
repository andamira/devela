// devela::data::array
//
//! Arrays, extends
//! `std::`[`array`][mod@std::array].
//!
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

/* contains always compiled items */

mod always;
#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use always::*;

/* feature-gated */

// private sub-modules
#[cfg(feature = "data")]
mod core_impls;
#[cfg(feature = "data")]
mod methods;
#[cfg(feature = "data")]
mod types;

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {always::*, types::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always::*, types::*};
}
