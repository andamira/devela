// devela/src/lang/sem/rel/mod.rs
//
//! Semantic relations.
//!
//! > How meaning relates.
//

// mod about; // Aboutness and semantic reference
// mod comp; // Composition of relations
// mod equiv; // Semantic equivalence
mod relation; // Relation
// mod sim; // Semantic similarity and affinity

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // about::_all::*,
            // comp::_all::*,
            // equiv::_all::*,
            relation::Relation,
            // sim::_all::*,
        };
    }
}
