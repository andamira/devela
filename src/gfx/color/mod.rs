// devela::gfx::color
//
//! Chromatic functionality.
//

/* feature-gated, non-public modules */

#[cfg(feature = "gfx")]
mod error;
#[cfg(feature = "gfx")]
mod fns;
#[cfg(feature = "gfx")]
mod r#trait;

#[cfg(feature = "gfx")]
pub use {error::*, fns::*, r#trait::*};

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "gfx")]
    pub use super::{error::*, fns::*, r#trait::*};
}
