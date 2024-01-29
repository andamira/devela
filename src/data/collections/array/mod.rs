// devela::data::collections::array
//
//! Arrays, extends `std::`[`array`].
//!
//! [`array`]: mod@std::array
//!
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

/* modules */

// always compiled, non-public
mod always;
mod definitions;

// always compiled, non-public, nothing to re-export
mod core_impls;
mod methods;

/* re-export */

// always compiled, non-public
pub use {always::*, definitions::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{always::*, definitions::*};
}
