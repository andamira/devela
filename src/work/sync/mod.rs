// devela::work::sync
//
//! Synchronization primitives.
// #![doc = crate::doc_!(modules: crate::work; sync)]
// #![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: sync)]
//

mod atomic;

#[cfg(feature = "alloc")]
mod reexports;

#[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_sync")))]
mod spin_lock; // SpinLock, SpinLockGuard

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::atomic::*;
        #[cfg(feature = "alloc")]
        pub use super::reexports::*;

        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
        pub use super::spin_lock::*;

        // WIPZONE
        // pub use super::{
        //     counter::*,
        // };
    }
    pub(super) mod _all { #[allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::atomic::*;
        #[cfg(feature = "alloc")]
        pub use super::reexports::*;
    }
}
// WIPZONE
// mod counter;
