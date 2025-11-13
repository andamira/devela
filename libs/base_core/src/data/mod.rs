// devela_base_core::data
//
#![doc = crate::_DOC_DATA!()]
//
// safety
#![cfg_attr(base_safe_data, forbid(unsafe_code))]

mod bit; // Bitwise, bitfield!
mod handle; // define_handle!
mod sort; // Sort

pub mod codec;
pub mod errors;
pub mod iter;
pub mod list;
// pub mod key;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{bit::_all::*, handle::*, sort::_all::*};
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::errors::*;
        pub use super::{codec::_all::*, iter::_all::*, list::_all::*};
        // pub use super::key::_all::*;
    }
}
