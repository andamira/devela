// devela_base_std::data
//
#![doc = crate::_DOC_DATA!()]
//
// safety
#![cfg_attr(all(feature = "base_safe", feature = "safe_data"), forbid(unsafe_code))]

pub mod codec;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            codec::_all::*,
        };
    }
}
