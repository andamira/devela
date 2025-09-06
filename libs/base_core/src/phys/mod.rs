// devela_base_core::phys
//
#![doc = crate::_DOC_PHYS!()]
//
// safety
#![cfg_attr(base_safe_phys, forbid(unsafe_code))]

mod time;

crate::structural_mods! { // _mods
    _mods {
        pub use super::time::_all::*;
    }
}
