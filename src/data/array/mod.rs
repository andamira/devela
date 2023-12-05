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
mod core_impls;
mod definitions;
mod methods;

#[cfg(not(feature = "data"))]
pub(crate) use {always::*, core_impls::*, definitions::*, methods::*};

/* feature-gated */

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {always::*, definitions::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always::*, definitions::*};
}
