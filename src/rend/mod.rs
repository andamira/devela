// devela::rend
//
//! Renderizable multimedia: audio, color, font, image, music, video.
//

// warnings:
#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_rend", forbid(unsafe_code))]

mod error;
pub use error::*;

#[cfg(feature = "rend")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend")))]
pub mod color;
#[cfg(feature = "rend")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "rend")))]
pub mod image;

#[doc(no_inline)]
#[cfg(feature = "rend")]
pub use {color::all::*, image::all::*};

pub(crate) mod all {
    pub use super::error::*;

    #[doc(inline)]
    #[cfg(feature = "rend")]
    pub use super::{color::all::*, image::all::*};
}
