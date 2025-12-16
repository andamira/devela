// devela_base_std::sys
//
#![doc = crate::_DOC_SYS!()]
//
// safety
#![cfg_attr(base_safe_sys, forbid(unsafe_code))]

pub mod arch;
pub mod env;
pub mod fs;
pub mod mem;
pub mod net;
pub mod os;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            arch::_all::*,
            env::_all::*,
            fs::_all::*,
            mem::_all::*,
            net::_all::*,
            os::_all::*,
        };
    }
}
