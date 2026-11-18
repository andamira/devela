//
//! Minimal, documentation-independent crate bootstrap.
//!
//! Provides the construction macros needed before the other modules are available.
//

#![allow(unused_imports)]

mod _doc;
mod r#const;
mod mods_out;
mod doclink;

pub use _doc::{_DOC_ALL, _DOC_ALL_, _DOC_ALL_PLUS_};
pub use doclink::{__DOCLINK_CUSTOM_CHANNEL, __DOCLINK_CUSTOM_DOMAIN, doclink};
pub use {
    ::core::unreachable,
    r#const::CONST,
    devela_macros::{compile, compile_attr, mods_in, paste},
    mods_out::mods_out,
};
