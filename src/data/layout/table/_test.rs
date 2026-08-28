// devela/src/data/layout/table/_test.rs

use crate::{Array, ArrayShape, Table, TableCoord, TableLayout, TableShape};
#[cfg(feature = "alloc")]
use crate::{Vec, vec_};

mod shape {
    use super::*;

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
}

mod layout {
    use super::*;

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
}

mod coord {
    use super::*;

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
}

mod define {
    use super::*;

    #[test]
    fn wraps_array_semantically() {
        let layout = TableLayout::row_major(TableShape::new(2, 3)).unwrap();
        let array = Array::try_from_array([0, 1, 2, 3, 4, 5], layout.as_array()).unwrap();
        let table = Table::from_array(array);
        assert_eq!(table.shape(), TableShape::new(2, 3));
        assert_eq!(table.layout(), layout);
        assert_eq!(table.rows(), 2);
        assert_eq!(table.columns(), 3);
        assert_eq!(table.cell_count(), 6);
        assert!(!table.is_empty());
    }
    #[test]
    fn array_roundtrip_preserves_representation() {
        let layout = TableLayout::column_major(TableShape::new(2, 3)).unwrap();
        let array = Array::try_from_array([0, 1, 2, 3, 4, 5], layout.as_array()).unwrap();
        let table = Table::from_array(array);
        assert_eq!(table.as_array().layout(), layout.as_array());
        let array = table.into_array();
        assert_eq!(array.layout(), layout.as_array());
        assert_eq!(array.data(), &[0, 1, 2, 3, 4, 5]);
    }
    #[test]
    fn into_parts_uses_layout() {
        let layout = TableLayout::row_major(TableShape::new(2, 3)).unwrap();
        let array = Array::try_from_array([0, 1, 2, 3, 4, 5], layout.as_array()).unwrap();
        let (data, part_layout) = Table::from_array(array).into_parts();
        assert_eq!(data, [0, 1, 2, 3, 4, 5]);
        assert_eq!(part_layout, layout);
    }
}

mod backing {
    use super::*;
    use crate::{const_assert, unwrap};

    const TABLE_FIXED: Table<[u8; 6]> =
        unwrap![ok Table::try_from_array_copy([0, 1, 2, 3, 4, 5], TABLE_LAYOUT)];
    const TABLE_VALUE: Option<u8> = TABLE_FIXED.get(TableCoord::new(1, 2)).copied();
    const TABLE_LAYOUT: TableLayout = unwrap![ok TableLayout::row_major(TableShape::new(2, 3))];

    #[test]
    const fn const_fixed_access() {
        const_assert!(eq TABLE_VALUE.unwrap(), 5);
    }
    #[test]
    fn column_major_cell_access() {
        let layout = TableLayout::column_major(TableShape::new(2, 3)).unwrap();
        let table = Table::try_from_array([0, 1, 2, 3, 4, 5], layout).unwrap();
        assert_eq!(table.get(TableCoord::new(0, 0)), Some(&0));
        assert_eq!(table.get(TableCoord::new(1, 0)), Some(&1));
        assert_eq!(table.get(TableCoord::new(0, 1)), Some(&2));
        assert_eq!(table.get(TableCoord::new(1, 2)), Some(&5));
        assert_eq!(table.get(TableCoord::new(2, 0)), None);
        assert_eq!(table.get(TableCoord::new(0, 3)), None);
    }
    #[test]
    fn table_reborrows_storage() {
        let layout = TableLayout::row_major(TableShape::new(2, 3)).unwrap();
        let mut table = Table::try_from_array([0, 1, 2, 3, 4, 5], layout).unwrap();
        {
            let view = table.reborrow();
            let _: Table<&[u8]> = view;
            assert_eq!(view.get(TableCoord::new(1, 2)), Some(&5));
        }
        {
            let mut view = table.reborrow_mut();
            let _: &mut Table<&mut [u8]> = &mut view;
            *view.get_mut(TableCoord::new(0, 1)).unwrap() = 9;
        }
        assert_eq!(table.get(TableCoord::new(0, 1)), Some(&9));
    }
    #[test]
    fn empty_axis_traversal() {
        let layout = TableLayout::row_major(TableShape::new(3, 0)).unwrap();
        let table = Table::<[(); 0]>::try_from_array([], layout).unwrap();
        assert!(table.row_iter(0).is_some());
        assert!(table.row_iter(0).unwrap().is_empty());
        assert!(table.row_iter(2).is_some());
        assert!(table.row_iter(3).is_none());
        assert!(table.column_iter(0).is_none());
    }
    #[test]
    fn empty_rows_still_have_columns() {
        let layout = TableLayout::row_major(TableShape::new(0, 3)).unwrap();
        let table = Table::<[(); 0]>::try_from_array([], layout).unwrap();
        assert!(table.row_iter(0).is_none());
        assert!(table.column_iter(0).is_some());
        assert!(table.column_iter(0).unwrap().is_empty());
        assert!(table.column_iter(2).is_some());
        assert!(table.column_iter(3).is_none());
    }
    #[test]
    fn axis_iteration_respects_layout() {
        let shape = TableShape::new(2, 3);
        let row_major =
            Table::try_from_array([0, 1, 2, 3, 4, 5], TableLayout::row_major(shape).unwrap())
                .unwrap();
        let mut row = row_major.row_iter(1).unwrap();
        assert_eq!(row.next(), Some(&3));
        assert_eq!(row.next(), Some(&4));
        assert_eq!(row.next(), Some(&5));
        assert_eq!(row.next(), None);
        let mut column = row_major.column_iter(1).unwrap();
        assert_eq!(column.next(), Some(&1));
        assert_eq!(column.next(), Some(&4));
        assert_eq!(column.next(), None);
        let column_major =
            Table::try_from_array([0, 1, 2, 3, 4, 5], TableLayout::column_major(shape).unwrap())
                .unwrap();
        let mut row = column_major.row_iter(1).unwrap();
        assert_eq!(row.next(), Some(&1));
        assert_eq!(row.next(), Some(&3));
        assert_eq!(row.next(), Some(&5));
        let mut column = column_major.column_iter(1).unwrap();
        assert_eq!(column.next(), Some(&2));
        assert_eq!(column.next(), Some(&3));
    }
    #[test]
    fn mutable_axis_iteration() {
        let layout = TableLayout::row_major(TableShape::new(2, 3)).unwrap();
        let mut table = Table::try_from_array([0, 1, 2, 3, 4, 5], layout).unwrap();
        {
            let mut column = table.column_iter_mut(1).unwrap();

            *column.next().unwrap() += 10;
            *column.next().unwrap() += 20;
            assert!(column.next().is_none());
        }
        assert_eq!(table.storage(), &[0, 11, 2, 3, 24, 5]);
    }
    #[cfg(feature = "alloc")]
    mod alloc {
        use super::*;

        #[test]
        fn vec_boxed_conversion_preserves_table_layout() {
            let layout = TableLayout::column_major(TableShape::new(2, 3)).unwrap();
            let table = Table::try_from_vec(vec_![0, 1, 2, 3, 4, 5], layout).unwrap();
            let table = table.into_boxed();
            assert_eq!(table.layout(), layout);
            assert_eq!(table.get(TableCoord::new(1, 2)), Some(&5));
            let table = table.into_vec();
            assert_eq!(table.layout(), layout);
        }
    }
}
