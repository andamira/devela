// devela_base_core::ui
//
#![doc = crate::_DOC_UI!()]
//
// safety
#![cfg_attr(base_safe_ui, forbid(unsafe_code))]

pub mod front;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            front::_all::*,
        };
    }
}
