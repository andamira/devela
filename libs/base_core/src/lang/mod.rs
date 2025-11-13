// devela_base_core::lang
//
#![doc = crate::_DOC_LANG!()]
#![doc = crate::_doc!(modules: crate; lang: dsl, ffi, hum)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: ffi)]
//
// safety
#![cfg_attr(base_safe_lang, forbid(unsafe_code))]

pub mod ffi;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            ffi::_all::*,
        };
    }
}
