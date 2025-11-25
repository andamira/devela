// devela_base_core::data
//
#![doc = crate::_DOC_DATA!()]
#![doc = crate::_doc!(modules: crate; data: codec, errors, iter, list)] // key, table, uid
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
//
// safety
#![cfg_attr(base_safe_data, forbid(unsafe_code))]

mod bit; // Bitwise, bitfield!
mod handle; // define_handle!
mod sort; // Sort

pub mod codec;
pub mod errors;
pub mod iter;
// pub mod key;
pub mod list;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{bit::_all::*, handle::*, sort::_all::*};
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::errors::*;
        pub use super::{
            codec::_all::*,
            iter::_all::*,
            // key::_all::*,
            list::_all::*,
        };
    }
}
