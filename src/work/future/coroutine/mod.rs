// devela::work::future::coroutine
//
#![doc = crate::_DOC_WORK_FUTURE_COROUTINE!()] // public
#![doc = crate::_doc!(modules: crate::work::future; coroutine)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/work/future/coroutine/_reexport.rs

// #[cfg(test)]
// #[cfg(feature = "alloc")]
// mod tests;

mod coro; // CoroManager, CoroWork, CoroWorker

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::coro::*;
    }
    _reexports {
        pub use {
            super::_reexport_core::*,
            // devela_base_core::work::future::{CoroWorker}, // TODO
        };
    }
}
