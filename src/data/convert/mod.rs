// devela::data::convert
//
//! Conversion, extends
//! `std::`[`convert`][std::convert].
//

/* contains always compiled items */

pub mod primitive;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub(crate) use primitive::*;

/* feature-gated */

#[cfg(feature = "data")]
pub mod collection;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "data")]
pub use {collection::*, primitive::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{collection::*, primitive::*};
}
