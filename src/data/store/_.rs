// devela/src/data/store/_.rs
//
#![doc = crate::_DOC_DATA_STORE!()] // public
#![doc = crate::_doc!(modules: crate::data; store: arena, key, pool)] // cache, db, intern
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(extends: collections)]
//!
//! Storage governs how values remain available across operations.
//!
//! A store defines a retention model: how values enter it, how they are found,
//! when they cease to be retained, and whether reclaimed capacity
//! or identities may later be reused.
//

crate::mods_in! {
    pub mod_ arena; // Monotonic stores with stable handles and collective reclamation
    // mod cache; // Stores that retain values to reduce recomputation or retrieval cost
    // mod db; // Persistent queryable stores with schema and transactional semantics
    pub mod_ intern; // Canonical stores for deduplicated values and shared identity
    pub mod_ key; // Keyed stores and lookup structures
    pub mod_ pool; // Reusable stores with stable handles and individual reclamation
}
crate::mods_out! { // _mods, _pub_mods, _reexports, _hidden
    _mods {
        pub use super::{
            // cache::_all::*,
            // db::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            arena::_all::*,
            intern::_all::*,
            key::_all::*,
            pool::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            arena::arena,
            intern::intern_string,
            key::map::map,
            pool::pool,
        };
    }
    _hidden {
        pub use super::{
            arena::_hidden::*,
            intern::_hidden::*,
            key::_hidden::*,
            pool::_hidden::*,
        };
    }
}
