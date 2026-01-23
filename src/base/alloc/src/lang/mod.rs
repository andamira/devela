// devela_base_alloc::lang
//
#![doc = crate::_DOC_LANG!()]
//
// safety
#![cfg_attr(base_safe_lang, forbid(unsafe_code))]

// pub mod gram; // grammar machinery
// pub mod hum; // human languages
pub mod prog; // programming languages
// pub mod repr; // representation languages
// pub mod sem; // semantic relations

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            prog::_all::*,
        };
    }
}
