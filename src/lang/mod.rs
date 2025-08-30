// devela::lang
//
#![doc = crate::_DOC_LANG!()]
#![doc = crate::_doc!(modules: crate; lang: dsl, ffi, i18n, ling)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: ffi)]
//
// LINKS
// - https://en.wikipedia.org/wiki/Language | https://es.wikipedia.org/wiki/Lenguaje
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]

pub mod dsl; // forth, …
pub mod ffi; // c, glsl, js, …
pub mod i18n; // gettext, fluent, …
pub mod ling; // art, nat, … (linguistics)

// WIPZONE
// mod script;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::ffi::_all::*;
        pub use super::dsl::_all::*;
        pub use super::i18n::_all::*;
        pub use super::ling::_all::*;

        // WIPZONE
        // pub use super::script::*;
    }
    _crate_internals {
        pub(crate) use super::ffi::_crate_internals::*;
    }
}
