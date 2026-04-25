// devela::work::task::coro
//
#![doc = crate::_DOC_WORK_TASK_CORO!()] // public
#![doc = crate::_doc!(modules: crate::work::task; coro)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_core;

// #[cfg(test)]
// #[cfg(feature = "alloc")]
// mod tests;

mod future; // CoroManager, CoroWork, CoroWorker (IMPROVE do not depend on alloc)

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::future::*;
    }
    _reexports {
        pub use {
            super::_reexport_core::*,
        };
    }
}
