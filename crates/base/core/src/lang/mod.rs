// devela_base_core::lang
//
#![doc = crate::_DOC_LANG!()] // public, root
#![doc = crate::_DOC_LANG_MODULES!()]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_LANG_MODULES =
    crate::_doc!(modules: crate; lang: prog); // disc, gram, hum, repr, sem
}

mod disc; //  discourse & expression
mod gram; // grammar machinery
mod hum; // human languages
pub mod prog; // programming languages
mod repr; // representation languages
mod sem; // semantic relations

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            disc::_all::*,
            gram::_all::*,
            hum::_all::*,
            prog::_all::*,
            repr::_all::*,
            sem::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_LANG_MODULES;
    }
}
