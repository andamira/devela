// devela/src/media/visual/image/raster/grid/_test.rs

use super::*;
use crate::{Array, ArrayLayout, ArrayShape, Slice, const_assert};
use crate::{Extent2, Position2, RegionS2, ext, pos};

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
