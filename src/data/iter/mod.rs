// devela::data::iter
//
#![doc = crate::_DOC_DATA_ITER!()]
//!
#![doc = crate::_doc!(extends: iter)]
//

crate::mod_path!(_c "../../../libs/base_core/src/data/iter/reexports.rs");
mod namespace;

crate::structural_mods! { // _mods
    _mods {
        pub use super::_c::*;
        pub use super::namespace::*;
    }
}
