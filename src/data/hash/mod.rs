// devela::data::hash
//
//! Generic hashing support, extends `std::`[`hash`].
//!
//! [`hash`]: std::hash
//

/* contains always compiled items */

// ...

/* feature-gated */

// private sub-modules
#[cfg(feature = "data")]
mod reexports;

// re-export private sub-modules
#[allow(unused_imports)]
#[cfg(feature = "data")]
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "data")]
    pub use super::reexports::*;
}
