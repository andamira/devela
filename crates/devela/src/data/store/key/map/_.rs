//
#![doc = crate::_DOC_DATA_STORE_KEY_MAP!()] // public
#![doc = crate::_doc!(modules: crate::data::store::key; map)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//! Maps associate each admitted key with a value
//! and recover that value through key lookup.
//!
//! Implementations may differ in capacity,
//! allocation, hashing, and storage strategy.
//

crate::mods_in! {
    mod alias;

    mod_ fixed; // Fixed-capacity open-addressed hash maps with mutable entries
    // mod_ perfect; // Immutable perfect-hash maps over statically known key sets
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            alias::*,
            fixed::_all::*,
            // perfect::_all::*,
        };
    }
    _reexports {
        pub use crate::boot::rust::data::store_key_map::*;
    }
    _hidden {
        pub use super::{
            fixed::_hidden::*,
        };
    }
}
