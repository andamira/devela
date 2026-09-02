// devela/src/lang/disc/journal/_.rs
//
//! Journalistic inquiry, attribution, reporting, and presentation.
//!
//! Discourse forms that organize sourced truth-claims under evidential,
//! editorial, and public-accountability constraints.
//

crate::mods_in! {
    // mod_ attribution; // Association of statements and claims with their sources.
    // mod_ correction; // Amendments, retractions, and transparent revision.
    // mod_ investigate; // Evidence gathering and development of reported findings.
    // mod_ quote; // Selected verbatim or closely represented source speech.
    // mod_ report; // Factual reporting structures and event presentation.
    // mod_ source; // Sources, provenance, access, and source relationships.
    // mod_ work; // Publishable journalistic works and their organization.
}
crate::mods_out! { // _mods
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
