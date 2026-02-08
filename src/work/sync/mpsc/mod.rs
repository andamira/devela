// devela::work::sync::mpsc
//
#![doc = crate::_DOC_WORK_SYNC_MPSC!()] // public
#![doc = crate::_doc!(modules: crate::work::sync; mpsc)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]

#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /crates/base/std/src/work/sync/mpsc/_reexport.rs

mod namespace; // Mpsc

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::*;
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
