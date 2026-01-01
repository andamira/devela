// devela::work::future::coroutine
//
//! Coroutine implementations.
//

// #[cfg(nightly_coro)]
mod _reexport_core; // SYMLINK to /libs/base_core/src/work/future/coroutine/_reexport.rs

mod coro; // CoroManager, CoroWork, CoroWorker

// #[cfg(test)]
// #[cfg(feature = "alloc")]
// mod tests;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::coro::*;
    }
    _reexports {
        // #[cfg(nightly_coro)]
        // #[cfg_attr(nightly_doc, doc(cfg(nightly_coro)))]
        pub use super::_reexport_core::*;
    }
}
