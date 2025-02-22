// devela::work::future
//
//! Asynchronous functionality.
// #![doc = crate::doc_!(modules: crate::work; future)]
// #![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: future, task)]
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

#[cfg(feature = "std")]
#[cfg(not(feature = "dep_portable_atomic_util"))]
mod block;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::{coroutine::_all::*, ext::*, reexports::*};

        #[cfg(feature = "std")]
        #[cfg(not(feature = "dep_portable_atomic_util"))]
        pub use super::block::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{coroutine::_always::*, reexports::*};
    }
}
