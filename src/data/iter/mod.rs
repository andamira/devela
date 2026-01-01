// devela::data::iter
//
#![doc = crate::_DOC_DATA_ITER!()]
//!
#![doc = crate::_doc!(extends: iter)]
//

mod namespace; // Iter

// re-exports
crate::mod_path!(_c "../../../libs/base_core/src/data/iter/_reexport.rs");

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;

        // re-exports
        pub use super::_c::*;
        pub use devela_base_core::data::iter::{
            IteratorLending, IteratorLendingDoubleEnded, IteratorLendingExactSize,
            IteratorLendingPeek,
        };
        pub use devela_base_core::sys::mem::{SliceIter, SliceIterMut};
    }
}
