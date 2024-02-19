// devela::gfx
//
//! Graphics rendering, chromacity and image manipulation.
//

// warnings:
#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_gfx", forbid(unsafe_code))]

/* modules */

// feature-gated, public
#[cfg(feature = "gfx")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "gfx")))]
pub mod color;

/* re-exports */

// feature-gated, public
#[doc(no_inline)]
#[cfg(feature = "gfx")]
pub use color::all::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "gfx")]
    pub use super::color::*;
}
