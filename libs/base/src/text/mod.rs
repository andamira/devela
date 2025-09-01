// devela_base::text
//
#![doc = crate::_DOC_TEXT!()]
//
// safety
#![cfg_attr(all(feature = "base_safe", feature = "safe_text"), forbid(unsafe_code))]

pub mod errors;
pub mod fmt;
pub mod str;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[doc(inline)]
        pub use super::errors::*;
        pub use super::{
            fmt::_all::*,
            str::_all::*,
        };
    }
}
