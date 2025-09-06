// devela::work::future
//
#![doc = crate::_DOC_WORK_FUTURE!()]
// #![doc = crate::_doc!(modules: crate::work; future)]
// #![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: future, task)]
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

crate::structural_mods! { // _mods
    _mods {
        pub use super::{coroutine::_all::*, ext::*, reexports::*};

        #[cfg(feature = "std")]
        #[cfg(not(feature = "dep_portable_atomic_util"))]
        pub use super::block::*;
    }
}
