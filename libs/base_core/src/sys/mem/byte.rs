// devela_base_core::sys::mem::byte
//
//! Defines the [`MaybeByte`] alias.
//
// Used by:
// - sys::mem::arena::ArenaBytes.

#[allow(unused)]
use crate::MaybeUninit;

crate::CONST! {
    _DOC_MAYBE_BYTE = "A byte type that may be uninitialized depending on features.

# Features
When `unsafe_array` is **enabled** it aliases [`MaybeUninit<u8>`].
Otherwise it aliases [`u8`].";
}

#[doc = crate::_TAG_MAYBE!()]
#[doc = crate::_TAG_MEM!()]
#[doc = _DOC_MAYBE_BYTE!()]
#[cfg(any(base_safe_mem, not(feature = "unsafe_array")))] // safe
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
pub type MaybeByte = u8;

#[doc = crate::_TAG_MAYBE!()]
#[doc = crate::_TAG_MEM!()]
#[doc = _DOC_MAYBE_BYTE!()]
#[cfg(all(not(base_safe_mem), feature = "unsafe_array"))] // unsafe
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
pub type MaybeByte = MaybeUninit<u8>;
