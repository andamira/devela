// devela/src/data/store/mod.rs
//
#![doc = crate::_DOC_DATA_STORE!()] // public
#![doc = crate::_doc!(modules: crate::data; store: arena, key, pool)] // cache, db, intern
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Storage governs how values remain and are recovered.
//!
//! A store retains values across operations and defines how they are inserted,
//! retrieved, reclaimed, and potentially reused.
//!
//! [`Layout`](crate::data::layout) describes arrangement and occupancy.
//! [`Identity`](crate::data::id) distinguishes one entity from another.
//! Storage determines the lifecycle of the retained values themselves.
//!
//! - [`Arenas`](arena) advance monotonically and reclaim storage collectively.
// - [`Caches`](cache) retain computed or retrieved values to avoid repeated work.
// - [`Databases`](db) persist and query structured data under schema and transaction policies.
// - [`Interners`](intern) canonicalize equal values into shared representatives.
//! - [`Keyed stores`](key) recover values through keys and lookup structures.
//! - [`Pools`](mod@pool) reclaim and reuse individual slots.
//

pub mod arena; // Monotonic stores with stable handles and collective reclamation
// mod cache; // Stores that retain values to reduce recomputation or retrieval cost
// mod db; // Persistent queryable stores with schema and transactional semantics
// mod intern; // Canonical stores for deduplicated values and shared identity
pub mod key; // Keyed stores and lookup structures
pub mod pool; // Reusable stores with stable handles and individual reclamation

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        pub use super::{
            // cache::_all::*,
            // db::_all::*,
            // intern::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            arena::_all::*,
            key::_all::*,
            pool::_all::*,
        };
    }
    _hidden {
        pub use super::{
            arena::_hidden::*,
        };
    }
}
