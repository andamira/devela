// devela_base_std::data::codec
//
#![doc = crate::_DOC_DATA_CODEC!()] // public
#![doc = crate::_doc!(modules: crate::data; codec: hash)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: hash)]
//

pub mod hash;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            hash::_all::*,
        };
    }
}
