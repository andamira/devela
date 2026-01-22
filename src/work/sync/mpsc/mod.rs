// devela::work::sync::mpsc
//
#![doc = crate::_DOC_WORK_SYNC_MPSC!()]
// #![doc = crate::_doc!(extends: mpsc)] // IMPROVE

#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /src/base/std/src/work/sync/mpsc/_reexport.rs

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
