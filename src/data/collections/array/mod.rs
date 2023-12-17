// devela::data::collections::array
//
//! Arrays, extends `std::`[`array`].
//!
//! [`array`]: mod@std::array
//!
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

/* contains always compiled items */

// no new definitions
mod core_impls;
mod methods;

mod always;
mod definitions;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub use {always::*, core_impls::*, definitions::*, methods::*};

/* feature-gated */

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {always::*, definitions::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{always::*, definitions::*};
}
