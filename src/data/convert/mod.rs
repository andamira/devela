// devela::data::convert
//
//! Conversion, extends `std::`[`convert`].
//!
//! [`convert`]: std::convert
//

/* contains always compiled items */

mod cast;
mod collection;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub use {cast::*, collection::*};

/* feature-gated */

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {cast::*, collection::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{cast::*, collection::*};
}
