// devela_base_std::work
//
#![doc = crate::_DOC_WORK!()]
//
// safety
#![cfg_attr(all(feature = "base_safe", feature = "safe_work"), forbid(unsafe_code))]

pub mod process;
pub mod sync;
pub mod thread;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            process::_all::*,
            sync::_all::*,
            thread::_all::*,
        };
    }
}
