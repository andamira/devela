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
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        #[cfg(feature = "nightly_coro")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
        pub use super::reexports::*;
        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
        pub use super::coro::*;
    }
    pub(super) mod all { #[doc(inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[cfg(feature = "nightly_coro")] #[doc(hidden)] #[doc(no_inline)]
        pub use super::reexports::*;
    }
}
