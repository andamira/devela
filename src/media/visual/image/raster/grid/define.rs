// devela/src/media/visual/image/raster/grid/define.rs
//
//! Defines [`RasterGrid`].
//
// > What do pixel coordinates mean geometrically?

use crate::is;
use crate::{ArrayShape, Extent2, Overflow, Position2, RasterCoordIter, RegionS2};

#[doc = crate::_tags!(image layout)]
/// A finite logical grid of canonical raster cells.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster/grid", struct RasterGrid),
    test_size_of(RasterGrid = 8|64; niche !Option),
}]
/// `RasterGrid` defines the logical geometry of a two-dimensional raster,
/// independently of its samples and physical storage.
///
/// Its canonical coordinate system has:
/// - the origin at the upper-left boundary;
/// - positive `x` extending rightward;
/// - positive `y` extending downward;
/// - an extent ordered as `[width, height]`;
/// - `u32` raster-cell coordinates;
/// - valid cell coordinates satisfying `x < width` and `y < height`.
///
/// The cell at coordinate `[x, y]` occupies the half-open boundary-space region:
/// ```text
/// [x, x + 1) × [y, y + 1)
/// ```
///
/// Its conceptual center is:
/// ```text
/// [x + 1/2, y + 1/2]
/// ```
///
/// Signed raster-lattice positions used by rasterization and clipping are
/// distinct from contained raster-cell coordinates.
/// [`checked_coord`][Self::checked_coord] validates
/// and projects such positions into the grid.
///
/// Raster geometry is also independent of machine-addressable array geometry.
/// [`try_array_shape`][Self::try_array_shape] provides an explicit projection
/// into an [`ArrayShape`] when its dimensions are representable there.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterGrid {
    extent: Extent2<u32>,
}

impl RasterGrid {
    /* construction */

    /// Creates a logical raster grid with the given extent.
    ///
    /// Empty grids are valid.
    pub const fn new(extent: Extent2<u32>) -> Self {
        Self { extent }
    }

    /* dimensions */

    /// Returns the logical raster extent as `[width, height]`.
    ///
    /// Axis `0` is `x`/width and axis `1` is `y`/height.
    pub const fn extent(&self) -> Extent2<u32> {
        self.extent
    }
    /// Returns the raster width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.extent.dim[0]
    }
    /// Returns the raster height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.extent.dim[1]
    }
    /// Returns the exact number of logical raster cells.
    #[must_use]
    pub const fn cell_count(&self) -> u64 {
        self.width() as u64 * self.height() as u64
    }
    /// Returns whether the grid contains no raster cells.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.width() == 0 || self.height() == 0
    }

    /* regions and coordinates */

    /// Returns the complete logical raster bounds.
    ///
    /// The returned region begins at `[0, 0]` and has the grid's extent.
    pub const fn bounds(&self) -> RegionS2<u32> {
        RegionS2::new(Position2::new([0, 0]), self.extent)
    }
    /// Returns whether `coord` identifies a cell in this grid.
    #[must_use]
    pub const fn contains(&self, coord: Position2<u32>) -> bool {
        coord.dim[0] < self.width() && coord.dim[1] < self.height()
    }
    /// Converts a signed raster-lattice position into a contained raster coordinate.
    ///
    /// Negative and external positions are rejected.
    #[must_use]
    pub const fn checked_coord(&self, position: Position2<i64>) -> Option<Position2<u32>> {
        let [x, y] = position.dim;
        is! { x < 0 || y < 0 || x > u32::MAX as i64 || y > u32::MAX as i64, return None }
        let coord = Position2::new([x as u32, y as u32]);
        is! { self.contains(coord), Some(coord), None }
    }
    /// Returns the unit boundary-space region occupied by `coord`.
    ///
    /// Returns `None` when `coord` is outside the grid.
    #[must_use]
    pub const fn cell_bounds(&self, coord: Position2<u32>) -> Option<RegionS2<u32>> {
        is! { !self.contains(coord), return None }
        Some(RegionS2::new(coord, Extent2::new([1, 1])))
    }

    /* indexing */

    /// Returns the canonical logical index of `coord`.
    ///
    /// Raster indices are x-fast and independent of physical storage:
    ///
    /// ```text
    /// index = y * width + x
    /// ```
    ///
    /// The returned `u64` can represent every cell of every `u32 × u32`
    /// raster grid.
    ///
    /// Returns `None` when `coord` lies outside the grid.
    #[must_use]
    pub const fn cell_index(&self, coord: Position2<u32>) -> Option<u64> {
        is! { !self.contains(coord), return None }
        let [x, y] = coord.dim;
        Some(y as u64 * self.width() as u64 + x as u64)
    }
    /// Returns the coordinate at the canonical logical `index`.
    ///
    /// This is the inverse of [`cell_index`][Self::cell_index].
    ///
    /// Returns `None` when `index >= self.cell_count()`.
    #[must_use]
    pub const fn coord_at(&self, index: u64) -> Option<Position2<u32>> {
        is! { index >= self.cell_count(), return None }
        // `index < cell_count()` implies a non-zero width.
        let width = self.width() as u64;
        let (x, y) = (index % width, index / width);
        Some(Position2::new([x as u32, y as u32]))
    }

    /* traversal */

    /// Returns an exhaustive iterator over the raster-cell coordinates.
    ///
    /// Coordinates are yielded as [`Position2<u32>`] values in canonical
    /// raster order, with `x` changing fastest:
    ///
    /// ```text
    /// [0, 0], [1, 0], …, [width - 1, 0],
    /// [0, 1], [1, 1], …
    /// ```
    ///
    /// Traversal is independent of sample storage, physical layout,
    /// and machine-addressable array coordinates.
    pub const fn coords(&self) -> RasterCoordIter {
        RasterCoordIter::new(*self)
    }

    /* projections */

    /// Projects this raster grid into a two-dimensional [`ArrayShape`].
    ///
    /// Raster coordinates and extents use `u32`, independently of machine
    /// pointer width. [`ArrayShape`] uses `usize` axis lengths, so this
    /// conversion is fallible when either raster dimension cannot be
    /// represented as a `usize`.
    ///
    /// Axis `0` corresponds to raster `x`/width and axis `1` to `y`/height.
    ///
    /// This projection describes only the logical array shape. It does not
    /// choose a storage order or construct an [`ArrayLayout`][crate::ArrayLayout].
    /// In particular, the resulting shape's total [`element_count`][ArrayShape::element_count]
    /// may still be unrepresentable as a `usize`.
    ///
    /// # Errors
    /// Returns [`Overflow`] if either raster dimension cannot be represented as a `usize`.
    pub const fn try_array_shape(&self) -> Result<ArrayShape<2>, Overflow> {
        let [width, height] = self.extent.dim;
        if width as u64 > usize::MAX as u64 || height as u64 > usize::MAX as u64 {
            return Err(Overflow(None));
        }
        Ok(ArrayShape::new([width as usize, height as usize]))
    }
}
