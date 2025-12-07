// devela::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()]
//!
#![doc = crate::_doc!(extends: sync)]
//

#[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_sync")))]
mod spin_lock; // SpinLock, SpinLockGuard

pub mod atomic; // core::sync::atomic::*
pub mod mpsc; // Mpsc, std::sync::mpsc::*
// mod counter;

// re-exports
crate::mod_path!(std _s "../../../libs/base_std/src/work/sync/reexports.rs");

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
        pub use super::spin_lock::*;
        // pub use super::counter::*;

        // re-exports
        #[cfg(feature = "alloc")]
        pub use devela_base_alloc::{
            Arc, ArcWeak
        };
        #[cfg(feature = "std")]
        pub use super::_s::*;
    }
    _pub_mods {
        pub use super::{atomic::_all::*, mpsc::_all::*};
    }
}
