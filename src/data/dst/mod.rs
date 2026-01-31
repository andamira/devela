// devela::data::dst
//
#![doc = crate::_doc_miri_warn!(tag)]
#![doc = crate::_DOC_DATA_DST!()] // public
#![doc = crate::_doc!(modules: crate::data; dst)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
#![doc = crate::_doc_miri_warn!(body,
    url: "https://github.com/thepowersgang/stack_dst-rs/issues/14")]
#![doc = include_str!("./Mod.md")]
//!
#![doc = crate::_doc!(vendor: "stack_dst")]
//
//
#![allow(clippy::result_unit_err)] // IMPROVE

mod helpers;

#[cfg(test)]
mod tests;

mod buffer;
mod queue;
mod stack;
mod value;

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{buffer::*, queue::*, stack::*, value::*};
    }
    _crate_internals {
        pub(super) use super::helpers::{
            check_fat_pointer, decompose_pointer, list_push_gen, make_fat_ptr,
            round_to_words, store_metadata,
        };
    }
}
