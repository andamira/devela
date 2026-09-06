// devela/src/_doc/_.rs
//
//! Extra documentation about the library.
#![doc = crate::_doc!(modules: crate; _doc:
    constitution, features, macros, nightly, vendored)] // api, structure, tooling
#![doc = crate::_doc!(hr)]
//
#![cfg(doc)]
#![cfg(not(doctest))]
#![cfg_attr(nightly_doc, doc(cfg(doc)))]

// /// Library API design.
// pub mod api {
//     #![doc = include_str!("./api.md")]
// }
/// # Design constitution.
pub mod constitution {
    #![doc = include_str!("./constitution.md")]
}
/// # Library features.
pub mod features {
    #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./features.md")]
}
/// # Nightly features.
pub mod nightly {
    #![doc = include_str!("../../docs/nightly.md")]
}
// /// # Library structure and source organization.
// pub mod structure {
//     #![doc = include_str!("./structure.md")]
// }
// /// # Repository tooling and maintenance workflows.
// pub mod tooling {
//     #![doc = include_str!("./tooling.md")]
// }

crate::mods_in! {
    /// # Concepts, patterns, and practical notes on Rust macros.
    pub mod_ macros;

    /// # Vendored work.
    pub mod_ vendored;
}
