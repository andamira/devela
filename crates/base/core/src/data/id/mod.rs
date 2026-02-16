// devela_base_core::data::id
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id)] // key
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_QUO_DATA_ID!()]
//

mod handle; // define_handle!
mod uid; // IdPin

// pub mod key; //

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            handle::*,
            uid::_all::*,
        };
    }
    _pub_mods {
        // pub use super::{
        //     // key::_all::*,
        // };
    }
}
