// devela_base_core::data::layout::array
//
#![doc = crate::_DOC_DATA_LAYOUT_ARRAY!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; array)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, vec)]
//

mod _reexport; // SYMLINK from /src/data/layout/array/_reexport_core.rs

// mod d2; // array_2d! WIP
mod ext; // ArrayExt, ArrayFmt
mod from; // ArrayFrom
mod init; // init_array!

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            // d2::*, // WIP
            ext::*,
            from::*,
            init::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
