// devela_base_alloc::work
//
#![doc = crate::_DOC_WORK!()] // public, root
#![doc = crate::_DOC_WORK_MODULES!()]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(base_safe_work, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_WORK_MODULES =
    crate::_doc!(modules: crate; work: sync); // future, process, thread
}

pub mod sync;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            sync::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_WORK_MODULES;
    }
}
