// devela_base_std::phys
//
#![doc = crate::_DOC_PHYS!()]
//
// safety
#![cfg_attr(base_safe_phys, forbid(unsafe_code))]

pub mod time;

crate::structural_mods! { // _mods
    _mods {
        pub use super::time::_all::*;
    }
}
