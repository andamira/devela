// devela::data::slice
//
//! Slice, extends `std::`[`slice`][std::slice].
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use always_fns::*;

/* only compiled with the `char` feature */

#[cfg(feature = "data")]
mod ext;
#[cfg(feature = "data")]
mod fns;

/* re-exports */

#[cfg(feature = "data")]
pub use all::*;
#[cfg(feature = "data")]
pub(crate) mod all {
    pub use super::{always_fns::*, ext::*, fns::*};
}
