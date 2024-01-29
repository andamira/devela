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
#[cfg(all(
    not(feature = "safe_work"),
    feature = "unsafe_async",
    feature = "alloc"
))]
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(all(feature = "unsafe_async", feature = "alloc")))
)]
mod lite;

/* re-exports */

// feature-gated, non-public
#[cfg(feature = "nightly_coro")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
pub use reexports::*;
//
#[cfg(all(
    not(feature = "safe_work"),
    feature = "unsafe_async",
    feature = "alloc"
))]
pub use lite::*;

pub(crate) mod all {
    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "nightly_coro")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "nightly_coro")))]
    pub use super::reexports::*;
    //
    #[doc(inline)]
    #[cfg(all(
        not(feature = "safe_work"),
        feature = "unsafe_async",
        feature = "alloc"
    ))]
    pub use super::lite::*;
}
