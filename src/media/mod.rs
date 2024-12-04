// devela::media
//
//! Multimedia functionality.
#![doc = crate::doc_!(modules: crate; media: audio, color, draw, font, image, layout)]
#![doc = crate::doc_!(newline)]
//
// safety
#![cfg_attr(feature = "safe_media", forbid(unsafe_code))]

#[cfg(_media_·)]
crate::items! {
    use crate::items;

    #[cfg_attr(feature = "nightly_doc", doc(cfg(_media_·)))]
    mod error;
    #[doc(inline)]
    pub use error::*;
}

#[cfg(feature = "audio")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "audio")))]
    pub mod audio;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use audio::all::*;
}
#[cfg(feature = "color")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "color")))]
    pub mod color;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use color::all::*;
}
#[cfg(feature = "draw")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "draw")))]
    pub mod draw;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use draw::all::*;
}
#[cfg(feature = "font")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "font")))]
    pub mod font;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use font::all::*;
}
#[cfg(feature = "image")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "image")))]
    pub mod image;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use image::all::*;
}
#[cfg(feature = "layout")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "layout")))]
    pub mod layout;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use layout::all::*;
}

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    #[cfg(_media_·)]
    pub use super::error::*;

    #[doc(inline)]
    #[cfg(feature = "audio")]
    pub use super::audio::all::*;
    #[doc(inline)]
    #[cfg(feature = "color")]
    pub use super::color::all::*;
    #[doc(inline)]
    #[cfg(feature = "draw")]
    pub use super::draw::all::*;
    #[doc(inline)]
    #[cfg(feature = "font")]
    pub use super::font::all::*;
    #[doc(inline)]
    #[cfg(feature = "image")]
    pub use super::image::all::*;
    #[doc(inline)]
    #[cfg(feature = "layout")]
    pub use super::layout::all::*;
}
