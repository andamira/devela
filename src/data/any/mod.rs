// devela::data::any
//
//! Dynamic typing and reflection, extends `std::`[`any`].
//!
//! [`any`]: std::any
//

/* contains always compiled items */

// ...

/* feature-gated */

// private sub-modules
#[cfg(feature = "data")]
mod ext;
#[cfg(feature = "data")]
mod reexports;

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {ext::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "data")]
    pub use super::{ext::*, reexports::*};
}
