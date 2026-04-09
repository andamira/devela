// devela::data::iter
//
#![doc = crate::_DOC_DATA_ACCESS_ITER!()] // public
#![doc = crate::_doc!(modules: crate::data::access; iter)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_core;

mod lending; // IteratorLending[DoubleEnded|ExactSize|Peek]
mod namespace; // Iter
mod strided; // StridedIter, StridedIterMut

crate::structural_mods! { // _mods, _reexports
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
