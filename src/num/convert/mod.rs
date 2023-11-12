// devela::num::convert
//
//! Conversion, extends
//! `std::`[`convert`][std::convert].
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "num")]
pub mod collection;
#[cfg(feature = "num")]
pub mod primitive;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "num")]
pub use {collection::*, primitive::*};

#[cfg(feature = "num")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{collection::*, primitive::*};
}
