// devela::data::collections::array
//
//! Arrays, extends `std::`[`array`].
//!
//! [`array`]: mod@std::array
//!
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

/* always compiled, non-public mdules */

// without re-exports
mod impl_traits;
mod methods;

mod array_init;
mod definitions;
pub use {array_init::*, definitions::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{array_init::*, definitions::*};
}
