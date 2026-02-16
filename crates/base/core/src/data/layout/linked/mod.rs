// devela_base_core::data::layout::linked
//
#![doc = crate::_DOC_DATA_LAYOUT_LINKED!()] // private
#![doc = crate::_doc!(modules: crate::data::layout; linked)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//

mod r#const; // ConstList[Iterator]

crate::structural_mods! { // _mods
    _mods {
        pub use super::r#const::*;
    }
}
