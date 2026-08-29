// devela/src/data/mod.rs
//
#![doc = crate::_DOC_DATA!()] // public, root
#![doc = crate::_DOC_DATA_MODULES!()]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: array, collections, hash, iter, vec)]
//!
//! The data tree separates several independent concerns:
//!
//! - [`Access`](mod@access) determines how values are reached and traversed.
//! - [`Codecs`](mod@codec) transform or derive data representations.
//! - [`Identification`](mod@id) distinguishes values through identities and contextual references.
//! - [`Layout`](mod@layout) determines how values are positioned and grouped.
//! - [`History`](mod@history) records how values originate and evolve.
//! - [`Storage`](mod@store) governs how values remain and are recovered.
//! - [`Topology`](mod@topol) describes connectivity, adjacency, and ordered relations.
//! - [`Values`](mod@value) provide the semantic forms carried by these structures.
//! - [`Words`](mod@word) expose exact canonical raw representations of copyable values.
//
// safety
#![cfg_attr(feature = "safe_data", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_DATA_MODULES =
    crate::_doc!(modules: crate; data:
        access, codec, history, id, layout, store, topol, value, word);
}

pub mod access; // Mechanisms of reachability and traversal
pub mod codec; // Data encoding and decoding abstractions
pub mod history; // Origins and evolution of data across derivations and change
pub mod id; // Identifiers and references for stable and contextual distinction
pub mod layout; // Structural arrangement of elements in memory or sequence
pub mod store; // Retained data stores and retrieval semantics
pub mod topol; // Relational topology over structured data
pub mod value; // Semantic value categories and composable data forms
pub mod word; // Data words with exact canonical raw representations

crate::structural_mods! { // _pub_mods, _crate_internals, _reexports, _hidden
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            access::_all::*,
            codec::_all::*,
            id::_all::*,
            layout::_all::*,
            history::_all::*,
            store::_all::*,
            topol::_all::*,
            value::_all::*,
            word::_all::{Word, WordTry, word},
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            access::{ByteCursor, IteratorLending},
            codec::{HasherFx, bitfield, set},
            id::handle::handle,
            layout::array::Array,
            store::{arena::arena, pool::pool},
            value::ValueKind,
            word::WordTry,
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
