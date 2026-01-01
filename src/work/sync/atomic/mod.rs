// devela::work::sync::atomic
//
#![doc = crate::_DOC_WORK_SYNC_ATOMIC!()]
//!
#![doc = crate::_doc!(extends: sync)]
//
// IMPROVE: move _reexport_dep to base_core

mod _reexport_core; // SYMLINK to /libs/base_core/src/work/sync/atomic/_reexport.rs
mod _reexport_dep; // from dep_atomic, dep_portable_atomic (and impls ConstInit)

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::{
            _reexport_core::*,
            _reexport_dep::*,
        };
    }
}
