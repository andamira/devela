// devela/src/data/layout/array/_.rs
//
#![doc = crate::_DOC_DATA_LAYOUT_ARRAY!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; array)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, vec)]
//!
//! Array primitives, owning utilities,
//! and logical multidimensional views over backing storage.
//

crate::mods_in! {
    mod _reexport_core;

    mod adt; // DataArray

    mod define; // Array
    mod_ backing; // Array impls over backing storage

    mod coord; // ArrayCoordIter and coordinate ops
    mod layout; // ArrayShape
    mod shape; // ArrayShape

    mod ext; // ArrayExt, ArrayFmt
    mod from; // ArrayFrom
    mod init; // init_array!
    mod_ owned; // Owning array containers and storage-specific utilities
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            adt::DataArray,
            coord::ArrayCoordIter,
            define::Array,
            ext::{ArrayExt, ArrayFmt},
            from::ArrayFrom,
            init::init_array,
            layout::ArrayLayout,
            shape::ArrayShape,
            owned::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
