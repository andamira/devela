// devela::rend
//
//! Renderizable multimedia: audio, color, font, image, video.
//

// warnings:
#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_rend", forbid(unsafe_code))]

mod error;
pub use error::*;

#[cfg(feature = "rend_color")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_color")))]
pub mod color;
#[cfg(feature = "rend_font")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_font")))]
pub mod font;
#[cfg(feature = "rend_image")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_image")))]
pub mod image;
#[cfg(feature = "rend_video")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend_video")))]
pub mod video;

#[doc(no_inline)]
#[cfg(feature = "rend")]
pub use {color::all::*, image::all::*};

pub(crate) mod all {
    pub use super::error::*;

    #[doc(inline)]
    #[cfg(feature = "rend")]
    pub use super::{color::all::*, image::all::*};
}
