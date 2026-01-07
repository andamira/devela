// devela::lang
//
#![doc = crate::_DOC_LANG!()]
#![doc = crate::_DOC_LANG_MODULES!()]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: ffi)]
//
// LINKS
// - https://en.wikipedia.org/wiki/Language | https://es.wikipedia.org/wiki/Lenguaje
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_LANG_MODULES =
    crate::_doc!(modules: crate; lang: dsl, ffi, hum);
}

pub mod dsl; // forth, …
pub mod ffi; // c, glsl, js, …
pub mod hum; // art, nat, … (linguistics)

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            ffi::_all::*,
            dsl::_all::*,
            hum::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_LANG_MODULES;
        pub(crate) use super::ffi::_crate_internals::*;
    }
}
