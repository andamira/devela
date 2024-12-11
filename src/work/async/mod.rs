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

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline { #![allow(unused)]
        pub use super::{coroutine::all::*, ext::*, reexports::*, waker::*};

        #[cfg(feature = "std")]
        pub use super::block::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{coroutine::always::*, reexports::*};
    }
}
