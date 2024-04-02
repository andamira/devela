// devela::exec::sync
//
//! Synchronization, extends `std::`[`sync`].
//!
//! [`sync`]: std::sync
//

/* always compiled */

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

/* feature-gated */

#[cfg(feature = "exec")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
mod atomic;
#[allow(unused_imports)]
#[cfg(feature = "exec")]
pub use atomic::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "exec")]
    pub use super::atomic::*;
}
