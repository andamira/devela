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
mod d2; // 2-dimensional
mod ext; // ExtArray, ArrayFmt

pub use {array_init::*, d1::all::*, d2::all::*, ext::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{array_init::*, d1::all::*, d2::all::*, ext::*};
}
