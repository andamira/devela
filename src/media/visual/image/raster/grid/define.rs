// devela/src/media/visual/image/raster/grid/define.rs
//
//! Defines [`RasterGrid`].
//
// > What do pixel coordinates mean geometrically?

use crate::{ArrayCoordIter, ArrayShape, Extent2, Overflow, Position2, RegionS2};
use crate::{is, unwrap};

#[doc = crate::_tags!(image layout)]
/// A finite logical grid of canonical raster cells.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster/grid"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RasterGrid = 8|64),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RasterGrid = 16|128),
}]
/// `RasterGrid` defines the logical geometry of a two-dimensional raster,
/// independently of its samples and physical storage.
///
/// Its canonical coordinate system has:
/// - the origin at the upper-left boundary;
/// - positive `x` extending rightward;
/// - positive `y` extending downward;
/// - an extent ordered as `[width, height]`;
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
/// The corresponding [`ArrayShape`] uses axis `0` for `x` and axis `1` for `y`.
/// [`coords`][Self::coords] therefore traverses `x` fastest.
///
/// # Invariant
///
/// The total cell count is representable as a `usize`.
/// [`try_new`][Self::try_new] establishes this invariant.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterGrid {
    shape: ArrayShape<2>,
}

impl RasterGrid {
    /* constructors */

    /// Creates a raster grid with the given logical extent.
    ///
    /// Empty grids are valid. A zero width or height produces no cells,
    /// regardless of the other dimension.
    ///
    /// # Errors
    ///
    /// Returns [`Overflow`] if `width × height` is not representable
    /// as a `usize`.
    pub const fn try_new(extent: Extent2<usize>) -> Result<Self, Overflow> {
        let shape = ArrayShape::new(extent.dim);
        unwrap![=ok_map shape.element_count(), |__| Self { shape }]
    }

    /* dimensions */

    /// Returns the logical raster extent as `[width, height]`.
    pub const fn extent(&self) -> Extent2<usize> {
        Extent2::new(self.shape.into_lengths())
    }
    /// Returns the logical raster extent as an array shape.
    ///
    /// Axis `0` is `x`/width and axis `1` is `y`/height.
    pub const fn shape(&self) -> ArrayShape<2> {
        self.shape
    }
    /// Returns the raster width.
    #[must_use]
    pub const fn width(&self) -> usize {
        self.shape.lengths()[0]
    }
    /// Returns the raster height.
    #[must_use]
    pub const fn height(&self) -> usize {
        self.shape.lengths()[1]
    }
    /// Returns the number of raster cells.
    ///
    /// Every safely constructed grid has a representable cell count.
    #[must_use]
    pub const fn cell_count(&self) -> usize {
        unwrap![ok_expect self.shape.element_count(), "invalid RasterGrid cell-count invariant"]
    }
    /// Returns whether the grid contains no raster cells.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.shape.is_empty()
    }

    /* regions and coordinates */

    /// Returns the complete logical raster bounds.
    ///
    /// The returned region begins at `[0, 0]` and has the grid's extent.
    pub const fn bounds(&self) -> RegionS2<usize> {
        RegionS2::new(Position2::new([0, 0]), self.extent())
    }
    /// Returns whether `coord` identifies a cell in this grid.
    #[must_use]
    pub const fn contains(&self, coord: Position2<usize>) -> bool {
        self.shape.contains_coord(coord.dim)
    }
    /// Converts a signed raster-lattice position into a valid cell coordinate.
    ///
    /// Negative positions and positions outside the grid are rejected.
    ///
    /// This performs only integer-lattice validation. It does not convert
    /// continuous geometric points or apply rounding, flooring, sampling,
    /// scaling, or an affine transformation.
    #[must_use]
    pub const fn checked_coord(&self, position: Position2<isize>) -> Option<Position2<usize>> {
        let [x, y] = position.dim;
        is! { x < 0 || y < 0, return None }
        let coord = Position2::new([x as usize, y as usize]);
        if self.contains(coord) { Some(coord) } else { None }
    }
    /// Returns the unit boundary-space region occupied by `coord`.
    ///
    /// Returns `None` when `coord` is outside the grid.
    #[must_use]
    pub const fn cell_bounds(&self, coord: Position2<usize>) -> Option<RegionS2<usize>> {
        is! { !self.contains(coord), return None }
        Some(RegionS2::new(coord, Extent2::new([1, 1])))
    }

    /* traversal */

    /// Returns an exhaustive iterator over the raster cell coordinates.
    ///
    /// Coordinates are returned as `[x, y]`, matching the coordinate input
    /// expected by [`Array`][crate::Array].
    ///
    /// `x` changes fastest:
    ///
    /// ```text
    /// [0, 0], [1, 0], …, [width - 1, 0],
    /// [0, 1], [1, 1], …
    /// ```
    ///
    /// The iterator is independent of sample storage and physical layout.
    pub const fn coords(&self) -> ArrayCoordIter<2> {
        ArrayCoordIter::new(self.shape, self.cell_count())
    }
}

