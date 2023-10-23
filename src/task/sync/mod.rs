// devela::task::sync
//
//! Synchronization, extends
//! `std::`[`sync`][std::sync].
//

/* always compiled for internal use */

/* only compiled with the `convert` feature */

#[cfg(feature = "task")]
pub mod atomic;

/* re-exports */

#[doc(no_inline)]
#[cfg(feature = "task")]
pub use atomic::*;

#[cfg(feature = "task")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::atomic::*;
}
