// devela_base::lang
//
#![doc = crate::_DOC_LANG!()]
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
