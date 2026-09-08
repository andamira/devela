// devela/src/_doc/_.rs
//
//! Extra documentation about the library.
#![doc = crate::_doc!(modules: crate; _doc: design, practice, reference)]
#![doc = crate::_doc!(hr)]
//!
//! This section gathers the broader material that belongs
//! outside the documentation of any single item or module.
//!
//! It gives space to ideas, conventions, and accumulated knowledge
//! that concern the library as a whole, evolving with the code.
//
#![cfg(doc)]
#![cfg(not(doctest))]
#![cfg_attr(nightly_doc, doc(cfg(doc)))]

crate::mods_in! {
    pub mod_ design; // Principles, structure, and API conventions.
    pub mod_ practice; // Patterns, techniques, and project tooling.
    pub mod_ reference; // Configuration, inventories, and provenance.
}
