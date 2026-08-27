// devela/src/data/layout/table/shape.rs
//
//! Defines [`TableShape`].
//

use crate::{ArrayShape, ConstInit, Overflow, TableCoord, TableCoordIter, unwrap};

#[doc = crate::_tags!(data_structure)]
/// The numbers of rows and columns in a table.
#[doc = crate::_doc_meta!{
    location("data/layout/table", struct TableShape),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TableShape = 8|64; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TableShape = 16|128; niche !Option),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TableShape {
    shape: ArrayShape<2>,
}

#[rustfmt::skip]
impl TableShape {
    /// Creates a table shape with `rows` rows and `columns` columns.
    pub const fn new(rows: usize, columns: usize) -> Self {
        Self { shape: ArrayShape::new([rows, columns]) }
    }

    /// Returns the number of rows.
    pub const fn rows(self) -> usize { self.shape.lengths()[0] }

    /// Returns the number of columns.
    pub const fn columns(self) -> usize { self.shape.lengths()[1] }

    /// Returns whether this shape contains no cells.
    pub const fn is_empty(self) -> bool { self.shape.is_empty() }

    /// Returns the total number of cells.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the cell count is not representable.
    pub const fn cell_count(self) -> Result<usize, Overflow> {
        self.shape.element_count()
    }
    /// Returns whether `coord` lies within this table shape.
    pub const fn contains(self, coord: TableCoord) -> bool {
        self.shape.contains_coord(coord.as_array())
    }

    /// Returns the underlying two-dimensional array shape.
    pub const fn as_array(self) -> ArrayShape<2> { self.shape }

    /// Creates a table shape from an array shape interpreted as
    /// `[rows, columns]`.
    pub const fn from_array(shape: ArrayShape<2>) -> Self { Self { shape } }

    /// Returns an iterator over every logical table coordinate.
    ///
    /// Coordinates are yielded row by row, with columns changing fastest.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the cell count is not representable.
    pub const fn try_coords(self) -> Result<TableCoordIter, Overflow> {
        unwrap![=ok_map self.cell_count(), |count| TableCoordIter::new(self, count)]
    }
}

impl ConstInit for TableShape {
    const INIT: Self = Self::from_array(ArrayShape::INIT);
}
impl Default for TableShape {
    fn default() -> Self {
        Self::INIT
    }
}
impl From<ArrayShape<2>> for TableShape {
    fn from(shape: ArrayShape<2>) -> Self {
        Self::from_array(shape)
    }
}
impl From<TableShape> for ArrayShape<2> {
    fn from(shape: TableShape) -> Self {
        shape.as_array()
    }
}
