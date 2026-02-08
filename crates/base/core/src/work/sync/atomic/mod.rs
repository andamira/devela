// devela_base_core::work::sync::atomic
//
#![doc = crate::_DOC_WORK_SYNC_ATOMIC!()] // public
#![doc = crate::_doc!(modules: crate::work::sync; atomic)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//

mod _reexport; // SYMLINK from /src/work/sync/atomic/_reexport_core.rs

crate::structural_mods! { // _reexports
    _reexports { pub use super::_reexport::*; }
}
