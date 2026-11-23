//
#![doc = crate::_DOC_WORK_FUTURE!()] // public
#![doc = crate::_doc!(modules: crate::work; future)]
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: future, task)]
//!
//! See also the fundamental [`async`] and [`await`] keywords and the
//! [async book](https://rust-lang.github.io/async-book/).
//!
//! [`async`]: https://doc.rust-lang.org/std/keyword.async.html
//! [`await`]: https://doc.rust-lang.org/std/keyword.await.html
//

crate::mods_in! {
    #[cfg(feature = "future")]
    mod ext; // FutureExt

    #[cfg(all(feature = "future", feature = "std"))]
    #[cfg(not(feature = "dep_portable_atomic_util"))]
    mod block;
}
crate::mods_out! { // _mods, _reexports, _crate_internals
    _mods {
        #[cfg(feature = "future")]
        pub use super::{
            ext::FutureExt,
        };
    }
    _reexports {
        pub use crate::boot::rust::work::future::*;
    }
    _crate_internals {
        #[cfg(all(feature = "future", feature = "std"))]
        #[cfg(not(feature = "dep_portable_atomic_util"))]
        pub(crate) use super::block::future_block;
    }
}
