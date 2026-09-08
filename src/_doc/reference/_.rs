// devela/src/_doc/reference/_.rs
//
//! Configuration, inventories, and provenance.
#![doc = crate::_doc!(modules: crate::_doc; reference: features, nightly, vendored)]
#![doc = crate::_doc!(hr)]
//!
//! Some project knowledge is mostly about keeping facts straight.
//!
//! This section keeps the material that supports the library over time,
//! including configurations, inventories, and provenance.
//

/// Cargo features and configuration flags.
pub mod features {
    #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./features.md")]
}
/// Tracked unstable Rust features and nightly configuration.
pub mod nightly {
    #![doc = include_str!("../../../docs/nightly.md")]
}
crate::mods_in! {
    /// Provenance and modifications for vendored work.
    pub mod_ vendored;
}
