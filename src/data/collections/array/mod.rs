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

mod array_init;
mod d1; // 1-dimensional

pub use {array_init::*, d1::all::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{array_init::*, d1::all::*};
}
