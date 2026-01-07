// devela_base_core::data
//
#![doc = crate::_DOC_DATA!()]
#![doc = crate::_DOC_DATA_MODULES!()]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
//
// safety
#![cfg_attr(base_safe_data, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_DATA_MODULES =
    crate::_doc!(modules: crate; data: bit, codec, error, iter, list, uid); // key, table
}

mod handle; // define_handle!
mod sort; // Sort

// pub mod address; // WIP
pub mod bit; // Bitwise, bitfield!
pub mod codec;
pub mod error;
pub mod iter;
// pub mod key;
pub mod list;
// pub mod space; // Grid
pub mod uid; // IdPin

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            handle::*,
            sort::_all::*,
            uid::_all::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            // address::_all::*, // WIP
            bit::_all::*,
            codec::_all::*,
            error::*,
            iter::_all::*,
            // key::_all::*,
            list::_all::*,
            // space::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_DATA_MODULES;
    }
}
