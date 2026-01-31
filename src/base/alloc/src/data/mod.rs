// devela_base_alloc::data
//
#![doc = crate::_DOC_DATA!()] // public, root
#![doc = crate::_DOC_DATA_MODULES!()]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
//
// safety
#![cfg_attr(base_safe_data, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_DATA_MODULES =
    crate::_doc!(modules: crate; data: key, list, uid); // address, codec, error, iter, table
}

mod bit; // WIP
mod sort;

pub mod key;
pub mod list;
pub mod uid;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            bit::_all::*, // WIP
            sort::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            list::_all::*,
            key::_all::*,
            uid::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_DATA_MODULES;
    }
}
