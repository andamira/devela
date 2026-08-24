// devela/src/media/visual/image/raster/grid/_test.rs

use super::*;
use crate::{Array, ArrayLayout};
use crate::{Extent2, Position2, RegionS2, Slice, const_assert, ext, lets, pos, unwrap};

const GRID: RasterGrid = RasterGrid::new(ext![3_u32, 2]);
const EXTENT: Extent2<u32> = GRID.extent();
const BOUNDS: RegionS2<u32> = GRID.bounds();
const CHECKED_COORD: Position2<u32> =
    unwrap![some_expect GRID.checked_coord(pos![2_i64, 1]), "expected contained coordinate"];
const CELL_BOUNDS: RegionS2<u32> =
    unwrap![some_expect GRID.cell_bounds(pos![2_u32, 1]), "expected contained cell"];
const COORDS: [Position2<u32>; 6] = {
    let mut iter = GRID.coords();
    let mut coords = [Position2::new([0, 0]); 6];
    let mut index = 0;
    while let Some(coord) = iter.next() {
        coords[index] = coord;
        index += 1;
    }
    coords
};

#[test]
const fn construction_and_queries_are_const() {
    const_assert!(Slice::<u32>::eq(&EXTENT.dim, &[3, 2]));
    const_assert!(eq GRID.width(), 3);
    const_assert!(eq GRID.height(), 2);
    const_assert!(eq GRID.cell_count(), 6);
    const_assert!(!GRID.is_empty());
    const_assert!(Slice::<u32>::eq(&BOUNDS.pos.dim, &[0, 0]));
    const_assert!(Slice::<u32>::eq(&BOUNDS.ext.dim, &[3, 2]));
    const_assert!(GRID.contains(Position2::new([0, 0])));
    const_assert!(GRID.contains(Position2::new([2, 1])));
    const_assert!(!GRID.contains(Position2::new([3, 1])));
    const_assert!(!GRID.contains(Position2::new([2, 2])));
    const_assert!(Slice::<u32>::eq(&CHECKED_COORD.dim, &[2, 1],));
    const_assert!(Slice::<u32>::eq(&CELL_BOUNDS.pos.dim, &[2, 1],));
    const_assert!(Slice::<u32>::eq(&CELL_BOUNDS.ext.dim, &[1, 1],));
}
#[test]
fn coordinates_are_x_fastest() {
    assert_eq!(COORDS, [pos![0, 0], pos![1, 0], pos![2, 0], pos![0, 1], pos![1, 1], pos![2, 1]]);
}
#[test]
fn coordinate_iterator_is_double_ended() {
    let mut coords = GRID.coords();
    assert_eq!(coords.remaining(), 6);
    assert_eq!(coords.next(), Some(pos![0, 0]));
    assert_eq!(coords.next_back(), Some(pos![2, 1]));
    assert_eq!(coords.next(), Some(pos![1, 0]));
    assert_eq!(coords.next_back(), Some(pos![1, 1]));
    assert_eq!(coords.remaining(), 2);
    assert_eq!(coords.next(), Some(pos![2, 0]));
    assert_eq!(coords.next_back(), Some(pos![0, 1]));
    assert!(coords.is_empty());
}
#[test]
fn empty_extents_are_valid() {
    for extent in [
        Extent2::new([0, 0]),
        Extent2::new([0, 7]),
        Extent2::new([9, 0]),
        Extent2::new([0, u32::MAX]),
        Extent2::new([u32::MAX, 0]),
    ] {
        let grid = RasterGrid::new(extent);
        assert_eq!(grid.extent(), extent);
        assert_eq!(grid.cell_count(), 0);
        assert!(grid.is_empty());
        assert!(grid.coords().is_empty());
    }
}
#[test]
fn full_u32_extent_is_representable() {
    let grid = RasterGrid::new(Extent2::new([u32::MAX, u32::MAX]));
    assert_eq!(grid.cell_count(), u32::MAX as u64 * u32::MAX as u64);
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
fn cell_indices_are_x_fastest_and_reversible() {
    assert_eq!(GRID.cell_index(pos![0_u32, 0]), Some(0));
    assert_eq!(GRID.cell_index(pos![1_u32, 0]), Some(1));
    assert_eq!(GRID.cell_index(pos![2_u32, 0]), Some(2));
    assert_eq!(GRID.cell_index(pos![0_u32, 1]), Some(3));
    assert_eq!(GRID.cell_index(pos![2_u32, 1]), Some(5));
    assert_eq!(GRID.coord_at(0), Some(pos![0_u32, 0]));
    assert_eq!(GRID.coord_at(3), Some(pos![0_u32, 1]));
    assert_eq!(GRID.coord_at(5), Some(pos![2_u32, 1]));
    assert_eq!(GRID.coord_at(6), None);
}
#[test]
fn canonical_index_matches_dense_first_array_storage() {
    let grid = RasterGrid::new(ext![3_u32, 2]);
    let shape = grid.try_array_shape().unwrap();
    let layout = ArrayLayout::dense_first(shape).unwrap();
    for coord in grid.coords() {
        let [x, y] = coord.dim;
        assert_eq!(
            layout.storage_index([x as usize, y as usize]),
            Some(grid.cell_index(coord).unwrap() as usize),
        );
    }
}
#[test]
fn every_coordinate_round_trips_through_its_index() {
    for coord in GRID.coords() {
        let index = GRID.cell_index(coord).unwrap();
        assert_eq!(GRID.coord_at(index), Some(coord));
    }
}
#[test]
fn full_u32_grid_indices_remain_exact() {
    let grid = RasterGrid::new(ext![u32::MAX, u32::MAX]);
    let last = pos![u32::MAX - 1, u32::MAX - 1];
    assert_eq!(grid.cell_index(last), Some(grid.cell_count() - 1));
    assert_eq!(grid.coord_at(grid.cell_count() - 1), Some(last));
}
#[test]
fn traversal_consumes_into_array_coordinates() {
    let grid = RasterGrid::new(Extent2::new([3, 2]));
    let shape = grid.try_array_shape().unwrap();
    let layout = ArrayLayout::dense_first(shape).unwrap();
    let mut storage = [0_u8; 6];
    let mut array = Array::try_from_slice_mut(&mut storage, layout).unwrap();
    for coord in grid.coords() {
        lets! { [x, y] = coord.dim, array_coord = [x as usize, y as usize] }
        *array.get_mut(array_coord).unwrap() = (x + y * 10) as u8;
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
    let grid = RasterGrid::new(Extent2::new([3, 2]));
    let shape = grid.try_array_shape().unwrap();
    let layout = ArrayLayout::dense_last(shape).unwrap();
    let mut storage = [0_u8; 6];
    let mut array = Array::try_from_slice_mut(&mut storage, layout).unwrap();
    for coord in grid.coords() {
        lets! { [x, y] = coord.dim, array_coord = [x as usize, y as usize] }
        *array.get_mut(array_coord).unwrap() = (x + y * 10) as u8;
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
#[test]
fn traversal_projects_into_array_coordinates() {
    let grid = RasterGrid::new(Extent2::new([3, 2]));
    let shape = grid.try_array_shape().unwrap();
    let layout = ArrayLayout::dense_first(shape).unwrap();
    let mut storage = [0_u8; 6];
    let mut array = Array::try_from_slice_mut(&mut storage, layout).unwrap();
    for coord in grid.coords() {
        lets! { [x, y] = coord.dim, array_coord = [x as usize, y as usize] }
        *array.get_mut(array_coord).unwrap() = (x + y * 10) as u8;
    }
    assert_eq!(array.storage(), &[0, 1, 2, 10, 11, 12]);
}
#[test]
fn small_grid_projects_to_array_shape() {
    let grid = RasterGrid::new(Extent2::new([3, 2]));
    let shape = grid.try_array_shape().unwrap();
    assert_eq!(shape.lengths(), &[3, 2]);
}
#[test]
#[cfg(target_pointer_width = "32")]
fn array_projection_preserves_unaddressable_shape() {
    let grid = RasterGrid::new(Extent2::new([u32::MAX, 2]));
    let shape = grid.try_array_shape().unwrap();
    assert_eq!(shape.lengths(), &[usize::MAX, 2]);
    assert!(shape.element_count().is_err());
    assert!(ArrayLayout::dense_first(shape).is_err());
}
#[cfg(target_pointer_width = "32")]
#[test]
fn array_shape_projection_does_not_imply_addressability() {
    let grid = RasterGrid::new(Extent2::new([u32::MAX, 2]));
    let shape = grid.try_array_shape().unwrap();
    assert_eq!(shape.lengths(), &[u32::MAX as usize, 2]);
    assert!(shape.element_count().is_err());
    assert!(ArrayLayout::dense_first(shape).is_err());
}
