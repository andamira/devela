// devela/src/data/layout/table/define.rs
//
//! Defines [`Table`].
//

use crate::{Array, TableCoordIter, TableLayout, TableShape};

#[doc = crate::_tags!(data_structure mem)]
/// A logical table over backing storage.
#[doc = crate::_doc_meta!{
    location("data/layout/table", struct Table),
    #[cfg(target_pointer_width = "32")]
    test_size_of(__:Table<[u8; 6]> = 28|224; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(__:Table<[u8; 6]> = 48|384; niche !Option),
}]
/// A table interprets a two-dimensional [`Array`] with axes `[row, column]`.
///
/// `D` determines ownership and storage capabilities;
/// [`TableLayout`] maps logical cells into storage.
///
/// # Invariant
///
/// Backing/layout validity is inherited from the wrapped [`Array`].
/// `Table` adds no further storage invariant.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Table<D> {
    pub(super) array: Array<D, 2>,
}

#[rustfmt::skip]
impl<D> Table<D> {
    /* construction and representation */

    /// Interprets a two-dimensional array as a table with axes `[row, column]`.
    pub const fn from_array(array: Array<D, 2>) -> Self { Self { array } }

    /// Returns the underlying two-dimensional array.
    pub const fn as_array(&self) -> &Array<D, 2> { &self.array }

    /// Returns the underlying two-dimensional array exclusively.
    pub const fn as_array_mut(&mut self) -> &mut Array<D, 2> { &mut self.array }

    /// Consumes the table and returns the underlying array.
    pub fn into_array(self) -> Array<D, 2> { self.array }

    /* backing */

    /// Returns a shared reference to the backing data.
    pub const fn data(&self) -> &D { self.array.data() }

    /// Consumes the table and returns its backing data.
    pub fn into_data(self) -> D { self.array.into_data() }

    /// Decomposes the table into its backing data and table layout.
    pub fn into_parts(self) -> (D, TableLayout) {
        let (data, layout) = self.array.into_parts();
        (data, TableLayout::from_array(layout))
    }

    /* logical structure */

    /// Returns the table layout.
    pub const fn layout(&self) -> TableLayout { TableLayout::from_array(self.array.layout()) }

    /// Returns the logical table shape.
    pub const fn shape(&self) -> TableShape { TableShape::from_array(self.array.shape()) }

    /// Returns the number of rows.
    pub const fn rows(&self) -> usize { self.shape().rows() }

    /// Returns the number of columns.
    pub const fn columns(&self) -> usize { self.shape().columns() }

    /// Returns the number of logical cells.
    pub const fn cell_count(&self) -> usize { self.array.element_count() }

    /// Returns whether the logical table contains no cells.
    pub const fn is_empty(&self) -> bool { self.array.is_empty() }

    /// Returns an iterator over every logical table coordinate.
    pub const fn coords(&self) -> TableCoordIter {
        TableCoordIter::new(self.shape(), self.cell_count())
    }
}

impl<D> From<Array<D, 2>> for Table<D> {
    fn from(array: Array<D, 2>) -> Self {
        Self::from_array(array)
    }
}
impl<D> From<Table<D>> for Array<D, 2> {
    fn from(table: Table<D>) -> Self {
        table.into_array()
    }
}
