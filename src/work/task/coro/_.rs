// devela/src/work/task/coro/_.rs
//
#![doc = crate::_DOC_WORK_TASK_CORO!()] // public
#![doc = crate::_doc!(modules: crate::work::task; coro)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod _reexport_core;

    // #[cfg(test)]
    // #[cfg(feature = "alloc")]
    // mod tests;

    mod future; // CoroManager, CoroWork, CoroWorker (IMPROVE do not depend on alloc)
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::future::*;
    }
    _reexports {
        pub use {
            super::_reexport_core::*,
        };
    }
}
