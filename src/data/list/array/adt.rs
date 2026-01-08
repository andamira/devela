// devela::data::list::array::adt
//
//! [`DataArray`] abstract data type
//
// TOC
// - define trait DataArray
// - impl for:
//   - Array

use crate::{DataCollection, IndexOutOfBounds, Storage};

#[doc = crate::_tags!(data_structure)]
/// An abstract *array* data type.
#[doc = crate::_doc_location!("data/list/array")]
///
/// - <https://en.wikipedia.org/wiki/Array_(data_type)#Abstract_arrays>
pub trait DataArray: DataCollection {
    /// Returns an immutable reference to the element at the specified `index`.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if the given `index` is out of bounds.
    fn array_ref_get(
        &self,
        index: usize,
    ) -> Result<&<Self as DataCollection>::Element, IndexOutOfBounds>;

    /// Returns a mutable reference to the element at the given `index`.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if the given `index` is out of bounds.
    fn array_mut_get(
        &mut self,
        index: usize,
    ) -> Result<&mut <Self as DataCollection>::Element, IndexOutOfBounds>;

    /// Sets the element at the specified `index` to the given `value`.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if the given `index` is out of bounds.
    fn array_set(
        &mut self,
        index: usize,
        value: <Self as DataCollection>::Element,
    ) -> Result<(), IndexOutOfBounds>;

    /// Sets the element at the specified `index` to the given clonable `value`.
    /// # Errors
    /// Returns [`IndexOutOfBounds`] if the given `index` is out of bounds.
    fn array_set_ref(
        &mut self,
        index: usize,
        value: &<Self as DataCollection>::Element,
    ) -> Result<(), IndexOutOfBounds>
    where
        Self::Element: Clone;
}

/* impl for Array */

impl<T, const LEN: usize, S: Storage> DataArray for crate::data::Array<T, LEN, S> {
    fn array_ref_get(
        &self,
        idx: usize,
    ) -> Result<&<Self as DataCollection>::Element, IndexOutOfBounds> {
        if let Some(e) = self.get(idx) {
            Ok(e)
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_mut_get(
        &mut self,
        idx: usize,
    ) -> Result<&mut <Self as DataCollection>::Element, IndexOutOfBounds> {
        if let Some(e) = self.get_mut(idx) {
            Ok(e)
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_set(
        &mut self,
        idx: usize,
        value: <Self as DataCollection>::Element,
    ) -> Result<(), IndexOutOfBounds> {
        if let Some(e) = self.get_mut(idx) {
            *e = value;
            Ok(())
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_set_ref(
        &mut self,
        idx: usize,
        value: &<Self as DataCollection>::Element,
    ) -> Result<(), IndexOutOfBounds>
    where
        T: Clone,
    {
        if let Some(e) = self.get_mut(idx) {
            *e = value.clone();
            Ok(())
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
}

/* impl for core array [T; LEN] */

impl<T, const LEN: usize> DataArray for [T; LEN] {
    fn array_ref_get(
        &self,
        idx: usize,
    ) -> Result<&<Self as DataCollection>::Element, IndexOutOfBounds> {
        if let Some(e) = self.get(idx) {
            Ok(e)
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_mut_get(
        &mut self,
        idx: usize,
    ) -> Result<&mut <Self as DataCollection>::Element, IndexOutOfBounds> {
        if let Some(e) = self.get_mut(idx) {
            Ok(e)
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_set(
        &mut self,
        idx: usize,
        value: <Self as DataCollection>::Element,
    ) -> Result<(), IndexOutOfBounds> {
        if let Some(e) = self.get_mut(idx) {
            *e = value;
            Ok(())
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_set_ref(
        &mut self,
        idx: usize,
        value: &<Self as DataCollection>::Element,
    ) -> Result<(), IndexOutOfBounds>
    where
        T: Clone,
    {
        if let Some(e) = self.get_mut(idx) {
            *e = value.clone();
            Ok(())
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
}

/* impl for Vec */

#[cfg(feature = "alloc")]
impl<T> DataArray for crate::Vec<T> {
    fn array_ref_get(
        &self,
        idx: usize,
    ) -> Result<&<Self as DataCollection>::Element, IndexOutOfBounds> {
        if let Some(e) = self.get(idx) {
            Ok(e)
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_mut_get(
        &mut self,
        idx: usize,
    ) -> Result<&mut <Self as DataCollection>::Element, IndexOutOfBounds> {
        if let Some(e) = self.get_mut(idx) {
            Ok(e)
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_set(
        &mut self,
        idx: usize,
        value: <Self as DataCollection>::Element,
    ) -> Result<(), IndexOutOfBounds> {
        if let Some(e) = self.get_mut(idx) {
            *e = value;
            Ok(())
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
    fn array_set_ref(
        &mut self,
        idx: usize,
        value: &<Self as DataCollection>::Element,
    ) -> Result<(), IndexOutOfBounds>
    where
        T: Clone,
    {
        if let Some(e) = self.get_mut(idx) {
            *e = value.clone();
            Ok(())
        } else {
            Err(IndexOutOfBounds(Some(idx)))
        }
    }
}
