// devela::time
//
//! Temporal quantification, extends
//! `std::`[`time`][std::time].
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "time")]
mod reexports;

// re-exports private sub-modules
#[cfg(feature = "time")]
pub use reexports::*;

#[cfg(feature = "time")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;
}
