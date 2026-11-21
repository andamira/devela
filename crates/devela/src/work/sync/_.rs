//
#![doc = crate::_DOC_WORK_SYNC!()] // public
#![doc = crate::_doc!(modules: crate::work; sync: atomic, mpsc)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//

crate::mods_in! {
    #[cfg(all(
        feature = "sync",
        feature = "unsafe_sync",
        not(feature = "safe_work"),
        any(feature = "dep_portable_atomic", target_has_atomic = "8"),
    ))]
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "sync", feature = "unsafe_sync"))))]
    mod spin_lock; // SpinLock, SpinLockGuard

    pub mod_ atomic; // core::sync::atomic::*
    #[cfg(feature = "sync")]
    pub mod_ mpsc; // Mpsc, std::sync::mpsc::*
    // mod counter; // WIP
    // mod queue; // WIP
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        #[cfg(all(
            feature = "sync",
            feature = "unsafe_sync",
            not(feature = "safe_work"),
            any(feature = "dep_portable_atomic", target_has_atomic = "8"),
        ))]
        pub use super::spin_lock::*;
        // pub use super::counter::*;
        // pub use super::queue::*;
    }
    _pub_mods {
        pub use super::atomic::_all::*;
        #[cfg(feature = "sync")]
        pub use super::mpsc::_all::*;
    }
    _reexports {
        pub use crate::boot::rust::work::sync::*;
    }
}
