// devela_base::phys
//
#![doc = crate::_DOC_PHYS!()]
//
// safety
#![cfg_attr(all(feature = "base_safe", feature = "safe_phys"), forbid(unsafe_code))]

mod time;

crate::structural_mods! { // _mods
    _mods {
        pub use super::time::_all::*;
    }
}
