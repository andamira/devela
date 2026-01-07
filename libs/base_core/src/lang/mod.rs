// devela_base_core::lang
//
#![doc = crate::_DOC_LANG!()]
#![doc = crate::_DOC_LANG_MODULES!()]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: ffi)]
//
// safety
#![cfg_attr(base_safe_lang, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_LANG_MODULES =
    crate::_doc!(modules: crate; lang: ffi); // dsl, hum
}

pub mod ffi;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            ffi::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_LANG_MODULES;
    }
}
