// devela/src/code/ops/call/mod.rs
//
#![doc = crate::_DOC_CODE_OPS_OVERLOAD!()] // public
#![doc = crate::_doc!(modules: crate::code::ops; overload)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: ops)]
//

mod _reexport_core;

crate::structural_mods! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use super::_reexport_core::*;
    }
}
