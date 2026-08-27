// devela/src/data/store/key/mod.rs
//
#![doc = crate::_DOC_DATA_STORE_KEY!()] // public
#![doc = crate::_doc!(modules: crate::data::store; key: map, set)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Provides tools for mapping, indexing, and efficiently organizing data
//! through unique keys, including maps, sets, and other key-value structures.
//

pub mod map; // Key-value maps organized by lookup and storage strategy
pub mod set; // Sets organized by membership and storage strategy
// mod trie; // FUTURE

crate::structural_mods! { // pub_mods, _reexports
    _pub_mods {
        pub use super::{
            map::_all::*,
            set::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            map::map,
            set::SparseSetArray,
        };
    }
}
