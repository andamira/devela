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
mod methods;
mod types;

#[cfg(not(feature = "data"))]
pub(crate) use {always::*, core_impls::*, methods::*, types::*};

/* feature-gated */

// private sub-modules

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {always::*, types::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always::*, types::*};
}
