// devela/src/text/translit/namespace.rs
//
//! Defines the [`Translit`] namespace.
//

#[doc = crate::_tags!(text namespace)]
/// Lossy text transliteration utilities.
///
/// Provides small, mostly `const` mappings
/// from Unicode scalars to simpler textual approximations.
///
/// # Features
/// All current functionality depends on the `translit` feature.
#[doc = crate::_doc_meta!{location("text")}]
#[derive(Debug)]
pub struct Translit;
