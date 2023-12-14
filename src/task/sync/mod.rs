// devela::task::sync
//
//! Synchronization, extends `std::`[`sync`].
//!
//! [`sync`]: std::sync
//

/* contains always compiled items */

// ...

/* feature-gated */

#[cfg(feature = "task")]
pub mod atomic;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "task")]
pub use atomic::*;

#[cfg(feature = "task")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::atomic::*;
}
