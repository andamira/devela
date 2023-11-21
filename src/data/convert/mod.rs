// devela::data::convert
//
//! Conversion, extends
//! `std::`[`convert`][std::convert].
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "data")]
pub mod collection;
#[cfg(feature = "data")]
pub mod primitive;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "data")]
pub use {collection::*, primitive::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{collection::*, primitive::*};
}
