// devela::work::sync
//
//! Synchronization primitives.
#![doc = crate::code::doc_extends!(sync)]
//!
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

#[cfg(feature = "work")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "work")))]
crate::items! {
    mod atomic;
    #[allow(unused_imports)]
    pub use atomic::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;

    #[doc(inline)]
    #[cfg(feature = "work")]
    pub use super::atomic::*;
}
