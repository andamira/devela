// devela/src/media/visual/image/raster/draw/line.rs
//
//! Defines [`RasterLineIter`].
//
// > Which raster cells represent a one-cell-wide lattice line?

use crate::{IteratorFused, Position2, RasterElement, RasterGrid, is, lets};

#[doc = crate::_tags!(image iterator)]
/// An iterator over the covered cells of an aliased raster line.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster/draw"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RasterLineIter = 40|320),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RasterLineIter = 80|640),
}]
/// `RasterLineIter` rasterizes a one-cell-wide line
/// between two signed raster-lattice positions.
///
/// The line:
/// - includes both endpoints;
/// - is 8-connected;
/// - emits at most one element per major-axis position;
/// - emits only cells contained by its [`RasterGrid`];
/// - gives every emitted element [`Coverage8::FULL`][crate::Coverage8::FULL].
///
/// Rasterization is independent of sample storage, color, paint, compositing,
/// and physical raster layout.
///
/// # Cell-selection invariant
///
/// For any endpoints `a` and `b`, swapping the endpoints reverses the emitted
/// sequence without changing the selected cells:
///
/// ```text
/// line(a, b) == reverse(line(b, a))
/// ```
///
/// This includes exact half-cell ties and cells skipped by grid clipping.
///
/// At exact ties, this invariant is preferred over complete reflection symmetry:
/// reflecting a line across an axis may select the complementary equally-near cells.
///
/// # Algorithm
///
/// Uses a canonicalized major-axis integer error accumulator in the
/// Bresenham/DDA family. One major-axis step is taken per candidate cell, while
/// a bounded unsigned error recurrence decides when to advance the minor axis.
///
/// The recurrence avoids floating-point arithmetic, division during iteration,
/// and doubled deltas that could overflow at the full `isize` coordinate range.
///
/// # Grid clipping
///
/// The complete integer line is rasterized in signed lattice space and cells
/// outside the grid are skipped. Empty grids and lines whose bounding boxes
/// are trivially disjoint from the grid produce no elements immediately.
///
/// Consequently, work is generally proportional to:
/// ```text
/// max(abs(end.x - start.x), abs(end.y - start.y)) + 1
/// ```
/// rather than only to the number of emitted cells.
///
/// This type represents an aliased line only. Antialiasing, stroke width,
/// caps, joins, paint, and compositing are separate operations.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RasterLineIter {
    grid: RasterGrid,
    position: Position2<isize>,
    end: Position2<isize>,

    major_delta: usize,
    minor_delta: usize,
    error: usize,

    major_axis: u8,
    minor_step: i8,
    forward: bool,
    finished: bool,
}

impl RasterLineIter {
    /* constructors */

    /// Creates an aliased raster-line iterator.
    ///
    /// `start` and `end` are signed raster-lattice positions. They may lie
    /// outside `grid`; only contained cells are emitted.
    pub const fn new(grid: RasterGrid, start: Position2<isize>, end: Position2<isize>) -> Self {
        let dx = abs_diff(start.dim[0], end.dim[0]);
        let dy = abs_diff(start.dim[1], end.dim[1]);
        let (major_axis, major_delta, minor_delta) =
            if dx >= dy { (0, dx, dy) } else { (1, dy, dx) };
        lets! { major = major_axis as usize, minor = 1 - major};
        // Canonical traversal always increases along the major axis.
        let forward = start.dim[major] <= end.dim[major];
        let (canonical_start, canonical_end) = is! { forward, (start, end), (end, start) };
        let minor_step = if canonical_start.dim[minor] < canonical_end.dim[minor] {
            1
        } else if canonical_start.dim[minor] > canonical_end.dim[minor] {
            -1
        } else {
            0
        };
        let finished = !line_bounds_intersect(grid, start, end);
        Self {
            grid,
            position: start,
            end,
            major_delta,
            minor_delta,
            error: major_delta / 2,
            major_axis,
            minor_step,
            forward,
            finished,
        }
    }

    /* queries */

