// devela_base_std::work::thread
//
#![doc = crate::_DOC_WORK_THREAD!()] // public
#![doc = crate::_doc!(modules: crate::work; thread)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: thread)]
//

mod _reexport; // SYMLINK from /src/work/thread/_reexport_std.rs

crate::structural_mods! { // _reexports
    _reexports {
        pub use super::_reexport::*;
    }
}
