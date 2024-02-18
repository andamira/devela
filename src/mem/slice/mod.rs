// devela::mem::slice
//
//! Slices, extends `std::`[`slice`].
//!
//! [`slice`]: std::slice
//

/* always compiled, non-public modules */

mod always_fns;
mod ext;
mod wrapper;

pub use {always_fns::*, ext::*, wrapper::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::{always_fns::*, ext::*, wrapper::*};
}
