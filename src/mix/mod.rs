// devela::mix
//
//! Mixed media types and functionality, audio, color, image, midi, video.
//

// warnings:
#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_mix", forbid(unsafe_code))]

/* always-compiled */

mod error;
pub use error::*;

/* feature-gated */
#[cfg(feature = "mix")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "mix")))]
pub mod color;
#[cfg(feature = "mix")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "mix")))]
pub mod image;

#[doc(no_inline)]
#[cfg(feature = "mix")]
pub use {color::all::*, image::all::*};

pub(crate) mod all {
    // always-compiled
    pub use super::error::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "mix")]
    pub use super::{color::all::*, image::all::*};
}
