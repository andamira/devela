// devela/src/data/store/key/mod.rs
//
#![doc = crate::_DOC_DATA_STORE_KEY!()] // public
#![doc = crate::_doc!(modules: crate::data::store; key: map)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Provides tools for mapping, indexing, and efficiently organizing data
//! through unique keys, including maps, sets, and other key-value structures.
//

#[cfg(feature = "alloc")]
mod _reexport_alloc;
mod _reexport_dep;

pub mod map; // Key-value maps organized by lookup and storage strategy
mod set; // SparseSet[Array|Error], LinuxSparseSet
// mod trie;

crate::structural_mods! { // _mods, pub_mods, _reexports
    _mods {
        pub use super::{
            set::*,
            // trie::*,
        };
    }
    _pub_mods {
        pub use super::{
            map::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            map::map,
        };
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        pub use super::_reexport_dep::*;
    }
}
