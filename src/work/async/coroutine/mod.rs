// devela::work::async::coroutine
//
//! Coroutine implementations.
//

// NOTE: it depends on unsafe_async because of task_waker_noop
#[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_async")))]
mod coro;

#[cfg(feature = "nightly_coro")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
mod reexports;

// structural access
crate::items! {
    mod doc_inline {
        #[cfg(feature = "nightly_coro")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
        pub use super::reexports::*;
        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
        pub use super::coro::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::doc_inline::*;
    }
}
