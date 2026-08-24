// devela/src/media/visual/image/raster/grid/coord.rs
//
//! Defines [`RasterGridIter`].
//

use crate::{IteratorFused, Position2, RasterGrid, is};

#[doc = crate::_tags!(image iterator)]
/// An iterator over the coordinates of a raster grid.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster/grid"),
    test_size_of(RasterCoordIter = 32|256),
}]
/// Coordinates are yielded in canonical raster order, with `x` changing fastest.
///
/// The iterator traverses the complete logical raster domain
/// independently of sample storage and physical layout.
///
/// Unlike array-coordinate traversal, the number of raster cells may exceed
/// `usize::MAX` on targets where `usize` is narrower than `u64`.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RasterCoordIter {
    grid: RasterGrid,
    front: Position2<u32>,
    back: Position2<u32>,
    remaining: u64,
}

impl RasterCoordIter {
    pub(crate) const fn new(grid: RasterGrid) -> Self {
        let remaining = grid.cell_count();
        let back = if remaining == 0 {
            Position2::new([0, 0])
        } else {
            Position2::new([grid.width() - 1, grid.height() - 1])
        };
        Self {
            grid,
            front: Position2::new([0, 0]),
            back,
            remaining,
        }
    }
    /// Returns the raster grid being traversed.
    pub const fn grid(&self) -> RasterGrid {
        self.grid
    }
    /// Returns the exact number of coordinates not yet yielded.
    pub const fn remaining(&self) -> u64 {
        self.remaining
    }
    /// Returns whether no coordinates remain.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.remaining == 0
    }
    /// Returns the next coordinate from the front.
    #[must_use]
    pub const fn next(&mut self) -> Option<Position2<u32>> {
        is! { self.remaining == 0, return None }
        let coord = self.front;
        self.remaining -= 1;
        is! { self.remaining != 0, self.advance_front() }
        Some(coord)
    }
    /// Returns the next coordinate from the back.
    #[must_use]
    pub const fn next_back(&mut self) -> Option<Position2<u32>> {
        is! { self.remaining == 0, return None }
        let coord = self.back;
        self.remaining -= 1;
        is! { self.remaining != 0, self.advance_back() }
        Some(coord)
    }
    /// Returns the next coordinate from the front without removing it.
    pub const fn peek(&self) -> Option<Position2<u32>> {
        is! { self.remaining == 0, None, Some(self.front) }
    }
    /// Returns the next coordinate from the back without removing it.
    pub const fn peek_back(&self) -> Option<Position2<u32>> {
        is! { self.remaining == 0, None, Some(self.back) }
    }

    /* helpers */

    const fn advance_front(&mut self) {
        if self.front.dim[0] < self.grid.width() - 1 {
            self.front.dim[0] += 1;
        } else {
            self.front.dim[0] = 0;
            self.front.dim[1] += 1;
        }
    }
    const fn advance_back(&mut self) {
        if self.back.dim[0] != 0 {
            self.back.dim[0] -= 1;
        } else {
            self.back.dim[0] = self.grid.width() - 1;
            self.back.dim[1] -= 1;
        }
    }
}

impl Iterator for RasterCoordIter {
    type Item = Position2<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        Self::next(self)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.remaining > usize::MAX as u64 {
            (usize::MAX, None)
        } else {
            let len = self.remaining as usize;
            (len, Some(len))
        }
    }
}
impl DoubleEndedIterator for RasterCoordIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        Self::next_back(self)
    }
}
impl IteratorFused for RasterCoordIter {}
