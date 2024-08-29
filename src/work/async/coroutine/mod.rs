// devela::work::async::coroutine
//
//! Coroutine implementations.
//

// NOTE: it depends on unsafe_async because of task_waker_noop
#[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_async")))]
crate::items! {
    mod coro;
    pub use coro::*;
}

#[cfg(feature = "nightly_coro")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
crate::items! {
    mod reexports;
    #[allow(unused_imports)]
    pub use reexports::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(feature = "nightly_coro")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
    pub use super::reexports::*;
    //
    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
    pub use super::coro::*;
}
