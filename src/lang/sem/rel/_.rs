// devela/src/lang/sem/rel/_.rs
//
//! Semantic relations.
//!
//! > How meaning relates.
//

crate::mods_in! {
    // mod about; // Aboutness and semantic reference
    // mod comp; // Composition of relations
    // mod equiv; // Semantic equivalence
    mod relation; // Relation
    // mod sim; // Semantic similarity and affinity
}
crate::mods_out! { // _mods
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
