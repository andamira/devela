// devela_base_core::data::iter
//
#![doc = crate::_DOC_DATA_ITER!()]
//

mod lending; // IteratorLending[DoubleEnded|ExactSize|Peek]

// re-exports
mod _reexport;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            lending::*,
        };

        // re-exports
        pub use super::_reexport::*;
        // intra-crate
        #[doc(inline)]
        pub use crate::sys::mem::{SliceIter, SliceIterMut};
    }
}
