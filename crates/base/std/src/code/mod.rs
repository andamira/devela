// devela_base_std::code
//
#![doc = crate::_DOC_CODE!()] // public, root
#![doc = crate::_DOC_CODE_MODULES!()]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends:
    any, clone, convert, default, error, hint, marker, ops, panic, result)]
//
// safety
#![cfg_attr(feature = "safe_code", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_CODE_MODULES =
    crate::_doc!(modules: crate; code: error, panic); // marker, ops, result, util
}

pub mod error;
pub mod panic;
pub mod result; // only tests for now

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            error::_all::*,
            panic::_all::*,
            // result::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_CODE_MODULES;
    }
}