    /// Returns the raster grid that clips this line.
    pub const fn grid(&self) -> RasterGrid {
        self.grid
    }
    /// Returns whether the iterator has reached its finished state.
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.finished
    }

    /* iteration */

    /// Advances the iterator and returns the next contained raster element.
    #[must_use]
    pub const fn next(&mut self) -> Option<RasterElement> {
        loop {
            is! { self.finished, return None }
            let position = self.position;
            is! { self.at_end(), self.finished = true, self.advance() }
            if let Some(coord) = self.grid.checked_coord(position) {
                return Some(RasterElement::full(coord));
            }
        }
    }

    /* private */

    const fn at_end(&self) -> bool {
        self.position.dim[0] == self.end.dim[0] && self.position.dim[1] == self.end.dim[1]
    }
    const fn advance(&mut self) {
        is! { self.forward, self.advance_forward(), self.advance_backward() }
    }

    /// Advances one step in canonical positive-major-axis order.
    const fn advance_forward(&mut self) {
        let major = self.major_axis as usize;
        // SAFE: the current major coordinate lies strictly before the endpoint.
        self.position.dim[major] += 1;
        is! { self.minor_delta == 0, return }
        // Equivalent to:
        //   error += minor_delta;
        //   if error >= major_delta { error -= major_delta; step minor; }
        // but arranged so the addition cannot overflow `usize`.
        let gap = self.major_delta - self.minor_delta;
        if self.error >= gap {
            self.error -= gap;
            self.step_minor(self.minor_step);
        } else {
            self.error += self.minor_delta;
        }
    }

    /// Advances one step against canonical positive-major-axis order.
    const fn advance_backward(&mut self) {
        let major = self.major_axis as usize;
        // SAFE: the current major coordinate lies strictly after the endpoint.
        self.position.dim[major] -= 1;
        is! { self.minor_delta == 0, return }
        // Exact inverse of `advance_forward`.
        if self.error < self.minor_delta {
            self.error += self.major_delta - self.minor_delta;
            self.step_minor(-self.minor_step);
        } else {
            self.error -= self.minor_delta;
        }
    }
    const fn step_minor(&mut self, step: i8) {
        let minor = 1 - self.major_axis as usize;
        if step > 0 {
            self.position.dim[minor] += 1;
        } else if step < 0 {
            self.position.dim[minor] -= 1;
        }
    }
    const fn candidate_upper_bound(&self) -> Option<usize> {
        is! { self.finished, return Some(0) }
        let major = self.major_axis as usize;
        abs_diff(self.position.dim[major], self.end.dim[major]).checked_add(1)
    }
}

/* helpers */

/// Returns the absolute difference without overflowing signed arithmetic.
const fn abs_diff(a: isize, b: isize) -> usize {
    is! { a <= b, (b as i128 - a as i128) as usize, (a as i128 - b as i128) as usize }
}

/// Returns whether the line's bounding box intersects the grid.
const fn line_bounds_intersect(
    grid: RasterGrid,
    start: Position2<isize>,
    end: Position2<isize>,
) -> bool {
    is! { grid.is_empty(), return false }
    let min_x = if start.dim[0] <= end.dim[0] { start.dim[0] } else { end.dim[0] };
    let max_x = if start.dim[0] >= end.dim[0] { start.dim[0] } else { end.dim[0] };
    let min_y = if start.dim[1] <= end.dim[1] { start.dim[1] } else { end.dim[1] };
    let max_y = if start.dim[1] >= end.dim[1] { start.dim[1] } else { end.dim[1] };
    is! { max_x < 0 || max_y < 0, return false }
    is! { min_x >= 0 && min_x as usize >= grid.width(), return false }
    is! { min_y >= 0 && min_y as usize >= grid.height(), return false }
    true
}

/* traits */

impl Iterator for RasterLineIter {
    type Item = RasterElement;

    fn next(&mut self) -> Option<Self::Item> {
        Self::next(self)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        // Clipping means no positive lower bound can be promised.
        (0, self.candidate_upper_bound())
    }
}
impl IteratorFused for RasterLineIter {}
