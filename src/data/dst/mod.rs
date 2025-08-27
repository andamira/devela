// devela::data::dst
//
#![doc = crate::_doc_miri_warn!(tag)]
//! Dynamically-sized types stored without need of heap allocation.
//!
#![doc = crate::_doc_miri_warn!(body,
    url: "https://github.com/thepowersgang/stack_dst-rs/issues/14")]
#![doc = include_str!("./Mod.md")]
//!
#![doc = crate::doc_!(vendor: "stack_dst")]
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

crate::items! { // structural access: _mods, _all, _internals
    #[allow(unused)]
    pub use {_mods::*, _internals::*} ;

    mod _mods {
        pub use super::{buffer::*, queue::*, stack::*, value::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    mod _internals {
        pub(super) use super::helpers::{
            check_fat_pointer, decompose_pointer, list_push_gen, make_fat_ptr,
            round_to_words, store_metadata,
        };
    }
}
