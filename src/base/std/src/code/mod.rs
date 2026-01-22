// devela_base_std::code
//
#![doc = crate::_DOC_CODE!()]
//
// safety
#![cfg_attr(base_safe_code, forbid(unsafe_code))]

mod result; // only tests for now

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
