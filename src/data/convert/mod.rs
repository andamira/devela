// devela::data::convert
//
//! Conversion, extends `std::`[`convert`].
//!
//! [`convert`]: std::convert
//

/* contains always compiled items */

mod cast;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub use cast::*;

/* feature-gated */

#[cfg(feature = "data")]
mod collection;

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {cast::*, collection::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::cast::*;

    #[doc(inline)]
    #[cfg(feature = "data")]
    pub use super::collection::*;
}
