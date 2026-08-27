// devela/src/data/layout/table/coord.rs
//
//! Defines [`TableCoord`] and [`TableCoordIter`].
//

use crate::{ArrayCoordIter, ArrayShape, TableShape, lets, unwrap};
use crate::{ConstInit, IteratorDoubleEnded, IteratorExactSize, IteratorFused};

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

#[doc = crate::_tags!(data_structure iterator)]
/// An iterator over the coordinates of a table shape.
#[doc = crate::_doc_meta!{
    location("data/layout/table", struct TableCoordIter),
    #[cfg(target_pointer_width = "32")]
    test_size_of(TableCoordIter = 28|224; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(TableCoordIter = 56|448; niche !Option),
}]
/// Coordinates are yielded in canonical table order,
/// with columns changing fastest.
///
/// For a table with 2 rows and 3 columns, the sequence is:
/// ```text
/// (0, 0)
/// (0, 1)
/// (0, 2)
/// (1, 0)
/// (1, 1)
/// (1, 2)
/// ```
///
/// This logical order is independent of the physical [`TableLayout`][crate::TableLayout].
/// It matches physical storage order for a dense row-major layout;
/// for other layouts, logical and physical traversal order may differ.
#[must_use]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TableCoordIter {
    iter: ArrayCoordIter<2>,
}

impl crate::Debug for TableCoordIter {
    fn fmt(&self, f: &mut crate::Formatter<'_>) -> crate::FmtResult<()> {
        f.debug_struct("TableCoordIter")
            .field("shape", &self.shape())
            .field("next", &self.peek())
            .field("next_back", &self.peek_back())
            .field("remaining", &self.len())
            .finish()
    }
}

impl TableCoordIter {
    /// Creates an iterator with a previously validated cell count.
    pub(crate) const fn new(shape: TableShape, remaining: usize) -> Self {
        let array_shape = ArrayShape::new([shape.columns(), shape.rows()]);
        Self { iter: ArrayCoordIter::new(array_shape, remaining) }
    }

    /// Returns the number of coordinates not yet yielded.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.iter.len()
    }
    /// Returns whether no coordinates remain.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.iter.is_empty()
    }
    /// Returns the complete logical table shape being traversed.
    pub const fn shape(&self) -> TableShape {
        lets! { shape = self.iter.shape(), lengths = shape.lengths() }
        TableShape::new(lengths[1], lengths[0])
    }
    /// Advances the iterator and returns the next coordinate from the front.
    #[must_use]
    pub const fn next(&mut self) -> Option<TableCoord> {
        unwrap![=some_map self.iter.next(), |coord| Self::table_coord(coord)]
    }
    /// Advances the iterator and returns the next coordinate from the back.
    #[must_use]
    pub const fn next_back(&mut self) -> Option<TableCoord> {
        unwrap![=some_map self.iter.next_back(), |coord| Self::table_coord(coord)]
    }
    /// Returns the next coordinate from the front without advancing.
    pub const fn peek(&self) -> Option<TableCoord> {
        unwrap![=some_map self.iter.peek(), |coord| Self::table_coord(coord)]
    }
    /// Returns the next coordinate from the back without advancing.
    pub const fn peek_back(&self) -> Option<TableCoord> {
        unwrap![=some_map self.iter.peek_back(), |coord| Self::table_coord(coord)]
    }
    const fn table_coord(coord: [usize; 2]) -> TableCoord {
        TableCoord::new(coord[1], coord[0]) // Internal array axes are [column, row]
    }
}

impl Iterator for TableCoordIter {
    type Item = TableCoord;
    fn next(&mut self) -> Option<Self::Item> {
        Self::next(self)
    }
    fn count(self) -> usize {
        self.len()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl IteratorDoubleEnded for TableCoordIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        Self::next_back(self)
    }
}
impl IteratorExactSize for TableCoordIter {
    fn len(&self) -> usize {
        Self::len(self)
    }
}
impl IteratorFused for TableCoordIter {}
