// devela_base_core::data::value
//
#![doc = crate::_DOC_DATA_VALUE!()] // public
#![doc = crate::_doc!(modules: crate::data; value)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod of; // Oneof

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            of::_all::*,
        };
    }
}
