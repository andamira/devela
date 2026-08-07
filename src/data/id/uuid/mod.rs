// devela/src/data/id/handle/mod.rs
//
#![doc = crate::_DOC_DATA_ID_UUID!()] // public
#![doc = crate::_doc!(modules: crate::data::id; uuid)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! UUIDs carry identity across contexts
//! without requiring a shared local allocator or store.
//!
//! All UUIDs use the same standardized 128-bit representation.
//! Different versions assign different structure and
//! generation semantics within that representation.
//!
//! UUIDs are suited to identities that must persist or travel
//! across processes, storage, systems, or network boundaries.
//! Uniqueness guarantees depend on the version and generation method.
//

// mod __; //

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     __::*,
        // };
    }
}
