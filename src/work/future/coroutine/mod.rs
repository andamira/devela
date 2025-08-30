// devela::work::future::coroutine
//
//! Coroutine implementations.
//

mod coro; // CoroManager, CoroWork, CoroWorker
#[cfg(nightly_coro)]
#[cfg_attr(nightly_doc, doc(cfg(nightly_coro)))]
mod reexports;

// #[cfg(test)]
// #[cfg(feature = "alloc")]
// mod tests;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::coro::*;

        #[cfg(nightly_coro)]
        #[cfg_attr(nightly_doc, doc(cfg(nightly_coro)))]
        pub use super::reexports::*;
    }
    _always {
        #[cfg(nightly_coro)]
        pub use super::reexports::*;
    }
}
