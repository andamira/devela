// devela_base_alloc::text
//
#![doc = crate::_DOC_TEXT!()]
//

pub mod fmt;
pub mod str;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            fmt::_all::*,
            str::_all::*,
        };
    }
}
