// devela_base_alloc::data
//
#![doc = crate::_DOC_DATA!()]
#![doc = crate::_doc!(modules: crate; data: list, key, uid)]
#![doc = crate::_doc!(newline)]
//!
//
// safety
#![cfg_attr(base_safe_data, forbid(unsafe_code))]

mod bit;
mod sort;

pub mod list;
pub mod key;
pub mod uid;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            bit::_all::*,
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
