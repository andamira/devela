// devela/src/data/store/key/_.rs
//
#![doc = crate::_DOC_DATA_STORE_KEY!()] // public
#![doc = crate::_doc!(modules: crate::data::store; key: map, set)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Provides tools for mapping, indexing, and efficiently organizing data
//! through unique keys, including maps, sets, and other key-value structures.
//

crate::mods_in! {
    pub mod_ map; // Key-value maps organized by lookup and storage strategy
    pub mod_ set; // Sets organized by membership and storage strategy
    // mod trie; // FUTURE
}
crate::mods_out! { // pub_mods, _reexports, _hidden
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
    _hidden {
        pub use super::{
            map::_hidden::*,
        };
    }
}
