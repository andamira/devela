// devela_base_core::work::future::coroutine
//
#![doc = crate::_DOC_WORK_FUTURE_COROUTINE!()] // public
#![doc = crate::_doc!(modules: crate::work::future; coroutine)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(hr)]
//

mod _reexport; // SYMLINK from /src/work/future/coroutine/_reexport_core.rs

// mod coro; // CoroManager, CoroWork, CoroWorker WIP

crate::structural_mods! { // _mods, _reexports
    _mods {
        // pub use super::coro::*; // WIP
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
