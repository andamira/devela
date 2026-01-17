// devela_base_core::work
//
#![doc = crate::_DOC_WORK!()]
#![doc = crate::_DOC_WORK_MODULES!()]
#![doc = crate::_doc!(flat:"work")]
#![doc = "<br/><hr>"] // gives way to zall
//!
#![doc = crate::_doc!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(base_safe_work, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_WORK_MODULES =
    crate::_doc!(modules: crate; work: future, sync); // process, thread
}

pub mod future; // Coro*
pub mod process; // cmd!
pub mod sync;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            future::_all::*,
            process::_all::*,
            sync::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_WORK_MODULES;
    }
}
