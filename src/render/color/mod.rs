// devela::render::color
//
//! Chromatic functionality.
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "render")]
mod error;
#[cfg(feature = "render")]
mod fns;
#[cfg(feature = "render")]
mod r#trait;

// re-export private sub-modules
#[cfg(feature = "render")]
pub use {error::*, fns::*, r#trait::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "render")]
    pub use super::{error::*, fns::*, r#trait::*};
}
