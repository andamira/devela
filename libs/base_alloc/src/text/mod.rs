// devela_base_alloc::text
//
#![doc = crate::_DOC_TEXT!()]
//
// safety
#![cfg_attr(base_safe_text, forbid(unsafe_code))]

mod grapheme; // GraphemeString

pub mod fmt;
pub mod str;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::grapheme::GraphemeString;
    }
    _pub_mods {
        pub use super::{
            fmt::_all::*,
            str::_all::*,
        };
    }
}
