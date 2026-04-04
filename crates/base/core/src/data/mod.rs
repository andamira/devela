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
    crate::_doc!(modules: crate; data: access, codec, error, id, layout, store, topol, value);
}

pub mod access; // Mechanisms of reachability and traversal
pub mod codec; // Data encoding and decoding abstractions
pub mod error; // Data-related error types
pub mod id; // Identity abstractions for stable and contextual distinction
pub mod layout; // Structural arrangement of elements in memory or sequence
pub mod store; // Retained data stores and retrieval semantics
pub mod topol; // Relational topology over structured data
pub mod value; // Enumerated data values and types, classified by size

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            access::_all::*,
            codec::_all::*,
            error::*,
            id::_all::*,
            layout::_all::*,
            store::_all::*,
            topol::_all::*,
            value::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_DATA_MODULES;
    }
}
