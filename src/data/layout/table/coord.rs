// devela/src/data/layout/table/coord.rs
//
//! Defines [`TableCoord`].
//

use crate::ConstInit;

#[doc = crate::_tags!(data_structure)]
/// The row and column coordinates of a table cell.
#[doc = crate::_doc_meta!{
    location("data/layout/table", struct TableCoord),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TableCoord = 8|64; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TableCoord = 16|128; niche !Option),
}]
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TableCoord {
    coord: [usize; 2],
}

#[rustfmt::skip]
impl TableCoord {
    /// Creates a table coordinate from its zero-based row and column.
    pub const fn new(row: usize, column: usize) -> Self {
        Self { coord: [row, column] }
    }

    /// Returns the zero-based row.
    pub const fn row(self) -> usize { self.coord[0] }

    /// Returns the zero-based column.
    pub const fn column(self) -> usize { self.coord[1] }

    /// Returns the underlying array coordinate `[row, column]`.
    pub const fn as_array(self) -> [usize; 2] { self.coord }

    /// Creates a table coordinate from an array coordinate `[row, column]`.
    pub const fn from_array(coord: [usize; 2]) -> Self { Self { coord } }
}

impl ConstInit for TableCoord {
    const INIT: Self = Self::new(0, 0);
}
impl Default for TableCoord {
    fn default() -> Self {
        Self::INIT
    }
}

impl From<[usize; 2]> for TableCoord {
    fn from(coord: [usize; 2]) -> Self {
        Self::from_array(coord)
    }
}
impl From<TableCoord> for [usize; 2] {
    fn from(coord: TableCoord) -> Self {
        coord.as_array()
    }
}
