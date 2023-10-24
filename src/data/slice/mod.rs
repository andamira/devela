// devela::data::slice
//
//! Slices, extends
//! `std::`[`slice`][std::slice].
//

/* contains always compiled items */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use always_fns::*;

/* feature-gated */

// private sub-modules
#[cfg(feature = "data")]
mod ext;
#[cfg(feature = "data")]
mod fns;

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {always_fns::*, ext::*, fns::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    pub use super::{always_fns::*, ext::*, fns::*};
}
