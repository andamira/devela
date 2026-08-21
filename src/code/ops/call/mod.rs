// devela/src/code/ops/call/mod.rs
//
#![doc = crate::_DOC_CODE_OPS_CALL!()] // public
#![doc = crate::_doc!(modules: crate::code::ops; call)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: ops)]
//

mod _reexport_core;

mod semantics; // Call[Semantics|BindTime|Context|Dispatch|Openness|Storage]

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            semantics::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::_reexport_core::*;
    }
}
