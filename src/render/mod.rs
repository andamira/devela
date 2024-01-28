// devela::render
//
//! Graphics rendering and processing.
//

#![allow(unused_imports)]

/* modules */

// feature-gated, public
#[cfg(feature = "render")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "render")))]
pub mod color;

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
