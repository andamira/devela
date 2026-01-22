// devela_base_std::data
//
#![doc = crate::_DOC_DATA!()]
//
// safety
#![cfg_attr(base_safe_code, forbid(unsafe_code))]

pub mod codec;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            codec::_all::*,
        };
    }
}
