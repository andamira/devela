// devela_base_std::work::process
//
#![doc = crate::_DOC_WORK_PROCESS!()] // public
#![doc = crate::_doc!(modules: crate::work; process)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: process)]
//

mod _reexport; // SYMLINK from /src/work/process/_reexport_std.rs

crate::structural_mods! { // _mods, _reexports
    _mods {}
    _reexports {
        pub use super::_reexport::*;
    }
}
