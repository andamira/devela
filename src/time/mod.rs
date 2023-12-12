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

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "time")]
    pub use super::reexports::*;
}
