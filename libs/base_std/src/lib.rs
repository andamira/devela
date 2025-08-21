// devela_base_std::lib
//
//!
//

/* global configuration */
//
// lints
//
// (Most lints are defined in ::devela::Cargo.toml::lints)
#![deny(rustdoc::missing_debug_implementations)]
#![cfg_attr(
    not(all(doc, feature = "_docsrs")), // if features are incomplete…
    allow(rustdoc::broken_intra_doc_links) // …allow broken intra-doc links
)]
//
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// nightly (flags)
#![cfg_attr(nightly_doc, feature(doc_cfg))]

/* imports */

extern crate self as devela_base_std;

/* root modules */

pub mod build;
// pub mod sys;

#[doc(hidden)]
#[allow(unused_imports)]
pub use all::*;
pub mod all {
    // public items, feature-gated, visible at their origin and here in `all`
    //
    //! All the crate's items flat re-exported.
    //! <br/><hr>
    //!
    //! Note that these items are already re-exported (hidden) from the root,
    //! as is every other public module's contents from their parent.
    #[allow(unused_imports)]
    #[rustfmt::skip]
    #[doc(inline)]
    pub use super::{
        build::_all::*,
        // sys::_all::*,
    };
}
