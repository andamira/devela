// devela_base_core::code::marker
//
#![doc = crate::_DOC_CODE_MARKER!()] // public
#![doc = crate::_doc!(modules: crate::code; marker)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: marker)]
//

mod _reexport; // SYMLINK from /src/code/marker/_reexport_core.rs

mod prim; // Prim, PrimFitPtr
mod type_marker; // zero-cost generic type markers

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            prim::*,
            type_marker::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
