// devela::work::sync
//
//! Synchronization primitives.
#![doc = crate::doc_!(extends: sync)]
#![doc = crate::doc_!(modules: crate::work; sync)]
#![doc = crate::doc_!(newline)]
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
    #![allow(unused_imports)]

    #[doc(inline)]
    pub use super::reexports::*;

    #[doc(inline)]
    #[cfg(feature = "work")]
    pub use super::atomic::*;
}
