// devela::ops::convert
//
//! Conversion, extends
//! `std::`[`convert`][std::convert].
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "ops")]
pub mod collection;
#[cfg(feature = "ops")]
pub mod primitive;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "ops")]
pub use {collection::*, primitive::*};

#[cfg(feature = "ops")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{collection::*, primitive::*};
}
