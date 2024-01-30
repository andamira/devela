// devela::render
//
//! Graphics rendering and processing.
//

// warnings:
#![allow(unused_imports)]
// safety:
#![cfg_attr(feature = "safe_render", forbid(unsafe_code))]

/* modules */

// feature-gated, public
#[cfg(feature = "render")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "render")))]
pub mod color;
// #[cfg(feature = "render")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "render")))]
// pub mod img;

/* re-exports */

// feature-gated, public
#[doc(no_inline)]
#[cfg(feature = "render")]
pub use color::all::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "render")]
    pub use super::color::*;
}
