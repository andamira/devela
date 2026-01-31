// devela::lang
//
#![doc = crate::_DOC_LANG!()] // public, root
#![doc = crate::_DOC_LANG_MODULES!()]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//
// LINKS
// - https://en.wikipedia.org/wiki/Language | https://es.wikipedia.org/wiki/Lenguaje
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_LANG_MODULES =
    crate::_doc!(modules: crate; lang: gram, hum, prog, repr, sem);
}

pub mod gram; // grammar machinery
pub mod hum; // human languages
pub mod prog; // programming languages
pub mod repr; // representation languages
pub mod sem; // semantic relations

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            hum::_all::*,
            prog::_all::*,
            repr::_all::*,
            sem::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_LANG_MODULES;
        pub(crate) use super::prog::_crate_internals::*;
    }
}
