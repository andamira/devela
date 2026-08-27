// devela/src/data/layout/table/layout.rs
//
//! Defines [`TableLayout`].
//

use crate::{ArrayLayout, ConstInit, Overflow, TableCoord, TableShape};

#[doc = crate::_tags!(data_structure mem)]
/// An affine mapping from table cells to linear storage positions.
#[doc = crate::_doc_meta!{
    location("data/layout/table", struct TableLayout),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TableLayout = 20|160; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TableLayout = 40|320; niche !Option),
}]
/// A table coordinate is interpreted as `[row, column]`.
///
/// [`row_major`][Self::row_major] stores consecutive columns together,
/// while [`column_major`][Self::column_major] stores consecutive rows
/// together.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TableLayout {
    layout: ArrayLayout<2>,
}

#[rustfmt::skip]
impl TableLayout {
    /// Creates a dense row-major table layout.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the resulting layout is not representable.
    pub const fn row_major(shape: TableShape) -> Result<Self, Overflow> {
        match ArrayLayout::dense_last(shape.as_array()) {
            Ok(layout) => Ok(Self { layout }),
            Err(error) => Err(error),
        }
    }

    /// Creates a dense column-major table layout.
    ///
    /// # Errors
    /// Returns [`Overflow`] if the resulting layout is not representable.
    pub const fn column_major(shape: TableShape) -> Result<Self, Overflow> {
        match ArrayLayout::dense_first(shape.as_array()) {
            Ok(layout) => Ok(Self { layout }),
            Err(error) => Err(error),
        }
    }

    /// Returns the logical table shape.
    pub const fn shape(self) -> TableShape {
        TableShape::from_array(self.layout.shape())
    }

    /// Returns the number of rows.
    pub const fn rows(self) -> usize { self.shape().rows() }

    /// Returns the number of columns.
    pub const fn columns(self) -> usize { self.shape().columns() }

    /// Returns the physical origin in the backing storage.
    pub const fn offset(self) -> usize { self.layout.offset() }

    /// Returns the physical stride between consecutive rows.
    pub const fn row_stride(self) -> isize { self.layout.strides()[0] }

    /// Returns the physical stride between consecutive columns.
    pub const fn column_stride(self) -> isize { self.layout.strides()[1] }

    /// Returns whether the logical table contains no cells.
    pub const fn is_empty(self) -> bool { self.layout.is_empty() }

    /// Returns the number of logical cells.
    pub const fn cell_count(self) -> usize { self.layout.element_count() }

    /// Returns the minimum backing-storage length required by this layout.
    pub const fn required_storage_len(self) -> usize {
        self.layout.required_storage_len()
    }

    /// Returns whether `coord` lies within this table.
    pub const fn contains(self, coord: TableCoord) -> bool {
        self.shape().contains(coord)
    }

    /// Returns the backing-storage position corresponding to `coord`.
    pub const fn storage_index(self, coord: TableCoord) -> Option<usize> {
        self.layout.storage_index(coord.as_array())
    }

    /// Returns the underlying two-dimensional array layout.
    pub const fn as_array(self) -> ArrayLayout<2> { self.layout }

    /// Interprets a two-dimensional array layout as a table layout,
    /// with axes `[row, column]`.
    pub const fn from_array(layout: ArrayLayout<2>) -> Self {
        Self { layout }
    }
}

impl ConstInit for TableLayout {
    const INIT: Self = Self::from_array(ArrayLayout::INIT);
}
impl Default for TableLayout {
    fn default() -> Self {
        Self::INIT
    }
}
impl From<ArrayLayout<2>> for TableLayout {
    fn from(layout: ArrayLayout<2>) -> Self {
        Self::from_array(layout)
    }
}
impl From<TableLayout> for ArrayLayout<2> {
    fn from(layout: TableLayout) -> Self {
        layout.as_array()
    }
}
