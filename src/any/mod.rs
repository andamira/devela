// devela::any
//
//! Dynamic typing and reflection, extends
//! `std::`[`any`][std::any].
//

/* contains always compiled items */

// ...

/* feature-gated */

// private sub-modules
#[cfg(feature = "any")]
mod ext;
#[cfg(feature = "any")]
mod reexports;

// re-export private sub-modules
#[cfg(feature = "any")]
pub use {ext::*, reexports::*};

#[cfg(feature = "any")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{ext::*, reexports::*};
}
