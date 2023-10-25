// devela::convert
//
//! Conversion, extends
//! `std::`[`convert`][std::convert].
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "convert")]
pub mod collection;
#[cfg(feature = "convert")]
pub mod primitive;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "convert")]
pub use {collection::*, primitive::*};

#[cfg(feature = "convert")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{collection::*, primitive::*};
}
