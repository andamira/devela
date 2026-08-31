// devela/src/data/access/iter/_.rs
//
#![doc = crate::_DOC_DATA_ACCESS_ITER!()] // public
#![doc = crate::_doc!(modules: crate::data::access; iter)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! devela regularizes the standard iterator family around two shared prefixes.
//!
//! Iterator traits use the `Iterator*` family where practical:
//! `DoubleEndedIterator` becomes [`IteratorDoubleEnded`],
//! `ExactSizeIterator` becomes [`IteratorExactSize`],
//! `IntoIterator` becomes [`IteratorInto`], and so on.
//! [`Iterator`] itself retains its conventional name.
//!
//! The shorter `Iter*` prefix is used for
//! iterator namespace operations and related constructors.
//!
//! [`IteratorLending`] extends this vocabulary to
//! iteration whose yielded values borrow from the iterator itself.
//

crate::mods_in! {
    mod _reexport_core;

    mod_ lending; // IteratorLending[DoubleEnded|ExactSize|Peek]
    mod namespace; // Iter
    mod_ strided; // StridedIter, StridedIterMut
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            lending::_all::*,
            namespace::*,
            strided::_all::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;

        pub use crate::sys::mem::{SliceIter, SliceIterMut};
    }
}
