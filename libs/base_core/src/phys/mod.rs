// devela_base_core::phys
//
#![doc = crate::_DOC_PHYS!()]
#![doc = crate::_doc!(modules: crate; phys: time)] // bio, chem, elec, mech, time, unit, wave
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(base_safe_phys, forbid(unsafe_code))]

pub mod time;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::time::_all::*;
    }
}
