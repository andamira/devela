// devela_base_core::data::iter
//
#![doc = crate::_DOC_DATA_ITER!()] // public
#![doc = crate::_doc!(modules: crate::data; iter)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod _reexport; // SYMLINK from /src/data/iter/_reexport_core.rs

mod lending; // IteratorLending[DoubleEnded|ExactSize|Peek]

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            lending::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
