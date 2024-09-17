// devela::data::collections::array
//
//! Arrays.
#![doc = crate::code::doc_extends!(array)]
//!
//! Arrays are random-access, sequentially allocated, statically sized,
//! homogeneous data structures.
//!
//! They enable efficient iterable storage over a sequence of the same type.
//

mod d1; // 1-dimensional Array
mod d2; // 2-dimensional Array2d
mod ext; // ExtArray, ArrayFmt
mod init; // array_init!
#[allow(unused_imports)]
pub use {d1::all::*, d2::all::*, ext::*, init::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{d1::all::*, d2::all::*, ext::*, init::*};
}
