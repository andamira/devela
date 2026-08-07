// devela/src/data/mod.rs
//
#![doc = crate::_DOC_DATA!()] // public, root
#![doc = crate::_DOC_DATA_MODULES!()]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
//!
//! The data tree separates several independent concerns:
//!
//! - [`Access`](access) determines how values are reached and traversed.
//! - [`Codecs`](codec) transform values between representations.
//! - [`Identification`](id) distinguishes values through identities and contextual references.
//! - [`Layout`](layout) determines how values are positioned and grouped.
//! - [`Storage`](store) governs how values remain and are recovered.
//! - [`Topology`](topol) describes connectivity, adjacency, and ordered relations.
//! - [`Values`](value) provide the semantic forms carried by these structures.
//
// safety
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_DATA_MODULES =
    crate::_doc!(modules: crate; data: access, codec, id, layout, store, topol, value);
}

pub mod access; // Mechanisms of reachability and traversal
pub mod codec; // Data encoding and decoding abstractions
pub mod id; // Identifiers and references for stable and contextual distinction
pub mod layout; // Structural arrangement of elements in memory or sequence
pub mod store; // Retained data stores and retrieval semantics
pub mod topol; // Relational topology over structured data
pub mod value; // Semantic value categories and composable data forms
mod word; // Fixed-width encoded data words

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            word::*,
        };
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            access::_all::*,
            codec::_all::*,
            id::_all::*,
            layout::_all::*,
            store::_all::*,
            topol::_all::*,
            value::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_DATA_MODULES;
        pub(crate) use super::{
            codec::_crate_internals::*,
            layout::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::{
            codec::_hidden::*,
            store::_hidden::*,
        };
    }
}
