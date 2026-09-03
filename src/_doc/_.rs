// devela/src/_doc/_.rs
//
//! Extra documentation about the library.
#![doc = crate::_doc!(modules: crate; _doc: features, macros, nightly, vendored)]
#![doc = crate::_doc!(hr)]
//
#![cfg(doc)]
#![cfg_attr(nightly_doc, doc(cfg(doc)))]

// pub mod api {
//     //! Library API design.
//     #![doc = include_str!("./api.md")]
// }
pub mod constitution {
    //! # Design constitution.
    #![doc = include_str!("./constitution.md")]
}
pub mod features {
    //! # Library features.
    #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
    #![doc = include_str!("./features.md")]
}
pub mod nightly {
    //! # Nightly features.
    #![doc = include_str!("../../docs/nightly.md")]
}

crate::mods_in! {
    pub mod_ macros;

    /// # Vendored work.
    pub mod_ vendored;
}
