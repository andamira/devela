// devela::mem::slice
//
//! Slices, extends `std::`[`slice`].
//!
//! [`slice`]: std::slice
//

/* modules */

// always compiled, non-public
mod always_fns;

// feature-gated, non-public
#[cfg(feature = "mem")]
mod ext;
#[cfg(feature = "mem")]
mod wrapper;

/* re-exported */

// always compiled, non-public
pub use always_fns::*;

// feature-gated, non-public
#[allow(unused_imports)]
#[cfg(feature = "mem")]
pub use {ext::*, wrapper::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::always_fns::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::{ext::*, wrapper::*};
}
