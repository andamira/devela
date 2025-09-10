// devela_base_alloc::num
//
#![doc = crate::_DOC_NUM!()]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]

pub mod int;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            int::_all::*,
        };
    }
}
