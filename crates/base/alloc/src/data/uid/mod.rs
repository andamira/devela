// devela_base_alloc::data::uid
//
#![doc = crate::_DOC_DATA_UID!()] // public
#![doc = crate::_doc!(modules: crate::data; uid)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_DATA_UID!()]
//

mod pin_box;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            pin_box::*,
        };
    }
}
