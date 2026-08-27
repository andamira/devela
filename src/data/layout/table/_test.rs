// devela/src/data/layout/table/_test.rs

use crate::{ArrayShape, TableCoord, TableLayout, TableShape};

/* shape */

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
fn overflowing_shape_rejects_exact_iteration() {
    let shape = TableShape::new(usize::MAX, 2);
    assert!(shape.try_coords().is_err());
}
#[test]
fn wrappers_preserve_array_representation() {
    let shape = TableShape::new(2, 3);
    assert_eq!(shape.as_array(), ArrayShape::new([2, 3]));
    let layout = TableLayout::row_major(shape).unwrap();
    assert_eq!(layout.as_array().strides(), &[3, 1]);
}

/* layout */

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

/* coords */

#[test]
fn coordinate_double_ended() {
    let mut iter = TableShape::new(2, 3).try_coords().unwrap();
    assert_eq!(iter.len(), 6);
    assert_eq!(iter.next(), Some(TableCoord::new(0, 0)));
    assert_eq!(iter.next_back(), Some(TableCoord::new(1, 2)));
    assert_eq!(iter.len(), 4);
    assert_eq!(iter.peek(), Some(TableCoord::new(0, 1)));
    assert_eq!(iter.peek_back(), Some(TableCoord::new(1, 1)));
}

#[cfg(feature = "alloc")]
mod alloc {
    use super::*;
    use crate::Vec;

    #[test]
    fn coordinate_sequence() {
        let coords: Vec<_> = TableShape::new(2, 3).try_coords().unwrap().collect();
        assert_eq!(
            coords,
            [
                TableCoord::new(0, 0),
                TableCoord::new(0, 1),
                TableCoord::new(0, 2),
                TableCoord::new(1, 0),
                TableCoord::new(1, 1),
                TableCoord::new(1, 2),
            ]
        );
    }
    #[test]
    fn coordinate_order_is_layout_independent() {
        let shape = TableShape::new(2, 3);
        let rows: Vec<_> = TableLayout::row_major(shape).unwrap().coords().collect();
        let columns: Vec<_> = TableLayout::column_major(shape).unwrap().coords().collect();
        assert_eq!(rows, columns);
    }
}
