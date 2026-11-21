//
//! Minimal, documentation-independent crate bootstrap.
//!
//! Provides the construction macros needed before the other modules are available.
//

#![allow(unused_imports)]

mod _doc;
mod _reexport;
mod r#const;
mod mods_out;
mod doclink;

#[path = "rust/_.rs"]
pub(crate) mod rust; // re-exports from the Rust lib.

pub use _doc::{_DOC_ALL, _DOC_ALL_, _DOC_ALL_PLUS_};
#[cfg(not(root_yard))]
pub(crate) use _doc::{_doc_meta, _tags};

pub use _reexport::_reexport;
pub use r#const::CONST;
pub use doclink::{__DOCLINK_CUSTOM_CHANNEL, __DOCLINK_CUSTOM_DOMAIN, doclink};
pub use mods_out::mods_out;
pub use {
    ::core::unreachable,
    devela_macros::{compile, compile_attr, mods_in, paste},
};