#[cfg(test)]
mod _test {
    use super::*;
    use crate::{Array, ArrayLayout, Slice, const_assert, ext, pos};

    const GRID: RasterGrid = match RasterGrid::try_new(ext![3, 2]) {
        Ok(grid) => grid,
        Err(_) => panic!("unexpected RasterGrid overflow"),
    };
    const EXTENT: Extent2<usize> = GRID.extent();
    const SHAPE: ArrayShape<2> = GRID.shape();
    const BOUNDS: RegionS2<usize> = GRID.bounds();
    const CHECKED_COORD: Position2<usize> = match GRID.checked_coord(pos![2, 1]) {
        Some(coord) => coord,
        None => panic!("expected contained coordinate"),
    };
    const CELL_BOUNDS: RegionS2<usize> = match GRID.cell_bounds(pos![2, 1]) {
        Some(bounds) => bounds,
        None => panic!("expected contained cell"),
    };

    const COORDS: [[usize; 2]; 6] = {
        let mut iter = GRID.coords();
        let mut coords = [[0; 2]; 6];
        let mut index = 0;
        while let Some(coord) = iter.next() {
            coords[index] = coord;
            index += 1;
        }
        coords
    };

    #[test]
    const fn construction_and_queries_are_const() {
        const_assert!(Slice::<usize>::eq(&EXTENT.dim, &[3, 2]));
        const_assert!(Slice::<usize>::eq(SHAPE.lengths(), &[3, 2]));
        const_assert!(eq GRID.width(), 3);
        const_assert!(eq GRID.height(), 2);
        const_assert!(eq GRID.cell_count(), 6);
        const_assert!(!GRID.is_empty());
        const_assert!(Slice::<usize>::eq(&BOUNDS.pos.dim, &[0, 0]));
        const_assert!(Slice::<usize>::eq(&BOUNDS.ext.dim, &[3, 2]));
        const_assert!(GRID.contains(Position2::new([0, 0])));
        const_assert!(GRID.contains(Position2::new([2, 1])));
        const_assert!(!GRID.contains(Position2::new([3, 1])));
        const_assert!(!GRID.contains(Position2::new([2, 2])));
        const_assert!(Slice::<usize>::eq(&CHECKED_COORD.dim, &[2, 1],));
        const_assert!(Slice::<usize>::eq(&CELL_BOUNDS.pos.dim, &[2, 1],));
        const_assert!(Slice::<usize>::eq(&CELL_BOUNDS.ext.dim, &[1, 1],));
    }
    #[test]
    fn coordinates_are_x_fastest() {
        assert_eq!(COORDS, [[0, 0], [1, 0], [2, 0], [0, 1], [1, 1], [2, 1],],);
    }
    #[test]
    fn coordinate_iterator_is_double_ended() {
        let mut coords = GRID.coords();
        assert_eq!(coords.len(), 6);
        assert_eq!(coords.next(), Some([0, 0]));
        assert_eq!(coords.next_back(), Some([2, 1]));
        assert_eq!(coords.next(), Some([1, 0]));
        assert_eq!(coords.next_back(), Some([1, 1]));
        assert_eq!(coords.len(), 2);
        assert_eq!(coords.next(), Some([2, 0]));
        assert_eq!(coords.next_back(), Some([0, 1]));
        assert!(coords.is_empty());
        assert_eq!(coords.next(), None);
        assert_eq!(coords.next_back(), None);
    }
    #[test]
    fn empty_extents_are_valid() {
        for extent in [
            Extent2::new([0, 0]),
            Extent2::new([0, 7]),
            Extent2::new([9, 0]),
            Extent2::new([0, usize::MAX]),
            Extent2::new([usize::MAX, 0]),
        ] {
            let grid = RasterGrid::try_new(extent).unwrap();
            assert_eq!(grid.extent(), extent);
            assert_eq!(grid.cell_count(), 0);
            assert!(grid.is_empty());
            assert!(grid.coords().is_empty());
            assert!(!grid.contains(Position2::new([0, 0])));
        }
    }
    #[test]
    fn rejects_unrepresentable_cell_count() {
        assert!(RasterGrid::try_new(Extent2::new([usize::MAX, 2]),).is_err());
    }
    #[test]
    fn checked_coord_rejects_negative_and_external_positions() {
        assert_eq!(GRID.checked_coord(Position2::new([0, 0])), Some(Position2::new([0, 0])),);
        assert_eq!(GRID.checked_coord(Position2::new([2, 1])), Some(Position2::new([2, 1])),);
        assert_eq!(GRID.checked_coord(Position2::new([-1, 0])), None,);
        assert_eq!(GRID.checked_coord(Position2::new([0, -1])), None,);
        assert_eq!(GRID.checked_coord(Position2::new([3, 0])), None,);
        assert_eq!(GRID.checked_coord(Position2::new([0, 2])), None,);
    }
    #[test]
    fn cell_bounds_are_unit_regions() {
        assert_eq!(
            GRID.cell_bounds(Position2::new([0, 0])),
            Some(RegionS2::new(Position2::new([0, 0]), Extent2::new([1, 1]),)),
        );
        assert_eq!(
            GRID.cell_bounds(Position2::new([2, 1])),
            Some(RegionS2::new(Position2::new([2, 1]), Extent2::new([1, 1]),)),
        );
        assert_eq!(GRID.cell_bounds(Position2::new([3, 1])), None,);
        assert_eq!(GRID.cell_bounds(Position2::new([2, 2])), None,);
    }
    #[test]
    fn traversal_consumes_into_array_coordinates() {
        let grid = RasterGrid::try_new(Extent2::new([3, 2])).unwrap();
        let layout = ArrayLayout::dense_first(grid.shape()).unwrap();
        let mut storage = [0_u8; 6];
        let mut array = Array::try_from_slice_mut(&mut storage, layout).unwrap();
        for coord in grid.coords() {
            let [x, y] = coord;
            *array.get_mut(coord).unwrap() = (x + y * 10) as u8;
        }
        assert_eq!(array.get([0, 0]), Some(&0));
        assert_eq!(array.get([1, 0]), Some(&1));
        assert_eq!(array.get([2, 0]), Some(&2));
        assert_eq!(array.get([0, 1]), Some(&10));
        assert_eq!(array.get([1, 1]), Some(&11));
        assert_eq!(array.get([2, 1]), Some(&12));
        assert_eq!(array.storage(), &[0, 1, 2, 10, 11, 12]);
    }
    #[test]
    fn traversal_is_independent_of_physical_array_order() {
        let grid = RasterGrid::try_new(Extent2::new([3, 2])).unwrap();
        let layout = ArrayLayout::dense_last(grid.shape()).unwrap();
        let mut storage = [0_u8; 6];
        let mut array = Array::try_from_slice_mut(&mut storage, layout).unwrap();
        for coord in grid.coords() {
            let [x, y] = coord;
            *array.get_mut(coord).unwrap() = (x + y * 10) as u8;
        }
        // Logical values are unchanged.
        assert_eq!(array.get([0, 0]), Some(&0));
        assert_eq!(array.get([1, 0]), Some(&1));
        assert_eq!(array.get([2, 0]), Some(&2));
        assert_eq!(array.get([0, 1]), Some(&10));
        assert_eq!(array.get([1, 1]), Some(&11));
        assert_eq!(array.get([2, 1]), Some(&12));
        // Physical storage follows the distinct dense-last mapping.
        assert_eq!(array.storage(), &[0, 10, 1, 11, 2, 12]);
    }
}
