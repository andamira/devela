// devela::color
//
//! Chromatic functionality.
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "color")]
mod error;
#[cfg(feature = "color")]
mod fns;

#[cfg(feature = "color")]
mod r#trait;

// re-export private sub-modules
#[cfg(feature = "color")]
pub use {error::*, fns::*, r#trait::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "color")]
    pub use super::{error::*, fns::*, r#trait::*};
}
