// devela_base_std::work::sync::mpsc
//
#![doc = crate::_DOC_WORK_SYNC_MPSC!()] // public
#![doc = crate::_doc!(modules: crate::work::sync; mpsc)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: sync)]
//

mod _reexport; // SYMLINK from /src/work/sync/mpsc/_reexport_std.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
