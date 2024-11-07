// devela::rend
//
//! Rendering multi-media functionality.
#![doc = crate::doc_!(modules: crate; rend: audio, color, draw, font, image, layout)]
#![doc = crate::doc_!(newline)]
//
// safety:
#![cfg_attr(feature = "safe_rend", forbid(unsafe_code))]

#[cfg(_rend_·)]
use crate::code::items;

#[cfg(_rend_·)]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(_rend_·)))]
    mod error;
    #[doc(inline)]
    pub use error::*;
}

#[cfg(feature = "rend_audio")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_audio")))]
    pub mod audio;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use audio::all::*;
}
#[cfg(feature = "rend_color")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_color")))]
    pub mod color;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use color::all::*;
}
#[cfg(feature = "rend_draw")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_draw")))]
    pub mod draw;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use draw::all::*;
}
#[cfg(feature = "rend_font")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_font")))]
    pub mod font;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use font::all::*;
}
#[cfg(feature = "rend_image")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_image")))]
    pub mod image;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use image::all::*;
}
#[cfg(feature = "rend_layout")]
items! {
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_layout")))]
    pub mod layout;
    #[doc(no_inline)]
    #[allow(unused_imports)]
    pub use layout::all::*;
}

pub(crate) mod all {
    #![allow(unused_imports)]

    #[doc(inline)]
    #[cfg(_rend_·)]
    pub use super::error::*;

    #[doc(inline)]
    #[cfg(feature = "rend_audio")]
    pub use super::audio::all::*;
    #[doc(inline)]
    #[cfg(feature = "rend_color")]
    pub use super::color::all::*;
    #[doc(inline)]
    #[cfg(feature = "rend_draw")]
    pub use super::draw::all::*;
    #[doc(inline)]
    #[cfg(feature = "rend_font")]
    pub use super::font::all::*;
    #[doc(inline)]
    #[cfg(feature = "rend_image")]
    pub use super::image::all::*;
    #[doc(inline)]
    #[cfg(feature = "rend_layout")]
    pub use super::layout::all::*;
}
