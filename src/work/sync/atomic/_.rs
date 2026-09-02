// devela/src/work/sync/atomic/_.rs
//
#![doc = crate::_DOC_WORK_SYNC_ATOMIC!()] // public
#![doc = crate::_doc!(modules: crate::work::sync; atomic)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//
// IMPROVE: move _reexport_dep to base_core

crate::mods_in! {
    mod _reexport_core;
    mod _reexport_dep; // from dep_atomic, dep_portable_atomic (and impls ConstInit)
}
crate::mods_out! { // _reexports
    _reexports {
        pub use super::{
            _reexport_core::*,
            _reexport_dep::*,
        };
    }
}
