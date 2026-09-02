// devela/src/work/sync/_.rs
//
#![doc = crate::_DOC_WORK_SYNC!()] // public
#![doc = crate::_doc!(modules: crate::work; sync: atomic, mpsc)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//

crate::mods_in! {
    #[cfg(feature = "alloc")]
    mod _reexport_alloc;
    #[cfg(feature = "std")]
    mod _reexport_std;

    #[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_sync")))]
    mod spin_lock; // SpinLock, SpinLockGuard

    pub mod_ atomic; // core::sync::atomic::*
    pub mod_ mpsc; // Mpsc, std::sync::mpsc::*
    // mod counter; // WIP
    // mod queue; // WIP
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_sync"))]
        pub use super::spin_lock::*;
        // pub use super::counter::*;
        // pub use super::queue::*;
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
