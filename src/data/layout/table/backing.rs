// devela/src/data/layout/table/backing.rs
//
//! Backing-storage implementations for [`Table`][crate::Table].
//
// TOC
// - fixed array
// - shared slice
// - exclusive slice
// - boxed slice
// - vector

use crate::{Array, MismatchedCapacity, Table, TableCoord, TableLayout};

#[cfg(feature = "alloc")]
use crate::{Box, Vec};

/* fixed array */

#[rustfmt::skip]
impl<T, const LEN: usize> Table<[T; LEN]> {
    /* construction */

    /// Creates an owning table over fixed-array `storage`.
    ///
    /// Extra backing elements are permitted.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `storage` does not cover every
    /// physical position addressed by `layout`.
    pub fn try_from_array(storage: [T; LEN], layout: TableLayout)
        -> Result<Self, MismatchedCapacity> {
        match Array::try_from_array(storage, layout.as_array()) {
            Ok(array) => Ok(Self::from_array(array)),
            Err(error) => Err(error),
        }
    }

    /// Creates an owning table over a fixed native backing array in const contexts.
    ///
    /// This is the `T: Copy` counterpart of [`try_from_array`][Self::try_from_array].
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `storage` does not cover every
    /// physical position addressed by `layout`.
    pub const fn try_from_array_copy(storage: [T; LEN], layout: TableLayout)
        -> Result<Self, MismatchedCapacity> where T: Copy {
        match Array::try_from_array_copy(storage, layout.as_array()) {
            Ok(array) => Ok(Self::from_array(array)),
            Err(error) => Err(error),
        }
    }

    /* physical storage */

    /// Returns the complete physical backing slice.
    ///
    /// Physical storage order may differ from logical table order.
    pub const fn storage(&self) -> &[T] { self.array.storage() }

    /// Returns the complete exclusive physical backing slice.
    pub const fn storage_mut(&mut self) -> &mut [T] { self.array.storage_mut() }

    /// Returns exclusive access to the fixed native backing array.
    pub const fn data_mut(&mut self) -> &mut [T; LEN] { self.array.data_mut() }

    /// Returns the backing-storage length.
    pub const fn storage_len(&self) -> usize { self.array.storage_len() }

    /* logical access */

    /// Returns a shared reference to the cell at `coord`.
    pub const fn get(&self, coord: TableCoord) -> Option<&T> {
        self.array.get(coord.as_array())
    }
    /// Returns an exclusive reference to the cell at `coord`.
    pub const fn get_mut(&mut self, coord: TableCoord) -> Option<&mut T> {
        self.array.get_mut(coord.as_array())
    }

    /* reborrowing */

    /// Returns a shared slice-backed table reborrowed from this table.
    pub const fn reborrow(&self) -> Table<&[T]> {
        Table::from_array(self.array.reborrow())
    }
    /// Returns an exclusive slice-backed table reborrowed from this table.
    pub const fn reborrow_mut(&mut self) -> Table<&mut [T]> {
        Table::from_array(self.array.reborrow_mut())
    }
}

/* shared slice */

#[rustfmt::skip]
impl<'a, T> Table<&'a [T]> {
    /* construction */

    /// Creates a shared table view over `storage`.
    ///
    /// Extra backing elements are permitted.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `storage` does not cover every
    /// physical position addressed by `layout`.
    pub const fn try_from_slice(storage: &'a [T], layout: TableLayout)
        -> Result<Self, MismatchedCapacity> {
        match Array::<&[T], 2>::try_from_slice(storage, layout.as_array()) {
            Ok(array) => Ok(Self::from_array(array)),
            Err(error) => Err(error),
        }
    }
    /// Creates a shared slice-backed table with inference-friendly naming.
    pub const fn try_from_slice_ref(storage: &'a [T], layout: TableLayout)
        -> Result<Self, MismatchedCapacity> {
        Self::try_from_slice(storage, layout)
    }

    /* physical storage */

    /// Returns the complete physical backing slice.
    pub const fn storage(&self) -> &'a [T] { self.array.storage() }

    /// Returns the backing-storage length.
    pub const fn storage_len(&self) -> usize { self.array.storage_len() }

    /* logical access */

    /// Returns the cell at `coord`.
    pub const fn get(&self, coord: TableCoord) -> Option<&'a T> {
        self.array.get(coord.as_array())
    }

    /* reborrowing */

    /// Returns a shared table view reborrowed for the lifetime of `self`.
    pub const fn reborrow(&self) -> Table<&[T]> {
        Table::from_array(self.array.reborrow())
    }
}

/* exclusive slice */

#[rustfmt::skip]
impl<'a, T> Table<&'a mut [T]> {
    /* construction */

