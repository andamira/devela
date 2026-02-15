// devela::data::access
//
#![doc = crate::_DOC_DATA_ACCESS!()] // public
#![doc = crate::_doc!(modules: crate::data; access)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_QUO_DATA_ACCESS!()]
//

// mod cursor;
// mod address;

pub mod iter;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        // pub use super::{
        //     // address::_all::*,
        //     // cursor::_all::*,
        // };
    }
    _pub_mods {
        pub use super::{
            iter::_all::*,
        };
    }
}
