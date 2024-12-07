// devela::data::dst
//
#![doc = crate::doc_miri_warn!(tag)]
//! Dynamically-sized types stored without need of heap allocation.
//!
#![doc = crate::doc_miri_warn!(body,
    url: "https://github.com/thepowersgang/stack_dst-rs/issues/14")]
#![doc = include_str!("./Mod.md")]
//!
//! # Derived Work
#![doc = include_str!("./MODIFICATIONS.md")]
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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {doc_inline::*, items_private::*} ;

    mod doc_inline {
        pub use super::{buffer::*, queue::*, stack::*, value::*};
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    mod items_private {
        pub(super) use super::helpers::{
            check_fat_pointer, decompose_pointer, list_push_gen, make_fat_ptr,
            round_to_words, store_metadata,
        };
    }
}
