// devela::work::sync
//
//! Synchronization, extends `std::`[`sync`].
//!
//! [`sync`]: std::sync
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "work")]
pub mod atomic;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "work")]
pub use atomic::*;

#[cfg(feature = "work")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::atomic::*;
}
