// devela::_doc
//
//! Additional library documentation.
//
// TOC
// - standard libraries
// - external dependencies

#![cfg(any(doc, test))]
#![cfg_attr(feature = "nightly_doc", doc(cfg(doc)))]

/// Documented examples.
pub mod examples;

/// Library features explained.
#[cfg(doc)]
pub mod features {
    #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./features.md")]
}
