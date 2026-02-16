// devela_base_alloc::data::layout::array
//
#![doc = crate::_DOC_DATA_LAYOUT_ARRAY!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; array)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, vec)]
//

mod vec;

crate::structural_mods! { // _mods
    _mods {
        pub use super::vec::*;
    }
}
