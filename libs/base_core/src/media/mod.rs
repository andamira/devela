// devela_base_core::media
//
#![doc = crate::_DOC_MEDIA!()]
//
// safety
#![cfg_attr(base_safe_media, forbid(unsafe_code))]

// pub mod color;
#[cfg(feature = "image")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
pub mod image;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        // pub use super::color::_all::*;

        #[cfg(feature = "image")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
        pub use super::image::_all::*;
    }
}
