// devela::sync
//
//! Synchronization, extends `std::`[`sync`][std::sync].
//

/* always compiled for internal use */

/* only compiled with the `convert` feature */

#[cfg(feature = "sync")]
pub mod atomic;

/* re-exports */

#[doc(no_inline)]
#[cfg(feature = "sync")]
pub use atomic::*;

#[cfg(feature = "sync")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::atomic::*;
}
