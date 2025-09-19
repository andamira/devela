// devela::data::list::array
//
#![doc = crate::_DOC_DATA_LIST_ARRAY!()]
//!
#![doc = crate::_doc!(extends: array, vec)]
//!
//! They enable efficient iterable storage over a sequence of the same type.
//

mod adt; // DataArray
mod d1; // 1-dimensional Array
mod d2; // 2-dimensional Array2d
mod ext; // ExtArray, ArrayFmt

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod vec;

// re-exports
crate::mod_path!(_c "../../../../libs/base_core/src/data/list/array/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use super::{adt::*, d1::_all::*, d2::_all::*, ext::*};

        #[cfg(feature = "alloc")]
        pub use super::vec::_all::*;

        // re-exports
        pub use super::_c::*;
        #[doc(inline)]
        pub use devela_base_core::data::list::{ArrayFrom, array_init};
    }
}
