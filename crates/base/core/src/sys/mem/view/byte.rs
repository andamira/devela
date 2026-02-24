// devela_base_core::sys::mem::view::byte
//
//! Defines the [`MaybeByte`] alias.
//
// Used by:
// - sys::mem::alloc::arena::ArenaBytes.

#[allow(unused)]
use crate::MaybeUninit;

crate::CONST! {
    _DOC_MAYBE_BYTE = "
# Features
When `unsafe_array` is **enabled** it aliases [`MaybeUninit<u8>`].
Otherwise it aliases [`u8`].";
}

#[doc = crate::_tags!(maybe mem)]
/// A byte type that may be uninitialized depending on features.
#[doc = crate::_doc_location!("sys/mem/view")]
#[doc = _DOC_MAYBE_BYTE!()]
#[cfg(any(feature = "safe_mem", not(feature = "unsafe_array")))] // safe
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
pub type MaybeByte = u8;

#[doc = crate::_tags!(maybe mem)]
/// A byte type that may be uninitialized depending on features.
#[doc = crate::_doc_location!("sys/mem/view")]
#[doc = _DOC_MAYBE_BYTE!()]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_array"))] // unsafe
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
pub type MaybeByte = MaybeUninit<u8>;
