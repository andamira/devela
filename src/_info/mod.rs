// devela::_info
//
//! Extra information about the library.
//

#![cfg(any(doc, test))]
#![cfg_attr(nightly_doc, doc(cfg(any(doc, test))))]

/// Build-time metaprogramming and code generation utilities.
///
/// Contains logic executed during `cargo build`, including:
/// - Feature flag management
/// - Compile-time environment inspection
/// - Procedural code generation
#[cfg(feature = "std")]
#[path = "../../build/main/mod.rs"]
pub mod build;

/// Documented examples.
pub mod examples;

/// Procedural macros.
pub mod macros {
    #![doc = include_str!("../../libs/macros/src/Lib.md")]
}

#[cfg(doc)]
crate::items! {
    pub mod features {
        //! Library features.
        #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
        #![doc = include_str!("./features.md")]
    }
    pub mod nightly {
        //! Nightly features.
        #![doc = include_str!("../../docs/nightly.md")]
    }
    /// Vendored work.
    pub mod vendored;
}
