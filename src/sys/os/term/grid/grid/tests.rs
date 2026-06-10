// devela::sys::os::term::grid::grid::definition

use crate::TermGrid;
use crate::TermGridError;
use crate::{ext, pos};

#[test]
fn construction_and_geometry() {
    let storage = [0u8; 16];
    let grid = TermGrid::new(storage, ext!(3usize, 4usize)).unwrap();
    assert_eq!(grid.extent(), ext!(3, 4));
    assert_eq!(grid.width(), 3);
    assert_eq!(grid.height(), 4);
    assert_eq!(grid.len(), 12);
    assert_eq!(grid.spare_len(), 4);
    assert_eq!(grid.as_slice(), &[0; 12]);
}
#[test]
fn insufficient_storage() {
    let error = TermGrid::new([0u8; 5], ext!(3usize, 2usize)).unwrap_err();
    assert_eq!(error, TermGridError::NotEnoughElements { required: 6, available: 5 },);
}
#[test]
fn coordinate_mapping() {
    let grid = TermGrid::new([0u8; 12], ext!(4usize, 3usize)).unwrap();
    assert_eq!(grid.index_of(pos!(0usize, 0usize)), Some(0));
    assert_eq!(grid.index_of(pos!(3usize, 2usize)), Some(11));
    assert_eq!(grid.index_of(pos!(4usize, 2usize)), None);
    assert_eq!(grid.position_of(0), Some(pos!(0usize, 0usize)));
    assert_eq!(grid.position_of(11), Some(pos!(3usize, 2usize)));
    assert_eq!(grid.position_of(12), None);
}
#[test]
fn access_and_rows() {
    let mut grid = TermGrid::new([0, 1, 2, 3, 4, 5], ext!(3usize, 2usize)).unwrap();
    assert_eq!(grid.get_xy(1, 0), Some(&1));
    assert_eq!(grid.get_xy(2, 1), Some(&5));
    assert_eq!(grid.get_xy(3, 0), None);
    assert_eq!(grid.row(0), Some(&[0, 1, 2][..]));
    assert_eq!(grid.row(1), Some(&[3, 4, 5][..]));
    assert_eq!(grid.row(2), None);
    assert!(grid.set_xy(1, 1, 9));
    assert!(!grid.set_xy(3, 1, 9));
    assert_eq!(grid.row(1), Some(&[3, 9, 5][..]));
}
#[test]
fn fill_only_affects_active_grid() {
    let mut grid = TermGrid::new([1u8; 8], ext!(3usize, 2usize)).unwrap();
    grid.fill(7);
    assert_eq!(grid.as_slice(), &[7; 6]);
    assert_eq!(grid.storage_slice(), &[7, 7, 7, 7, 7, 7, 1, 1]);
}
