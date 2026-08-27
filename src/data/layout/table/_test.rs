// devela/src/data/layout/table/_test.rs

use crate::{ArrayShape, TableCoord, TableLayout, TableShape};

#[test]
fn shape_semantics() {
    let shape = TableShape::new(2, 3);
    assert_eq!(shape.rows(), 2);
    assert_eq!(shape.columns(), 3);
    assert_eq!(shape.cell_count(), Ok(6));
    assert!(shape.contains(TableCoord::new(1, 2)));
    assert!(!shape.contains(TableCoord::new(2, 0)));
    assert!(!shape.contains(TableCoord::new(0, 3)));
}
#[test]
fn row_major_layout() {
    let layout = TableLayout::row_major(TableShape::new(2, 3)).unwrap();
    assert_eq!(layout.row_stride(), 3);
    assert_eq!(layout.column_stride(), 1);
    assert_eq!(layout.storage_index(TableCoord::new(0, 0)), Some(0));
    assert_eq!(layout.storage_index(TableCoord::new(0, 2)), Some(2));
    assert_eq!(layout.storage_index(TableCoord::new(1, 0)), Some(3));
    assert_eq!(layout.storage_index(TableCoord::new(1, 2)), Some(5));
}
#[test]
fn column_major_layout() {
    let layout = TableLayout::column_major(TableShape::new(2, 3)).unwrap();
    assert_eq!(layout.row_stride(), 1);
    assert_eq!(layout.column_stride(), 2);
    assert_eq!(layout.storage_index(TableCoord::new(0, 0)), Some(0));
    assert_eq!(layout.storage_index(TableCoord::new(1, 0)), Some(1));
    assert_eq!(layout.storage_index(TableCoord::new(0, 1)), Some(2));
    assert_eq!(layout.storage_index(TableCoord::new(1, 2)), Some(5));
}
#[test]
fn wrappers_preserve_array_representation() {
    let shape = TableShape::new(2, 3);
    assert_eq!(shape.as_array(), ArrayShape::new([2, 3]));
    let layout = TableLayout::row_major(shape).unwrap();
    assert_eq!(layout.as_array().strides(), &[3, 1]);
}
