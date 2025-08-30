// devela::data::key
//
#![doc = crate::_DOC_DATA_KEY!()]
//!
//! Provides tools for mapping, indexing, and efficiently organizing data
//! through unique keys, including maps, sets, and other key-value structures.
//!
#![doc = crate::_doc!(extends: collections)]
//

mod reexports;
mod static_map; // define_static_map!

// WIPZONE
// mod set;
// mod trie;

crate::structural_mods! { // _mods
    _mods {
        pub use super::reexports::*;
        pub use super::static_map::*;
        // WIPZONE
        // pub use set::*;
        // pub use trie::*;
    }
}
