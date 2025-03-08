// devela::media
//
//! Multimedia functionality.
#![doc = crate::doc_!(modules: crate; media: audio, color, draw, font, image, video)]
#![doc = crate::doc_!(newline)]
//
// safety
#![cfg_attr(feature = "safe_media", forbid(unsafe_code))]

#[cfg(media··)]
#[cfg_attr(nightly_doc, doc(cfg(media··)))]
mod error;
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

crate::items! { // structural access: _pub_mods, _all, _always
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _pub_mods { #![allow(unused)]
        #[cfg(media··)] pub use super::error::*;
        #[cfg(feature = "audio")] pub use super::audio::_all::*;
        #[cfg(feature = "color")] pub use super::color::_all::*;
        #[cfg(feature = "draw")]  pub use super::draw::_all::*;
        #[cfg(feature = "font")]  pub use super::font::_all::*;
        #[cfg(feature = "image")] pub use super::image::_all::*;
        #[cfg(feature = "midi")]  pub use super::midi::_all::*;
        #[cfg(feature = "video")] pub use super::video::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "audio")] pub use super::audio::_always::*;
        #[cfg(feature = "color")] pub use super::color::_always::*;
        #[cfg(feature = "draw")]  pub use super::draw::_always::*;
        #[cfg(feature = "font")]  pub use super::font::_always::*;
        #[cfg(feature = "image")] pub use super::image::_always::*;
        #[cfg(feature = "midi")]  pub use super::midi::_always::*;
        #[cfg(feature = "video")] pub use super::video::_always::*;
    }
}
