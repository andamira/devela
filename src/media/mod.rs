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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        #[cfg(_media_·)]
        pub use super::error::*;
        #[cfg(feature = "audio")]
        pub use super::audio::all::*;
        #[cfg(feature = "color")]
        pub use super::color::all::*;
        #[cfg(feature = "draw")]
        pub use super::draw::all::*;
        #[cfg(feature = "font")]
        pub use super::font::all::*;
        #[cfg(feature = "image")]
        pub use super::image::all::*;
    }
    pub(super) mod all { #[doc(inline)] #[allow(unused_imports)]
        pub use super::doc_inline::*;
    }
}
