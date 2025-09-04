// devela_base_alloc::sys
//
#![doc = crate::_DOC_SYS!()]
//
// safety
#![cfg_attr(base_safe_sys, forbid(unsafe_code))]

pub mod mem;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            mem::_all::*,
        };
    }
}
