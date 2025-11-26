// devela_base_core::data::iter
//
#![doc = crate::_DOC_DATA_ITER!()]
//

mod lending; // IteratorLending, SliceIter, SliceIterMut
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            lending::*,
            reexports::*,
        };
    }
}
