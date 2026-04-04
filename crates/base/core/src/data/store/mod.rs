// devela_base_core::data::store
//
#![doc = crate::_DOC_DATA_STORE!()] // public
#![doc = crate::_doc!(modules: crate::data; store)] // cache, db, key
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_DATA_ID!()]
//

// pub mod cache; // Stores that retain values to reduce recomputation or retrieval cost
// pub mod db; // Persistent queryable stores with schema and transactional semantics
// pub mod key; // Keyed stores and lookup structures

crate::structural_mods! { // _pub_mods
    _pub_mods {
        // pub use super::{
        //     cache::_all::*,
        //     db::_all::*,
        //     key::_all::*,
        // };
    }
}
