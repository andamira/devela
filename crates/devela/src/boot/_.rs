//
//! Minimal, documentation-independent crate bootstrap.
//!
//! Provides the construction macros needed before the other modules are available.
//

#![allow(unused_imports)]

mod _doc; // documentation
pub mod dep; // dependencies
#[path = "macros/_.rs"]
mod macros; // utility macros
#[path = "rust/_.rs"]
pub(crate) mod rust; // re-exports from the Rust lib

pub use _all::*;
pub(crate) mod _all {
    pub use super::_doc::{_DOC_ALL, _DOC_ALL_, _DOC_ALL_PLUS_};
    #[cfg(not(root_yard))]
    pub(crate) use super::_doc::{_doc_meta, _tags};

    pub use super::macros::_all::*;

    pub use ::core::unreachable;
    pub use devela_macros::{compile, compile_attr, mods_in, paste};
}
pub(crate) mod _crate_internals {
    pub(crate) use super::macros::_crate_internals::*;
}
