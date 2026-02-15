// devela_base_alloc::data::id
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod key; //
mod uid; // IdPin

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            key::_all::*,
            uid::_all::*,
        };
    }
}
