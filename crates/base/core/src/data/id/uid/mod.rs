// devela_base_core::data::id::uid
//
#![doc = crate::_DOC_DATA_ID_UID!()] // private
#![doc = crate::_doc!(modules: crate::data::id; uid)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod pin; // IdPin
mod registry; // IdRegistry

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            pin::*,
            registry::*,
        };
    }
}
