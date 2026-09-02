// devela/src/work/_.rs
//
#![doc = crate::_DOC_WORK!()] // public
#![doc = crate::_DOC_WORK_MODULES!()]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(feature = "safe_work", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_WORK_MODULES =
    crate::_doc!(modules: crate; work: exec, future, sync, task); // plan
}

crate::mods_in! {
    pub mod_ exec;
    pub mod_ future;
    // pub mod plan;
    pub mod_ sync;
    pub mod_ task;
}
crate::mods_out! { // _pub_mods, _crate_internals, _hidden
    _pub_mods {
        pub use super::{
            exec::_all::*,
            future::_all::*,
            // plan::_all::*,
            sync::_all::*,
            task::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_WORK_MODULES;
        pub(crate) use super::{
            future::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::exec::_hidden::*;
    }
}
