// devela_base::work
//
#![doc = crate::_DOC_WORK!()]
//
// safety
#![cfg_attr(all(feature = "base_safe", feature = "safe_work"), forbid(unsafe_code))]

pub mod future;
pub mod sync;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            future::_all::*,
            sync::_all::*,
        };
    }
}
