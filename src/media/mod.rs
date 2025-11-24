// devela::media
//
#![doc = crate::_DOC_MEDIA!()]
#![doc = crate::_doc!(modules: crate; media: audio, color, draw, font, image, video)]
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(feature = "safe_media", forbid(unsafe_code))]

#[cfg(feature = "audio")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "audio")))]
pub mod audio;
#[cfg(feature = "color")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "color")))]
pub mod color;
#[cfg(feature = "draw")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "draw")))]
pub mod draw;
#[cfg(feature = "font")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "font")))]
pub mod font;
#[cfg(feature = "image")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
pub mod image;
#[cfg(feature = "midi")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "midi")))]
pub mod midi;
#[cfg(feature = "video")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "video")))]
pub mod video;

crate::structural_mods! { // _pub_mods, _internals
    _pub_mods {
        #[cfg(feature = "audio")] pub use super::audio::_all::*;
        #[cfg(feature = "color")] pub use super::color::_all::*;
        #[cfg(feature = "draw")]  pub use super::draw::_all::*;
        #[cfg(feature = "font")]  pub use super::font::_all::*;
        #[cfg(feature = "image")] pub use super::image::_all::*;
        #[cfg(feature = "midi")]  pub use super::midi::_all::*;
        #[cfg(feature = "video")] pub use super::video::_all::*;
    }
    _crate_internals {
        #[cfg(feature = "color")] pub use super::color::_crate_internals::*;
    }
    _hidden {
        #[cfg(feature = "image")] pub use super::image::_hidden::*;
    }
}
