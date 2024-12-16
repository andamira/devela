// devela::media
//
//! Multimedia functionality.
#![doc = crate::doc_!(modules: crate; media: audio, color, draw, font, image)]
#![doc = crate::doc_!(newline)]
//
// safety
#![cfg_attr(feature = "safe_media", forbid(unsafe_code))]

#[cfg(_media_·)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(_media_·)))]
mod error;
#[cfg(feature = "audio")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "audio")))]
pub mod audio;
#[cfg(feature = "color")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "color")))]
pub mod color;
#[cfg(feature = "draw")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "draw")))]
pub mod draw;
#[cfg(feature = "font")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "font")))]
pub mod font;
#[cfg(feature = "image")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "image")))]
pub mod image;
#[cfg(feature = "midi")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "midi")))]
pub mod midi;

crate::items! { // structural access: _pub_mods, _all, _always
    #[allow(unused)]
    pub use _pub_mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _pub_mods {
        #[cfg(_media_·)] pub use super::error::*;
        #[cfg(feature = "audio")] pub use super::audio::_all::*;
        #[cfg(feature = "color")] pub use super::color::_all::*;
        #[cfg(feature = "draw")]  pub use super::draw::_all::*;
        #[cfg(feature = "font")]  pub use super::font::_all::*;
        #[cfg(feature = "image")] pub use super::image::_all::*;
        #[cfg(feature = "midi")]  pub use super::midi::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)] pub use super::_pub_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "audio")] pub use super::audio::_always::*;
        #[cfg(feature = "color")] pub use super::color::_always::*;
        #[cfg(feature = "draw")]  pub use super::draw::_always::*;
        #[cfg(feature = "font")]  pub use super::font::_always::*;
        #[cfg(feature = "image")] pub use super::image::_always::*;
        #[cfg(feature = "midi")]  pub use super::midi::_always::*;
    }
}