    /// Creates an exclusive table view over `storage`.
    ///
    /// Extra backing elements are permitted.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `storage` does not cover every
    /// physical position addressed by `layout`.
    pub const fn try_from_slice(storage: &'a mut [T], layout: TableLayout)
        -> Result<Self, MismatchedCapacity> {
        match Array::<&mut [T], 2>::try_from_slice(storage, layout.as_array()) {
            Ok(array) => Ok(Self::from_array(array)),
            Err(error) => Err(error),
        }
    }

    /// Creates an exclusive slice-backed table with inference-friendly naming.
    pub const fn try_from_slice_mut(storage: &'a mut [T], layout: TableLayout)
        -> Result<Self, MismatchedCapacity> {
        Self::try_from_slice(storage, layout)
    }

    /* physical storage */

    /// Returns the complete physical backing slice.
    pub const fn storage(&self) -> &[T] { self.array.storage() }

    /// Returns the complete exclusive physical backing slice.
    pub const fn storage_mut(&mut self) -> &mut [T] { self.array.storage_mut() }

    /// Returns the backing-storage length.
    pub const fn storage_len(&self) -> usize { self.array.storage_len() }

    /* logical access */

    /// Returns a shared reference to the cell at `coord`.
    pub const fn get(&self, coord: TableCoord) -> Option<&T> {
        self.array.get(coord.as_array())
    }
    /// Returns an exclusive reference to the cell at `coord`.
    pub const fn get_mut(&mut self, coord: TableCoord) -> Option<&mut T> {
        self.array.get_mut(coord.as_array())
    }

    /* reborrowing */

    /// Returns a shared table view reborrowed for the lifetime of `self`.
    pub const fn reborrow(&self) -> Table<&[T]> {
        Table::from_array(self.array.reborrow())
    }
    /// Returns an exclusive table view reborrowed for the lifetime of `self`.
    pub const fn reborrow_mut(&mut self) -> Table<&mut [T]> {
        Table::from_array(self.array.reborrow_mut())
    }
    /// Consumes the exclusive view and returns a shared view.
    pub const fn into_shared(self) -> Table<&'a [T]> {
        Table::from_array(self.array.into_shared())
    }
}

/* boxed slice */

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[rustfmt::skip]
impl<T> Table<Box<[T]>> {
    /* construction */

    /// Creates an owning table over the boxed slice `storage`.
    pub fn try_from_boxed_slice(storage: Box<[T]>, layout: TableLayout)
        -> Result<Self, MismatchedCapacity> {
        match Array::try_from_boxed_slice(storage, layout.as_array()) {
            Ok(array) => Ok(Self::from_array(array)),
            Err(error) => Err(error),
        }
    }

    /* physical storage */

    /// Returns the complete physical backing slice.
    pub fn storage(&self) -> &[T] { self.array.storage() }
    /// Returns the complete exclusive physical backing slice.
    pub fn storage_mut(&mut self) -> &mut [T] { self.array.storage_mut() }
    /// Returns the backing-storage length.
    pub fn storage_len(&self) -> usize { self.array.storage_len() }

    /* logical access */

    /// Returns a shared reference to the cell at `coord`.
    pub fn get(&self, coord: TableCoord) -> Option<&T> {
        self.array.get(coord.as_array())
    }
    /// Returns an exclusive reference to the cell at `coord`.
    pub fn get_mut(&mut self, coord: TableCoord) -> Option<&mut T> {
        self.array.get_mut(coord.as_array())
    }

    /* reborrowing */

    /// Returns a shared table view reborrowed for the lifetime of `self`.
    pub fn reborrow(&self) -> Table<&[T]> {
        Table::from_array(self.array.reborrow())
    }
    /// Returns an exclusive table view reborrowed for the lifetime of `self`.
    pub fn reborrow_mut(&mut self) -> Table<&mut [T]> {
        Table::from_array(self.array.reborrow_mut())
    }

    /* converting */

    /// Converts this boxed-slice-backed table into a vector-backed table.
    ///
    /// The logical layout is preserved.
    pub fn into_vec(self) -> Table<Vec<T>> {
        Table::from_array(self.array.into_vec())
    }
}

/* vector */

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
#[rustfmt::skip]
impl<T> Table<Vec<T>> {
    /* construction */

    /// Creates an owning table over the vector `storage`.
    ///
    /// Only the initialized vector length counts as accessible storage.
    pub fn try_from_vec(storage: Vec<T>, layout: TableLayout) -> Result<Self, MismatchedCapacity> {
        match Array::try_from_vec(storage, layout.as_array()) {
            Ok(array) => Ok(Self::from_array(array)),
            Err(error) => Err(error),
        }
    }

    /* physical storage */

    /// Returns the complete physical backing slice.
    pub fn storage(&self) -> &[T] { self.array.storage() }
    /// Returns the complete exclusive physical backing slice.
    pub fn storage_mut(&mut self) -> &mut [T] { self.array.storage_mut() }
    /// Returns the backing-storage length.
    pub fn storage_len(&self) -> usize { self.array.storage_len() }

    /* logical access */

    /// Returns a shared reference to the cell at `coord`.
    pub fn get(&self, coord: TableCoord) -> Option<&T> {
        self.array.get(coord.as_array())
    }
    /// Returns an exclusive reference to the cell at `coord`.
    pub fn get_mut(&mut self, coord: TableCoord) -> Option<&mut T> {
        self.array.get_mut(coord.as_array())
    }

    /* reborrowing */

    /// Returns a shared table view reborrowed for the lifetime of `self`.
    pub fn reborrow(&self) -> Table<&[T]> {
        Table::from_array(self.array.reborrow())
    }
    /// Returns an exclusive table view reborrowed for the lifetime of `self`.
    pub fn reborrow_mut(&mut self) -> Table<&mut [T]> {
        Table::from_array(self.array.reborrow_mut())
    }

    /* converting */

    /// Converts this vector-backed table into a boxed-slice-backed table.
    ///
    /// The logical layout is preserved.
    pub fn into_boxed(self) -> Table<Box<[T]>> {
        Table::from_array(self.array.into_boxed())
    }
}
