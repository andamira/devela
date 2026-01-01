// devela_base_core::data::iter
//
#![doc = crate::_DOC_DATA_ITER!()]
//

mod _reexport;

mod lending; // IteratorLending[DoubleEnded|ExactSize|Peek]

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            lending::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
        #[doc(inline)]
        pub use crate::sys::mem::{SliceIter, SliceIterMut};
    }
}
