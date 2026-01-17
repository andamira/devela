// devela::work
//
#![doc = crate::_DOC_WORK!()]
#![doc = crate::_DOC_WORK_MODULES!()]
#![doc = crate::_doc!(flat:"work")]
#![doc = "<br/><hr>"] // gives way to zall
//!
#![doc = crate::_doc!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(feature = "safe_work", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_WORK_MODULES =
    crate::_doc!(modules: crate; work: future, process, sync, thread);
}

// pub mod actor;
// pub mod fiber;
pub mod future;
pub mod process;
pub mod sync;
pub mod thread;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            // actor::*,
            // fiber::*,
            future::_all::*,
            process::_all::*,
            sync::_all::*,
            thread::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_WORK_MODULES;
    }
}
