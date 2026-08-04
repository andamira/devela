// devela/src/data/store/mod.rs
//
#![doc = crate::_DOC_DATA_STORE!()] // public
#![doc = crate::_doc!(modules: crate::data; store: key)] // cache, db
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_DATA_ID!()]
//

mod arena; // Monotonic stores with stable handles and collective reclamation
// mod cache; // Stores that retain values to reduce recomputation or retrieval cost
// mod db; // Persistent queryable stores with schema and transactional semantics
pub mod key; // Keyed stores and lookup structures
mod pool; // Reusable stores with stable handles and individual reclamation

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        pub use super::{
            arena::_all::*,
            // cache::_all::*,
            // db::_all::*,
            pool::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            key::_all::*,
        };
    }
    _hidden {
        pub use super::{
            arena::_hidden::*,
        };
    }
}
