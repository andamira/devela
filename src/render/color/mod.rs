// devela::render::color
//
//! Chromatic functionality.
//

/* modules */

// feature-gated, non-public
#[cfg(feature = "render")]
mod error;
#[cfg(feature = "render")]
mod fns;
#[cfg(feature = "render")]
mod r#trait;

/* re-exports */

// feature-gated, non-public
#[cfg(feature = "render")]
pub use {error::*, fns::*, r#trait::*};

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "render")]
    pub use super::{error::*, fns::*, r#trait::*};
}
