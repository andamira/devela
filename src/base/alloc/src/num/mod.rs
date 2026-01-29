// devela_base_alloc::num
//
#![doc = crate::_DOC_NUM!()]
//
// safety
#![cfg_attr(base_safe_num, forbid(unsafe_code))]

pub mod dom; // Numeric domains and value representations
pub mod prob; // Probability theory and statistical inference

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            dom::_all::*,
            prob::_all::*,
        };
    }
}
