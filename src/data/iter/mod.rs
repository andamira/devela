// devela::data::iter
//
#![doc = crate::_DOC_DATA_ITER!()]
//!
#![doc = crate::_doc!(extends: iter)]
//

mod namespace;
mod reexports;

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
        pub use super::reexports::*;
    }
}
