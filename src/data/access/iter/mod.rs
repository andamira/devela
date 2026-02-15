// devela::data::iter
//
#![doc = crate::_DOC_DATA_ACCESS_ITER!()] // public
#![doc = crate::_doc!(modules: crate::data::access; iter)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/data/access/iter/_reexport.rs

mod namespace; // Iter

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        pub use devela_base_core::data::access::iter::{
            IteratorLending, IteratorLendingDoubleEnded, IteratorLendingExactSize,
            IteratorLendingPeek,
        };
        pub use devela_base_core::sys::mem::{SliceIter, SliceIterMut};
    }
}
