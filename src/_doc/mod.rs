// devela::_doc
//
//! Extra documentation about the library.
#![doc = crate::_doc!(modules: crate; _doc: examples, features, macros, nightly, vendored)]
#![doc = crate::_doc!(br+hr)] // gives way to the first root module
//
// #![cfg(any(doc, test))] // RETHINK
#![cfg_attr(nightly_doc, doc(cfg(any(doc, test))))]

// #[doc(hidden)] // TEMP
// #[cfg(feature = "std")]
// #[path = "../../build/main/mod.rs"]
// pub mod build; // Build-time configuration and code generation.

#[cfg(feature = "_docs_examples")]
pub mod examples; // Documented examples

pub mod macros;

#[cfg(doc)]
crate::items! {
    // pub mod api {
    //     //! Library API design.
    //     #![doc = include_str!("./api.md")]
    // }
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

// IMPROVE: some could be _crate_internals, currently blocked by define_error! and rand_pcg!
crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[cfg(feature = "_docs_examples")]
        pub use super::examples::_all::*;
    }
}
