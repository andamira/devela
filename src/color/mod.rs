// devela::color
//
//! Colors.
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "color")]
mod fns;

#[cfg(feature = "color")]
mod r#trait;

// ...

// re-export private sub-modules
#[cfg(feature = "color")]
pub use {fns::*, r#trait::*};

#[cfg(feature = "color")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{fns::*, r#trait::*};
}
