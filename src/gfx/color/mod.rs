// devela::gfx::color
//
//! Chromatic functionality.
//

/* modules */

// feature-gated, non-public
#[cfg(feature = "gfx")]
mod error;
#[cfg(feature = "gfx")]
mod fns;
#[cfg(feature = "gfx")]
mod r#trait;

/* re-exports */

// feature-gated, non-public
#[cfg(feature = "gfx")]
pub use {error::*, fns::*, r#trait::*};

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "gfx")]
    pub use super::{error::*, fns::*, r#trait::*};
}
