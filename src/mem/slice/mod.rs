// devela::mem::slice
//
//! Slices, extends `std::`[`slice`].
//!
//! [`slice`]: std::slice
//

/* contains always compiled items */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "mem"))]
pub(crate) use always_fns::*;

/* feature-gated */

// private sub-modules
#[cfg(feature = "mem")]
mod ext;
#[cfg(feature = "mem")]
mod fns;

// re-export private sub-modules
#[cfg(feature = "mem")]
pub use {always_fns::*, ext::*, fns::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::{ext::*, fns::*};
}
