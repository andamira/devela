// devela_base_core::work::future::coroutine
//
//!
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
