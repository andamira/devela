// devela/src/media/visual/image/raster/draw/line.rs
//
//! Defines [`RasterLineIter`].
//
// > Which raster cells represent a one-cell-wide lattice line?

use crate::{IteratorFused, Position2, RasterElement, RasterGrid, is, lets, unwrap};

#[doc = crate::_tags!(image iterator)]
/// An iterator over the covered cells of an aliased raster line.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster/draw", struct RasterLineIter),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RasterLineIter = 68|544; niche Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RasterLineIter = 72|576; niche Option),
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
/// and doubled deltas that could overflow at the full `i64` raster-lattice range.
///
/// Constructor-time clipping uses widened integer arithmetic to jump directly
/// to the exact recurrence state at a selected major-axis step.
/// Iteration itself remains incremental and division-free.
///
/// # Grid clipping
///
/// The candidate major-axis interval is restricted to the portion overlapping the raster grid.
///
/// The iterator reconstructs the exact integer-rasterization state at the
/// clipped interval boundaries, so clipping does not restart or alter the
/// underlying line recurrence. The emitted sequence therefore remains identical
/// to rasterizing the complete line and discarding cells outside the grid.
///
/// Candidates outside the grid along the minor axis are still skipped during iteration.
///
/// After construction, candidate work is bounded by the grid extent along the
/// line's major axis: at most `width` candidates for an x-major line or
/// `height` candidates for a y-major line.
#[must_use]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RasterLineIter {
    grid: RasterGrid,
    position: Position2<i64>,
    end: Position2<i64>,

    major_delta: u64,
    minor_delta: u64,
    error: u64,

    major_axis: u8,
    minor_step: i8,
    forward: bool,
    finished: bool,
}

impl RasterLineIter {
    /* constructors */

    /// Creates an aliased raster-line iterator.
    ///
    /// `start` and `end` are signed raster-lattice positions and may lie outside `grid`.
    /// Emitted [`RasterElement`] coordinates are always contained `u32` raster-cell coordinates.
    pub const fn new(grid: RasterGrid, start: Position2<i64>, end: Position2<i64>) -> Self {
        let dx = start.dim[0].abs_diff(end.dim[0]);
        let dy = start.dim[1].abs_diff(end.dim[1]);
        let (major_axis, major_delta, minor_delta) = is![dx >= dy, (0, dx, dy), (1, dy, dx)];
        lets! { major = major_axis as usize, minor = 1 - major };
        // Canonical traversal always increases along the major axis.
        let forward = start.dim[major] <= end.dim[major];
        let (canonical_start, canonical_end) = is![forward, (start, end), (end, start)];
        let minor_step = is![
            canonical_start.dim[minor] < canonical_end.dim[minor],
            1,
            is![canonical_start.dim[minor] > canonical_end.dim[minor], -1, 0]
        ];
        lets! { mut position = start, mut iter_end = end, mut error = major_delta / 2 }
        let mut finished = !line_bounds_intersect(grid, start, end);
        if !finished {
            if let Some((first_step, last_step)) =
                grid_major_step_range(grid, canonical_start, canonical_end, major_axis)
            {
                let (first_position, first_error) = state_at(
                    canonical_start,
                    major_axis,
                    major_delta,
                    minor_delta,
                    minor_step,
                    first_step,
                );
                let (last_position, last_error) = state_at(
                    canonical_start,
                    major_axis,
                    major_delta,
                    minor_delta,
                    minor_step,
                    last_step,
                );
                if forward {
                    position = first_position;
                    iter_end = last_position;
                    error = first_error;
                } else {
                    position = last_position;
                    iter_end = first_position;
                    error = last_error;
                }
            } else {
                finished = true;
            }
        }
        Self {
            grid,
            position,
            end: iter_end,
            major_delta,
            minor_delta,
            error,
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
        // Safe: current major coordinate is strictly before the endpoint.
        self.position.dim[major] += 1;
        is! { self.minor_delta == 0, return }
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
        // Safe: current major coordinate is strictly after the endpoint.
        self.position.dim[major] -= 1;
        is! { self.minor_delta == 0, return }
        if self.error < self.minor_delta {
            self.error += self.major_delta - self.minor_delta;
            self.step_minor(-self.minor_step);
        } else {
            self.error -= self.minor_delta;
        }
    }
    const fn step_minor(&mut self, step: i8) {
        let minor = 1 - self.major_axis as usize;
        is![step > 0, self.position.dim[minor] += 1, is![step < 0, self.position.dim[minor] -= 1]];
    }
    const fn candidate_upper_bound(&self) -> Option<usize> {
        is! { self.finished, return Some(0) }
        let major = self.major_axis as usize;
        let delta = self.position.dim[major].abs_diff(self.end.dim[major]);
        let count = unwrap![some? delta.checked_add(1)];
        is! { count > usize::MAX as u64, None, Some(count as usize) }
    }
}

/* helpers */

/// Returns whether the line's bounding box intersects the grid.
const fn line_bounds_intersect(
    grid: RasterGrid,
    start: Position2<i64>,
    end: Position2<i64>,
) -> bool {
    is! { grid.is_empty(), return false }
    let min_x = if start.dim[0] <= end.dim[0] { start.dim[0] } else { end.dim[0] };
    let max_x = if start.dim[0] >= end.dim[0] { start.dim[0] } else { end.dim[0] };
    let min_y = if start.dim[1] <= end.dim[1] { start.dim[1] } else { end.dim[1] };
    let max_y = if start.dim[1] >= end.dim[1] { start.dim[1] } else { end.dim[1] };
    is! { max_x < 0 || max_y < 0, return false }
    is! { min_x >= grid.width() as i64, return false }
    is! { min_y >= grid.height() as i64, return false }
    true
}
/// Returns the canonical major-step interval that can overlap the grid.
const fn grid_major_step_range(
    grid: RasterGrid,
    canonical_start: Position2<i64>,
    canonical_end: Position2<i64>,
    major_axis: u8,
) -> Option<(u64, u64)> {
    let major = major_axis as usize;
    let extent = if major == 0 { grid.width() } else { grid.height() };
    is! { extent == 0, return None }
    let start = canonical_start.dim[major];
    let end = canonical_end.dim[major];
    let grid_end = extent as i64 - 1;
    is! { end < 0 || start > grid_end, return None } // Canonical major coords always increase
    let first = is! { start < 0, start.abs_diff(0), 0 };
    let last = is! { end > grid_end, start.abs_diff(grid_end), start.abs_diff(end) };
    Some((first, last))
}
/// Reconstructs the exact canonical line state after `step` major steps.
const fn state_at(
    canonical_start: Position2<i64>,
    major_axis: u8,
    major_delta: u64,
    minor_delta: u64,
    minor_step: i8,
    step: u64,
) -> (Position2<i64>, u64) {
    is! { major_delta == 0, return (canonical_start, 0) }
    lets! { major = major_axis as usize, minor = 1 - major }
    // `u128` makes the full i64 lattice safe.
    let total = (major_delta / 2) as u128 + step as u128 * minor_delta as u128;
    let minor_steps = total / major_delta as u128;
    let error = (total % major_delta as u128) as u64;
    let mut dim = canonical_start.dim;
    dim[major] = (dim[major] as i128 + step as i128) as i64;
    dim[minor] = (dim[minor] as i128 + minor_step as i128 * minor_steps as i128) as i64;
    (Position2::new(dim), error)
}

/* traits */

impl Iterator for RasterLineIter {
    type Item = RasterElement;

    fn next(&mut self) -> Option<Self::Item> {
        Self::next(self)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.candidate_upper_bound())
    }
}
impl IteratorFused for RasterLineIter {}
