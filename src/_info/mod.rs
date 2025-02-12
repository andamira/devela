// devela::_info
//
//! Extra information about the library.
//

#![cfg(any(doc, test))]
#![cfg_attr(feature = "nightly_doc", doc(cfg(any(doc, test))))]

/// The build scripts.
#[cfg(feature = "std")]
#[path = "../../build/mod.rs"]
mod build;

/// Documented examples.
pub mod examples;

#[cfg(doc)]
pub mod features {
    //! Library features.
    #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./features.md")]
}

#[cfg(doc)]
/// Vendored work.
pub mod vendored;
