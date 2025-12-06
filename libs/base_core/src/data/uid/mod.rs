// devela_base_core::data::uid
//
#![doc = crate::_DOC_DATA_UID!()]
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
