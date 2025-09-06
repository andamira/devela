// devela::work::future::coroutine
//
//! Coroutine implementations.
//

mod coro; // CoroManager, CoroWork, CoroWorker

// #[cfg(test)]
// #[cfg(feature = "alloc")]
// mod tests;

// re-exports
#[cfg(nightly_coro)]
crate::mod_path!(_c "../../../../libs/base_core/src/work/future/coroutine/reexports.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use super::coro::*;

        // re-exports
        #[cfg(nightly_coro)]
        #[cfg_attr(nightly_doc, doc(cfg(nightly_coro)))]
        pub use super::_c::*;
    }
}
