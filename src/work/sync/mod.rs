// devela::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()]
// #![doc = crate::_doc!(modules: crate::work; sync)]
// #![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: sync)]
//

mod reexports;

#[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_sync")))]
mod spin_lock; // SpinLock, SpinLockGuard

pub mod atomic; // core::sync::atomic::*
pub mod mpsc; // Mpsc, std::sync::mpsc::*

// WIPZONE
// mod counter;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::reexports::*;

        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
        pub use super::spin_lock::*;

        // WIPZONE
        // pub use super::counter::*;
    }
    _pub_mods {
        pub use super::{atomic::_all::*, mpsc::_all::*};
    }
}
