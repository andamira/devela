// devela::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()]
// #![doc = crate::doc_!(modules: crate::work; sync)]
// #![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: sync)]
//

mod reexports;

#[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_sync")))]
mod spin_lock; // SpinLock, SpinLockGuard

pub mod atomic; // core::sync::atomic::*
pub mod mpsc; // Mpsc, std::sync::mpsc::*

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods { #![allow(unused)]
        pub use super::reexports::*;

        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
        pub use super::spin_lock::*;

        // WIPZONE
        // pub use super::counter::*;
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{atomic::_all::*, mpsc::_all::*};
    }
    pub(super) mod _all { #[allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::atomic::_all::*;
        #[cfg(feature = "alloc")]
        pub use super::reexports::*;
    }
}
// WIPZONE
// mod counter;
