// devela_base_core::data::id
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_QUO_DATA_ID!()]
//

mod handle; // define_handle!
// mod key; //
mod uid; // IdPin

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            handle::*,
            // key::_all::*,
            uid::_all::*,
        };
    }
}
