// devela_base::sys
//
#![doc = crate::_DOC_SYS!()]
//
// safety
#![cfg_attr(base_safe_sys, forbid(unsafe_code))]

pub mod arch;
pub mod env;
pub mod mem;
pub mod net;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            arch::_all::*,
            env::_all::*,
            mem::_all::*,
            net::_all::*,
        };
    }
}
