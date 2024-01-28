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

/// A generic array backed by the core [`array`] primitive.
pub struct Array<T, S: Storage, const LEN: usize> {
    pub(crate) array: S::Stored<[T; LEN]>,
}

/// An [`Array`] stored in the heap.
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub type BoxedArray<T, const LEN: usize> = Array<T, Boxed, LEN>;

/// An [`Array`] stored in the stack.
pub type DirectArray<T, const LEN: usize> = Array<T, (), LEN>;

/* trait */

/// An abstract Array.
///
/// - <https://en.wikipedia.org/wiki/Array_(data_type)#Abstract_arrays>
pub trait DataArray: DataCollection {
    /// Returns an immutable reference to the element at the specified `index`.
    fn array_ref_get(&self, index: usize) -> Result<&<Self as DataCollection>::Element>;

    /// Returns a mutable reference to the element at the given `index`.
    fn array_mut_get(&mut self, index: usize) -> Result<&mut <Self as DataCollection>::Element>;

    /// Sets the element at the specified `index` to the given `value`.
    fn array_set(&mut self, index: usize, value: <Self as DataCollection>::Element) -> Result<()>;

    /// Sets the element at the specified `index` to the given clonable `value`.
    fn array_set_ref(
        &mut self,
        index: usize,
        value: &<Self as DataCollection>::Element,
    ) -> Result<()>
    where
        Self::Element: Clone;
}

/* impls */

#[rustfmt::skip]
impl<T, S: Storage, const LEN: usize> DataCollection for Array<T, S, LEN> {
    type Element = T;
    /// The capacity of a fixed-size array is always equal to its length.
    fn collection_capacity(&self) -> Result<usize> { Ok(LEN) }
    fn collection_len(&self) -> Result<usize> { Ok(LEN) }
    /// Returns [`NotSupported`][E::NotSupported] since a fixed-size array is never empty or full.
    fn collection_is_empty(&self) -> Result<bool> { E::ns() }
    /// Returns [`NotSupported`][E::NotSupported] since a fixed-size array is never empty or full.
    fn collection_is_full(&self) -> Result<bool> { E::ns() }
    fn collection_contains(&self, element: Self::Element) -> Result<bool> where T: PartialEq {
        Ok(self.contains(&element))
    }
    fn collection_count(&self, element: &Self::Element) -> Result<usize> where T: PartialEq {
        Ok(self.iter().filter(|&e| e == element).count())
    }
}

impl<T, S: Storage, const LEN: usize> DataArray for Array<T, S, LEN> {
    fn array_ref_get(&self, idx: usize) -> Result<&<Self as DataCollection>::Element> {
        if let Some(e) = self.get(idx) {
            Ok(e)
        } else {
            Err(E::OutOfBounds(Some(idx)))
        }
    }
    fn array_mut_get(&mut self, idx: usize) -> Result<&mut <Self as DataCollection>::Element> {
        if let Some(e) = self.get_mut(idx) {
            Ok(e)
        } else {
            Err(E::OutOfBounds(Some(idx)))
        }
    }
    fn array_set(&mut self, idx: usize, value: <Self as DataCollection>::Element) -> Result<()> {
        if let Some(e) = self.get_mut(idx) {
            *e = value;
            Ok(())
        } else {
            Err(E::OutOfBounds(Some(idx)))
        }
    }
    fn array_set_ref(&mut self, idx: usize, value: &<Self as DataCollection>::Element) -> Result<()>
    where
        T: Clone,
    {
        if let Some(e) = self.get_mut(idx) {
            *e = value.clone();
            Ok(())
        } else {
            Err(E::OutOfBounds(Some(idx)))
        }
    }
}
