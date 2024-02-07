// devela::data::collections::array::definitions
//
//! types and traits definitions
//
// TOC
// - define types Array, BareArray, BoxedArray
// - define trait DataArray
// - implement DataCollection for Array
// - implement DataArray for Array

use crate::{
    data::{DataCollection, DataError as E, DataResult as Result},
    mem::{Bare, Storage},
};

#[cfg(feature = "alloc")]
use crate::mem::Boxed;

/* types */

/// A const generic array backed by the core [`array`] primitive.
///
/// See also the related trait: [`DataArray`][crate::DataArray]
/// and aliases: [`BareArray`] and [`BoxedArray`].
pub struct Array<T, S: Storage, const LEN: usize> {
    pub(crate) array: S::Stored<[T; LEN]>,
}

/// An [`Array`] stored in the stack.
pub type BareArray<T, const LEN: usize> = Array<T, Bare, LEN>;

/// An [`Array`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub type BoxedArray<T, const LEN: usize> = Array<T, Boxed, LEN>;
