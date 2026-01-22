// devela::data::list::array
//
#![doc = crate::_DOC_DATA_LIST_ARRAY!()]
//!
#![doc = crate::_doc!(extends: array, vec)]
//!
//! They enable efficient iterable storage over a sequence of the same type.
//

mod _reexport_core; // SYMLINK to /src/base/core/src/data/list/array/_reexport.rs

mod adt; // DataArray
mod d1; // 1-dimensional Array
mod d2; // 2-dimensional Array2d

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod vec;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            adt::*,
            d1::_all::*,
            d2::_all::*,
        };

        #[cfg(feature = "alloc")]
        pub use super::vec::_all::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[doc(inline)]
        pub use devela_base_core::data::list::{
            ArrayExt, ArrayFmt, ArrayFrom, init_array,
        };
    }
}
