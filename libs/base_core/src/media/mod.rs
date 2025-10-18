// devela_base_core::media
//
#![doc = crate::_DOC_MEDIA!()]
//
// safety
#![cfg_attr(base_safe_media, forbid(unsafe_code))]

// pub mod color;
pub mod image;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            // color::_all::*;
            image::_all::*,
        };
    }
}
