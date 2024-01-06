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

// re-export private sub-modules
#[cfg(feature = "data")]
pub use cast::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::cast::*;
}
