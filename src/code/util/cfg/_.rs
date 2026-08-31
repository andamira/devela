// devela/src/code/util/cfg/_.rs
//
#![doc = crate::_DOC_CODE_UTIL_CFG!()] // public
#![doc = crate::_doc!(modules: crate::code::util; cfg)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod _reexport_core;

    mod deprecate; // deprecate_feature!
}
crate::mods_out! { // _mods, _reexports,
    _mods {
        #[doc(inline)]
        pub use super::{
            deprecate::deprecate_feature,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[doc = crate::_tags!(code procedural_macro)]
        pub use devela_macros::{
            cif, compile, compile_attr, // compile_doc,
        };
    }
}
