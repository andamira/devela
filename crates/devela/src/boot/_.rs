//
//! Minimal, documentation-independent crate bootstrap.
//!
//! Provides the construction macros needed before the other modules are available.
//

mod doc;
mod mods_out;

pub use doc::{_DOC_ALL, _DOC_ALL_, _DOC_ALL_PLUS_};

pub use devela_macros::mods_in;
#[allow(unused)]
pub use mods_out::mods_out;
