// devela::exec::sync
//
//! Synchronization primitives.
#![doc = crate::code::doc_extends!(sync)]
//!
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(feature = "exec")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "exec")))]
mod atomic;
#[allow(unused_imports)]
#[cfg(feature = "exec")]
pub use atomic::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[doc(inline)]
    #[cfg(feature = "exec")]
    pub use super::atomic::*;
}
