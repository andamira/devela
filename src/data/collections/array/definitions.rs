// devela::data::collections::array::definitions
//
//! types and traits definitions
//
// TOC
// - define types Array, BoxedArray, Directarray
// - define trait DataArray
// - implement DataCollection for Array
// - implement DataArray for Array

use crate::{
    data::{DataCollection, DataErrors as E, DataResult as Result},
    mem::Storage,
};

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

/* types */

/// A const generic array backed by the core [`array`] primitive.
pub struct Array<T, S: Storage, const LEN: usize> {
    pub(crate) array: S::Stored<[T; LEN]>,
}

/// An [`Array`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub type BoxedArray<T, const LEN: usize> = Array<T, Boxed, LEN>;

/// An [`Array`] stored in the stack.
pub type DirectArray<T, const LEN: usize> = Array<T, (), LEN>;
