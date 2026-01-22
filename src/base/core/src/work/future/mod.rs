// devela_base_core::work::future
//
#![doc = crate::_DOC_WORK_FUTURE!()]
//

mod _reexport; // SYMLINK from /src/work/future/_reexport_core.rs

mod coroutine;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            coroutine::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
