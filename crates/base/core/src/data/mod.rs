// devela_base_core::data
//
#![doc = crate::_DOC_DATA!()] // public, root
#![doc = crate::_DOC_DATA_MODULES!()]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
#![doc = crate::_QUO_DATA!()]
//
// safety
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_DATA_MODULES =
    crate::_doc!(modules: crate; data: access, codec, error, id, layout, value); // topol
}

pub mod access;
pub mod codec;
pub mod error;
pub mod id; // distinction persistence
pub mod layout;
// pub mod topol;
pub mod value;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            access::_all::*,
            codec::_all::*,
            error::*,
            id::_all::*,
            layout::_all::*,
            // topol::_all::*,
            value::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_DATA_MODULES;
    }
}
