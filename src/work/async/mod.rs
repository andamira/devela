// devela::work::async
//
//! Asynchrony.
#![doc = crate::code::doc_extends!(future, task)]
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
#[allow(unused_imports)]
pub use {coroutine::*, ext::*, reexports::*, waker::*};

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
crate::items! {
    mod block;
    #[allow(unused_imports)]
    pub use block::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{coroutine::*, ext::*, reexports::*, waker::*};

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::block::*;
}
