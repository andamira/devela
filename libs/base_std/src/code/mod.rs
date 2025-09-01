// devela_base_std::code
//
#![doc = crate::_DOC_CODE!()]
//
// safety
#![cfg_attr(all(feature = "base_safe", feature = "safe_code"), forbid(unsafe_code))]

pub mod error;
pub mod panic;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            error::_all::*,
            panic::_all::*,
        };
    }
}
