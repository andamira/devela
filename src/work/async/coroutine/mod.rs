// devela::work::async::coroutine
//
//! Coroutine implementations.
//

/* modules */

// feature-gated, non-public
#[cfg(feature = "nightly_coro")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
mod reexports;
//
// NOTE: it depends on unsafe_async because of TaskWakerNoop
#[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_async")))]
mod coro;

/* re-exports */

// feature-gated, non-public
#[allow(unused_imports)]
#[cfg(feature = "nightly_coro")]
pub use reexports::*;
//
#[cfg(all(not(feature = "safe_work"), feature = "unsafe_async"))]
pub use coro::*;

pub(crate) mod all {
    // feature-gated
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
