// devela::data::collections::array
//
//! Arrays, extends `std::`[`array`].
//!
//! [`array`]: mod@std::array
//!
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//

/* always compiled */

mod d1; // 1-dimensional
mod d2; // 2-dimensional
#[allow(unused_imports)]
pub use {d1::all::*, d2::all::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{d1::all::*, d2::all::*};
}
