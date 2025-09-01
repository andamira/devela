// devela_base_alloc::data
//
#![doc = crate::_DOC_DATA!()]
//
// safety
#![cfg_attr(all(feature = "base_safe", feature = "safe_work"), forbid(unsafe_code))]

// mod bit; // TODO
mod sort;

pub mod list;
pub mod key;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::sort::_all::*;
    }
    _pub_mods {
        pub use super::{list::_all::*, key::_all::*};
    }
}
