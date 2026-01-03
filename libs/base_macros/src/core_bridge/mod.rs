// devela_base_macros::core_bridge
//
//! Copied helpers from `devela_base_core`.
//!
//! These are duplicated here because proc-macro crates cannot
//! depend on exported `macro_rules!` items.
//

mod _doc_location; // COPIED from /libs/base_core/src/code/util/_doc.rs
mod doclink; // COPIED from /libs/base_core/src/code/util/doclink.rs

pub(crate) use {_doc_location::*, doclink::*};
