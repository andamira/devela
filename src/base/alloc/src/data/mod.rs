// devela_base_alloc::data
//
#![doc = crate::_DOC_DATA!()]
#![doc = crate::_doc!(modules: crate; data: bit, list, key, uid)]
#![doc = crate::_doc!(newline)]
//!
//
// safety
#![cfg_attr(base_safe_data, forbid(unsafe_code))]

mod bit; // pub WIP
mod sort;

pub mod list;
pub mod key;
pub mod uid;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            bit::_all::*, // pub WIP
            sort::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            list::_all::*,
            key::_all::*,
            uid::_all::*,
        };
    }
}
