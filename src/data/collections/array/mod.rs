// devela::data::collections::array
//
//! Arrays.
//!
#![doc = crate::doc_!(extends: array)]
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

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{d1::_all::*, d2::_all::*, ext::*, init::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
