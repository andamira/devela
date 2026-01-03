// devela::data::key
//
#![doc = crate::_DOC_DATA_KEY!()]
//!
//! Provides tools for mapping, indexing, and efficiently organizing data
//! through unique keys, including maps, sets, and other key-value structures.
//!
#![doc = crate::_doc!(extends: collections)]
//

#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /libs/base_alloc/src/data/key/_reexport.rs
mod _reexport_dep;

mod static_map; // define_static_map!
// mod set;
// mod trie;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::static_map::*;
        // pub use set::*;
        // pub use trie::*;
    }
    _reexports {
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
        pub use super::_reexport_dep::*;
    }
}
