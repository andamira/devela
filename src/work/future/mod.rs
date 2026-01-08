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

mod _reexport_core; // SYMLINK to /libs/base_core/src/work/future/_reexport.rs

mod coroutine;
mod ext; // FutureExt

#[cfg(feature = "std")]
#[cfg(not(feature = "dep_portable_atomic_util"))]
mod block;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            coroutine::_all::*,
            ext::*,
        };

        #[cfg(feature = "std")]
        #[cfg(not(feature = "dep_portable_atomic_util"))]
        pub use super::block::*;
    }
    _reexports {
        pub use super::_reexport_core::*;

        /* from either `alloc` or `portable-atomic-util` and `alloc` */

        #[doc = crate::_tags!(concurrency atomic runtime atomic_alloc_portable_util)]
        #[cfg(all(feature = "alloc", feature = "dep_portable_atomic_util"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        /// The implementation of waking a task on an executor.
        #[doc = crate::_doc_location!("work/future")]
        #[doc = "---\n\n---\n\n"]
        pub use crate::_dep::portable_atomic_util::task::Wake as TaskWake;
        //
        #[doc = crate::_tags!(concurrency atomic runtime atomic_alloc_portable_util)]
        #[doc = crate::_doc_location!("work/future")]
        #[doc = "---\n\n---\n\n"]
        #[cfg(all(feature = "alloc", not(feature = "dep_portable_atomic_util")))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
        pub use crate::_dep::_alloc::task::Wake as TaskWake;
    }
}
