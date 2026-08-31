// devela/src/data/id/uuid/_.rs
//
#![doc = crate::_DOC_DATA_ID_UUID!()] // public
#![doc = crate::_doc!(modules: crate::data::id; uuid)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! UUIDs carry identity across contexts
//! without requiring a shared local allocator or store.
//!
//! The UUID format defines a standardized 128-bit value.
//! Different versions assign different structure and
//! generation semantics within that representation.
//!
//! UUIDs are suited to identities that must persist or travel
//! across processes, storage, systems, or network boundaries.
//! Uniqueness guarantees depend on the version and generation method.
//!
//! [`UuidV7Generator`] adds stateful version 7 generation,
//! preserving strict UUID ordering across repeated or regressing timestamps.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // Uuid
    mod generator; // UuidV7Generator
    mod non_nil; // UuidNonNil
    mod variant; // UuidVariant, UuidVersion
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Uuid,
            generator::UuidV7Generator,
            non_nil::UuidNonNil,
            variant::{UuidVariant, UuidVersion},
        };
    }
}
