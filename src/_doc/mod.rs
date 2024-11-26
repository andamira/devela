// devela::_doc
//
//! Additional library documentation.
//

#![cfg(any(doc, test))]
#![cfg_attr(feature = "nightly_doc", doc(cfg(doc)))]

#[cfg(feature = "std")]
#[path = "../../build/mod.rs"]
mod build;

/// Documented examples.
pub mod examples;

/// Library features explained.
#[cfg(doc)]
pub mod features {
    #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./features.md")]
}

/// Vendored and adapted work.
pub mod vendored {
    #![doc = include_str!("../../DOCS/VENDORED_rustdoc.md")]
    #![doc = include_str!("../../DOCS/VENDORED.md")]
}
