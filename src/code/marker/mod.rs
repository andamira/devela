// devela/src/code/marker/mod.rs
//
#![doc = crate::_DOC_CODE_MARKER!()] // public
#![doc = crate::_doc!(modules: crate::code; marker)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: marker)]
//

mod _reexport_core;

mod prim; // Prim, PrimFitPtr, PrimIndex, IndexRepr
mod repr; // Repr
mod type_marker; // zero-cost generic type markers

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            prim::*,
            repr::*,
            type_marker::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
