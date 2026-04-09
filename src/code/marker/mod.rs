// devela::code::marker
//
#![doc = crate::_DOC_CODE_MARKER!()] // public
#![doc = crate::_doc!(modules: crate::code; marker)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: marker)]
//

mod _reexport_core;

mod prim; // Prim, PrimFitPtr, PrimIndex
mod type_marker; // zero-cost generic type markers
mod type_resource; // zero-cost type-safe resource markers

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            prim::*,
            type_marker::*,
            type_resource::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
