// devela::gfx
//
//! Graphics rendering, chromacity and image manipulation.
//

// warnings:
#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_gfx", forbid(unsafe_code))]

/* always-compiled, non-public modules */

mod error;

pub use error::*;

/* feature-gated, public modules */

#[cfg(feature = "gfx")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "gfx")))]
pub mod color;
#[cfg(feature = "gfx")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "gfx")))]
pub mod img;

// feature-gated, public
#[doc(no_inline)]
#[cfg(feature = "gfx")]
pub use {color::all::*, img::all::*};

pub(crate) mod all {
    // always-compiled
    pub use super::error::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "gfx")]
    pub use super::{color::*, img::*};
}
