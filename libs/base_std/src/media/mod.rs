// devela_base_std::media
//
#![doc = crate::_DOC_MEDIA!()]
#![doc = crate::_doc!(modules: crate; media: color)] // audio, color, draw, font, image, video
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(base_safe_media, forbid(unsafe_code))]
//
#![cfg_attr(doc, allow(rustdoc::broken_intra_doc_links))]

// #[cfg(feature = "audio")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "audio")))]
// pub mod audio;

#[cfg(feature = "color")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "color")))]
pub mod color;

// #[cfg(feature = "draw")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "draw")))]
// pub mod draw;

// #[cfg(feature = "image")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
// pub mod image;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        // #[cfg(feature = "audio")]
        // pub use super::audio::_all::*;

        #[cfg(feature = "color")]
        pub use super::color::_all::*;

        // #[cfg(feature = "draw")]
        // pub use super::draw::_all::*;

        // #[cfg(feature = "image")]
        // #[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
        // pub use super::image::_all::*;
    }
}
