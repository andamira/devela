// devela::data::any
//
//! Dynamic typing and reflection, extends
//! `std::`[`any`][std::any].
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

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{ext::*, reexports::*};
}
