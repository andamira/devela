// devela/src/code/marker/_.rs
//
#![doc = crate::_DOC_CODE_MARKER!()] // public
#![doc = crate::_doc!(modules: crate::code; marker)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: marker)]
//

crate::mods_in! {
    mod _reexport_core;

    mod prim; // Prim, PrimFitPtr, PrimIndex, IndexRepr
    mod repr; // Repr
    mod type_marker; // zero-cost generic type markers
}
crate::mods_out! { // _mods, _reexports
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
