// devela::data::store::key
//
#![doc = crate::_DOC_DATA_STORE_KEY!()] // public
#![doc = crate::_doc!(modules: crate::data::store; key)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Provides tools for mapping, indexing, and efficiently organizing data
//! through unique keys, including maps, sets, and other key-value structures.
//!
//

#[cfg(feature = "alloc")]
mod _reexport_alloc;
mod _reexport_dep;

mod map; // map!
// mod set;
// mod trie;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            map::*,
            // set::*,
            // trie::*,
        };
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        pub use super::_reexport_dep::*;
    }
}
