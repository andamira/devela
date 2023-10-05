// devela::task
//
//! Asynchronous tasks, extends [`core::task`].
//

/* always compiled for internal use */

/* only compiled with the `task` feature */

#[cfg(all(feature = "task", feature = "alloc"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "char", feature = "alloc")))
)]
pub mod coroutine;

/* re-exports */

#[cfg(feature = "task")]
mod reexports;

#[cfg(feature = "task")]
pub use all::*;
#[cfg(feature = "task")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::coroutine::*;
}
