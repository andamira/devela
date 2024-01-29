// devela::data::any
//
//! Dynamic typing and reflection, extends `std::`[`any`].
//!
//! [`any`]: std::any
//

/* modules */

// feature-gated, non-public
#[cfg(feature = "data")]
mod ext;
#[cfg(feature = "data")]
mod reexports;

/* re-exports */

// feature-gated, non-public
#[cfg(feature = "data")]
pub use {ext::*, reexports::*};

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "data")]
    pub use super::{ext::*, reexports::*};
}
