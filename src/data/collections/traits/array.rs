// devela::data::collections::traits::array
//
//! DataArray abstract data type
//
// TOC
// - define trait DataArray
// - impl for devela types:
//   - Array

use crate::{
    data::{DataCollection, DataErrors as E, DataResult as Result},
    mem::Storage,
};

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

impl<T, S: Storage, const LEN: usize> DataArray for crate::data::Array<T, S, LEN> {
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