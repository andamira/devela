// devela/src/code/ops/call/overload/_.rs
//
#![doc = crate::_DOC_CODE_OPS_OVERLOAD!()] // public
#![doc = crate::_doc!(modules: crate::code::ops; overload)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: ops)]
//

crate::mods_in! {
    mod _reexport_core;
}
crate::mods_out! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use super::_reexport_core::*;
    }
}
