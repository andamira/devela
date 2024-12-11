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

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        #[cfg(feature = "nightly_coro")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
        pub use super::reexports::*;
        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
        pub use super::coro::*;
    }
    pub(super) mod all { #[allow(unused)]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        #[cfg(feature = "nightly_coro")]
        pub use super::reexports::*;
    }
}
