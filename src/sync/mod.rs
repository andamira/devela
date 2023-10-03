// devela::sync
//
//! Synchronization, extends [`core::sync`].
//

/* always compiled for internal use */

/* only compiled with the `convert` feature */

#[cfg(feature = "sync")]
pub mod atomic;

/* re-exports */

#[cfg(feature = "sync")]
pub use all::*;
#[cfg(feature = "sync")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::atomic::*;
}
