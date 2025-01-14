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

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        #[cfg(feature = "nightly_coro")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
        pub use super::reexports::*;
        #[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
        pub use super::coro::*;
    }
    pub(super) mod _all { #[allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "nightly_coro")]
        pub use super::reexports::*;
    }
}
