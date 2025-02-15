// devela::_info
//
//! Extra information about the library.
//

#![cfg(any(doc, test))]
#![cfg_attr(feature = "nightly_doc", doc(cfg(any(doc, test))))]

/// Build functionality.
#[cfg(feature = "std")]
#[path = "../../build/mod.rs"]
mod build;

/// Documented examples.
pub mod examples;

#[cfg(doc)]
crate::items!{
    pub mod features {
        //! Library features.
        #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
        #![doc = include_str!("./features.md")]
    }
    pub mod nightly {
        //! Nightly features.
        // WIP
        #![doc = include_str!("../../DOCS/NIGHTLY.md")]
    }
    /// Vendored work.
    pub mod vendored;
}
