//
//! The root-independent foundation of devela.
//!
//! Boot provides the minimum expressive baseline of the library: elementary
//! vocabulary, construction mechanisms, and shared infrastructure that do not
//! require any selectable domain module.
//!
//! Its responsibilities are:
//! - `synth`: structural synthesis and crate construction;
//! - `expr`: ergonomic expression, binding, and control flow;
//! - `rust`: access to Rust's library vocabulary;
//! - `dep` and `_doc`: dependency and documentation infrastructure.
//!
//! The baseline supports `no_std` without allocation or platform services.
//! Domain modules build upon it and may provide the semantic homes and public
//! documentation of abstractions implemented here.
//!
//! Admission to boot requires elemental or cross-domain constructive utility,
//! a small dependency closure, and independence from selectable domain modules.
//! Boot is not a general collection of shared utilities.
//

#![allow(unused_imports)]

mod _doc; // documentation
pub mod dep; // dependencies
#[path = "expr/_.rs"]
mod expr; // elementary expression
#[path = "rust/_.rs"]
pub(crate) mod rust; // re-exports from the Rust lib
#[path = "synth/_.rs"]
mod synth; // structural synthesis

pub use _bootstrap::*;
pub use expr::*;

// Early crate-construction exports; distinct from the expression vocabulary.
pub(crate) mod _bootstrap {
    pub use super::_doc::{
        __DOCLINK_CUSTOM_CHANNEL, __DOCLINK_CUSTOM_DOMAIN, _DOC_ALL, _DOC_ALL_, _DOC_ALL_PLUS_,
        doclink,
    };
    #[cfg(not(root_yard))]
    pub(crate) use super::_doc::{_doc_meta, _tags};

    pub use super::synth::_all::*;

    pub use ::core::unreachable;
    pub use devela_macros::{compile, compile_attr, mods_in, paste};
}
pub(crate) mod _crate_internals {
    pub(crate) use super::synth::_crate_internals::*;
}
