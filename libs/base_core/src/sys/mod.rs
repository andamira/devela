// devela_base_core::sys
//
#![doc = crate::_DOC_SYS!()]
//
// safety
#![cfg_attr(base_safe_sys, forbid(unsafe_code))]

pub mod arch;
pub mod env;
pub mod log;
pub mod mem;
pub mod net;

crate::structural_mods! { // _pub_mods, _hidden
    _pub_mods {
        pub use super::{
            arch::_all::*,
            env::_all::*,
            log::_all::*,
            mem::_all::*,
            net::_all::*,
        };
    }
    _hidden {
        use super::mem::_hidden::*;
    }
}
