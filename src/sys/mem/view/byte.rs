// devela/src/sys/mem/view/byte.rs
//
//! Defines the [`MaybeByte`] alias.
//
// Used by:
// - sys::mem::alloc::arena::ArenaBytes.

#[allow(unused)]
use crate::MaybeUninit;
use crate::{
    __cfg_item_safe_hide, __cfg_item_unsafe_hide, _doc_location, _tags, CONST, macro_apply,
};
CONST! {
    _DOC_MAYBE_BYTE = "
# Features
Uses [`MaybeUninit<u8>`] only in unsafe memory builds with `unsafe_array`.

In safe memory builds, or when `unsafe_array` is disabled, it aliases [`u8`].";
}

#[doc = _tags!(maybe mem)]
/// A byte type that may be uninitialized depending on features.
#[doc = _doc_location!("sys/mem/view")]
#[doc = _DOC_MAYBE_BYTE!()]
#[macro_apply(__cfg_item_safe_hide("safe_mem", "unsafe_array"))]
pub type MaybeByte = u8;

#[doc = _tags!(maybe mem)]
/// A byte type that may be uninitialized depending on features.
#[doc = _doc_location!("sys/mem/view")]
#[doc = _DOC_MAYBE_BYTE!()]
#[macro_apply(__cfg_item_unsafe_hide("safe_mem", "unsafe_array"))]
pub type MaybeByte = MaybeUninit<u8>;
