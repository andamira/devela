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
mod ext; // ArrayExt, ArrayFmt
mod from; // ArrayFrom
mod init; // init_array!
mod layout; // Array shape and layout foundations
mod owned; // Owning array containers and storage-specific utilities
mod view; // Array views over generic data carriers

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            adt::DataArray,
            ext::{ArrayExt, ArrayFmt},
            from::ArrayFrom,
            init::init_array,
            layout::{ArrayLayout, ArrayShape},
            owned::_all::*,
            view::ArrayView,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
