// devela::rend
//
//! Renderizable multimedia: audio, color, font, image, music, video.
//

// warnings:
#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_rend", forbid(unsafe_code))]

/* always-compiled */

mod error;
pub use error::*;

/* feature-gated */
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
    // always-compiled
    pub use super::error::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "rend")]
    pub use super::{color::all::*, image::all::*};
}
