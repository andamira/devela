// devela::mem::slice
//
//! Slices, extends `std::`[`slice`].
//!
//! [`slice`]: std::slice
//

/* always compiled */

mod ext;
mod wrapper;

pub use {ext::*, wrapper::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[cfg(feature = "mem")]
    pub use super::{ext::*, wrapper::*};
}
