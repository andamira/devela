// devela_base_core::data::list::link
//
#![doc = crate::_DOC_DATA_LIST_LINK!()] // private
#![doc = crate::_doc!(modules: crate::data::list; link)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//

mod r#const; // ConstList[Iterator]

crate::structural_mods! { // _mods
    _mods {
        pub use super::r#const::*;
    }
}
