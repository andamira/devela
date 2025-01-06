// devela::data::collections::array::d2::tests

use super::*;

#[test]
fn indexing() {
    type XMAJ = Array2d<u32, 3, 3, { 3 * 3 }>;

    assert_eq!(XMAJ::get_index_unchecked([0, 0]), 0);
    assert_eq!(XMAJ::get_index_unchecked([1, 0]), 1);
    assert_eq!(XMAJ::get_index_unchecked([0, 1]), 3);
    assert_eq!(XMAJ::get_index_unchecked([2, 2]), 8);

    assert_eq!(XMAJ::get_coords_unchecked(0), [0, 0]);
    assert_eq!(XMAJ::get_coords_unchecked(1), [1, 0]);
    assert_eq!(XMAJ::get_coords_unchecked(3), [0, 1]);
    assert_eq!(XMAJ::get_coords_unchecked(8), [2, 2]);

    assert!(XMAJ::get_index([3, 2]).is_err());
    assert!(XMAJ::get_coords(9).is_err());

    type YMAJ = Array2d<u32, 3, 3, { 3 * 3 }, false>;

    assert_eq!(YMAJ::get_index_unchecked([0, 0]), 0);
    assert_eq!(YMAJ::get_index_unchecked([1, 0]), 3);
    assert_eq!(YMAJ::get_index_unchecked([0, 1]), 1);
    assert_eq!(YMAJ::get_index_unchecked([2, 2]), 8);

    assert_eq!(YMAJ::get_coords_unchecked(0), [0, 0]);
    assert_eq!(YMAJ::get_coords_unchecked(3), [1, 0]);
    assert_eq!(YMAJ::get_coords_unchecked(1), [0, 1]);
    assert_eq!(YMAJ::get_coords_unchecked(8), [2, 2]);

    assert!(YMAJ::get_index([3, 2]).is_err());
    assert!(YMAJ::get_coords(9).is_err());
}

#[test]
fn check() {
    // Initialize a 2D grid (3x3) in ROW MAJOR order
    // let xmaj = Array2d::<u32, (), 3, 3, { 3 * 3 }>::with_cloned(0).unwrap();

    // Initialize a 2D grid (3x3) in COLUMN MAJOR order
    // let ymaj = Array2d::<u32, (), 3, 3, { 3 * 3 }, false>::with_cloned(0).unwrap();
}
