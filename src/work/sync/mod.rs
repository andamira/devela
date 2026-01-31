// devela::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()] // public
#![doc = crate::_doc!(modules: crate::work; sync)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//

#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /src/base/alloc/src/work/sync/_reexport.rs
#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /src/base/std/src/work/sync/_reexport.rs

#[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_sync")))]
mod spin_lock; // SpinLock, SpinLockGuard

pub mod atomic; // core::sync::atomic::*
pub mod mpsc; // Mpsc, std::sync::mpsc::*
// mod counter;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
        pub use super::spin_lock::*;
        // pub use super::counter::*;
    }
    _pub_mods {
        pub use super::{
            atomic::_all::*,
            mpsc::_all::*,
        };
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
