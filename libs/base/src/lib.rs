// devela_base::lib
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
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]

// nightly (flags)
#![cfg_attr(nightly_doc, feature(doc_cfg))]

/* imports */

extern crate self as devela_base;

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub extern crate alloc;

/* root modules */

pub mod code;
// pub mod num;

#[doc(hidden)]
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
        code::_all::*,
        // num::_all::*,
    };
}

#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use _internals::*;
#[doc(hidden)] #[rustfmt::skip]
pub/*workspace*/ mod _internals {
    #[allow(unused_imports)]
    pub/*workspace*/ use super::{
        code::_internals::*,
    };
}
