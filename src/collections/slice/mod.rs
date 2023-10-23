// devela::collections::slice
//
//! Slice, extends `std::`[`slice`][std::slice].
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "collections"))]
pub(crate) use always_fns::*;

/* only compiled with the `char` feature */

#[cfg(feature = "collections")]
mod ext;
#[cfg(feature = "collections")]
mod fns;

/* re-exports */

#[cfg(feature = "collections")]
pub use all::*;
#[cfg(feature = "collections")]
pub(crate) mod all {
    pub use super::{always_fns::*, ext::*, fns::*};
}
