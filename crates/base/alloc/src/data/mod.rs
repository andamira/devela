// devela_base_alloc::data
//
#![doc = crate::_DOC_DATA!()] // public, root
#![doc = crate::_DOC_DATA_MODULES!()]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
//
// safety
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_DATA_MODULES =
    crate::_doc!(modules: crate; data: key, list, uid); // address, codec, error, iter, table
}

mod bit; // WIP
mod sort;

pub mod id;
pub mod list;

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            bit::_all::*, // WIP
            sort::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            id::_all::*,
            list::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_DATA_MODULES;
    }
}
