// devela::work::future::coroutine
//
//! Coroutine implementations.
//

mod _reexport_core; // SYMLINK to /src/base/core/src/work/future/coroutine/_reexport.rs

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
