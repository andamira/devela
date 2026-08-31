// devela/src/data/history/_.rs
//
#![doc = crate::_DOC_DATA_HISTORY!()] // public
#![doc = crate::_doc!(modules: crate::data; history)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Historical relations are expressed as semantic [`Relation`][crate::Relation]s.
//!
//! They conventionally point from a result or later state
//! toward its source or predecessor.
//
//

crate::mods_in! {
    // mod activity; // Producing or transforming occurrences within a history MAYBE
    // mod change;   // Descriptions of transitions between distinguishable states
    // mod lineage;  // Ancestry and dependency views over historical relations
    mod rel; // Historical relations between data and its antecedents
    // mod snapshot; // Captured materializations of state at a historical point
    // mod version;  // Identifiable historical states and parentage MAYBE
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // activity::*,
            // change::*,
            // lineage::*,
            rel::{DerivedFrom, RevisionOf},
            // snapshot::*,
            // version::*,
        };
    }
}
