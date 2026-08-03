// devela/src/lang/disc/journal/mod.rs
//
//! Journalistic inquiry, attribution, reporting, and presentation.
//!
//! Discourse forms that organize sourced truth-claims under evidential,
//! editorial, and public-accountability constraints.
//

// mod attribution; // Association of statements and claims with their sources.
// mod correction; // Amendments, retractions, and transparent revision.
// mod investigate; // Evidence gathering and development of reported findings.
// mod quote; // Selected verbatim or closely represented source speech.
// mod report; // Factual reporting structures and event presentation.
// mod source; // Sources, provenance, access, and source relationships.
// mod work; // Publishable journalistic works and their organization.

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // attribution::_all::*,
            // correction::_all::*,
            // investigate::_all::*,
            // quote::_all::*,
            // report::_all::*,
            // source::_all::*,
            // work::_all::*,
        };
    }
}
