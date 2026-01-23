// devela::data::iter
//
#![doc = crate::_DOC_DATA_ITER!()]
//!
#![doc = crate::_doc!(modules: crate::data; iter)]
#![doc = crate::_doc!(flat:"data")]
//!
#![doc = crate::_doc!(extends: iter)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/data/iter/_reexport.rs

mod namespace; // Iter

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        pub use devela_base_core::data::iter::{
            IteratorLending, IteratorLendingDoubleEnded, IteratorLendingExactSize,
            IteratorLendingPeek,
        };
        pub use devela_base_core::sys::mem::{SliceIter, SliceIterMut};
    }
}
