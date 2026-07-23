// devela/src/data/layout/array/mod.rs
//
#![doc = crate::_DOC_DATA_LAYOUT_ARRAY!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; array)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, vec)]
//!
//! They enable efficient iterable storage over a sequence of the same type.
//

mod _reexport_core;

mod adt; // DataArray
mod d1; // 1-dimensional Array TEMP
mod d2; // 2-dimensional Array2d TEMP
mod ext; // ArrayExt, ArrayFmt
mod from; // ArrayFrom
mod init; // init_array!
mod layout; // Array shape and layout foundations
mod view; // Array views over generic data carriers

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod vec;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            adt::DataArray,
            d1::_all::*, // Array, ArrayUninit
            d2::_all::Array2d,
            ext::{ArrayExt, ArrayFmt},
            from::ArrayFrom,
            init::init_array,
            layout::{ArrayLayout, ArrayShape},
            view::{ArrayView},
        };

        #[cfg(feature = "alloc")]
        pub use super::vec::_all::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
