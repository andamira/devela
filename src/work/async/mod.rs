// devela::work::async
//
//! Asynchrony.
#![doc = crate::doc_!(extends: future, task)]
#![doc = crate::doc_!(modules: crate::work; async)]
#![doc = crate::doc_!(newline)]
//!
//! See also the fundamental [`async`] and [`await`] keywords and the
//! [async book](https://rust-lang.github.io/async-book/).
//!
//! [`async`]: https://doc.rust-lang.org/std/keyword.async.html
//! [`await`]: https://doc.rust-lang.org/std/keyword.await.html
//

mod coroutine;
mod ext;
mod reexports;
mod waker;

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
mod block;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{coroutine::*, ext::*, reexports::*, waker::*};
        #[cfg(feature = "std")]
        pub use super::block::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
