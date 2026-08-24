// devela/src/media/visual/image/raster/draw/_test.rs

use crate::{Coverage8, Extent2, IteratorFused, Position2, Slice, const_assert, ext, pos};
use crate::{RasterElement, RasterGrid};

mod line_iter {
    use super::*;
    use crate::RasterLineIter;

    const GRID: RasterGrid = RasterGrid::new(ext![8, 8]);
    const CONST_LINE: [RasterElement; 5] = {
        let mut iter = RasterLineIter::new(GRID, pos![1_i64, 1], pos![5_i64, 3]);
        let mut elements = [RasterElement::full(pos![0_u32, 0]); 5];
        let mut len = 0;
        while let Some(element) = iter.next() {
            if len == elements.len() {
                panic!("too many raster-line elements");
            }
            elements[len] = element;
            len += 1;
        }
        if len != elements.len() {
            panic!("too few raster-line elements");
        }
        elements
    };

    #[test]
    const fn construction_and_iteration_are_const() {
        const_assert!(Slice::<u32>::eq(&CONST_LINE[0].coord().dim, &[1, 1],));
        const_assert!(Slice::<u32>::eq(&CONST_LINE[1].coord().dim, &[2, 2],));
        const_assert!(Slice::<u32>::eq(&CONST_LINE[2].coord().dim, &[3, 2],));
        const_assert!(Slice::<u32>::eq(&CONST_LINE[3].coord().dim, &[4, 3],));
        const_assert!(Slice::<u32>::eq(&CONST_LINE[4].coord().dim, &[5, 3],));
        const_assert!(CONST_LINE[0].coverage().is_full());
        const_assert!(CONST_LINE[1].coverage().is_full());
        const_assert!(CONST_LINE[2].coverage().is_full());
        const_assert!(CONST_LINE[3].coverage().is_full());
        const_assert!(CONST_LINE[4].coverage().is_full());
    }
    #[test]
    fn accepts_positions_beyond_i32() {
        let grid = RasterGrid::new(Extent2::new([4, 4]));
        let mut line = RasterLineIter::new(
            grid,
            Position2::new([i32::MAX as i64 + 1, 0]),
            Position2::new([i32::MAX as i64 + 10, 0]),
        );
        assert_eq!(line.next(), None);
    }
    #[test]
    fn covers_every_octant() {
        let grid = RasterGrid::new(ext![11, 11]);
        let origin = [5_i64, 5_i64];
        // Caller traversal agrees with canonical positive-major traversal.
        const BASE_FORWARD: [[i64; 2]; 5] = [[0, 0], [1, 1], [2, 1], [3, 2], [4, 2]];
        // Caller traversal runs opposite the canonical direction.
        //
        // This is the canonical sequence traversed backward
        // and expressed relative to the caller's starting point.
        const BASE_BACKWARD: [[i64; 2]; 5] = [[0, 0], [1, 0], [2, 1], [3, 1], [4, 2]];
        for swap in [false, true] {
            for sign_x in [-1_i64, 1] {
                for sign_y in [-1_i64, 1] {
                    let transform = |coord: [i64; 2]| -> Position2<u32> {
                        let [x, y] = coord;
                        let [x, y] = if swap { [y, x] } else { [x, y] };
                        pos![(origin[0] + x * sign_x) as u32, (origin[1] + y * sign_y) as u32]
                    };
                    let end = transform([4, 2]);
                    let end_signed = pos![end.dim[0] as i64, end.dim[1] as i64];
                    // The delta of magnitude four is the major axis.
                    let major_sign = if swap { sign_y } else { sign_x };
                    let expected = if major_sign > 0 { BASE_FORWARD } else { BASE_BACKWARD };
                    let mut line =
                        RasterLineIter::new(grid, pos![origin[0], origin[1]], end_signed);
                    for relative in expected {
                        assert_eq!(
                            line.next().map(RasterElement::coord),
                            Some(transform(relative)),
                        );
                    }
                    assert_eq!(line.next(), None);
                }
            }
        }
    }
    #[test]
    fn reversing_endpoints_reverses_the_exact_sequence() {
        let grid = RasterGrid::new(ext![8, 8]);
        let mut forward = RasterLineIter::new(grid, pos![1, 1], pos![5, 3]);
        let mut backward = RasterLineIter::new(grid, pos![5, 3], pos![1, 1]);
        let expected_forward =
            [pos![1_u32, 1], pos![2_u32, 2], pos![3_u32, 2], pos![4_u32, 3], pos![5_u32, 3]];
        let expected_backward = [
            expected_forward[4],
            expected_forward[3],
            expected_forward[2],
            expected_forward[1],
            expected_forward[0],
        ];
        for expected in expected_forward {
            assert_eq!(forward.next().map(RasterElement::coord), Some(expected),);
        }
        for expected in expected_backward {
            assert_eq!(backward.next().map(RasterElement::coord), Some(expected),);
        }
        assert_eq!(forward.next(), None);
        assert_eq!(backward.next(), None);
    }
    #[test]
    fn clips_by_skipping_external_cells() {
        let grid = RasterGrid::new(ext![5, 4]);
        let mut line = RasterLineIter::new(grid, pos![-2, 1], pos![6, 1]);
        for x in 0..5 {
            let element = line.next().unwrap();
            assert_eq!(element.coord(), pos![x, 1]);
            assert_eq!(element.coverage(), Coverage8::FULL);
        }
        assert_eq!(line.next(), None);
    }
    #[test]
    fn clipped_reversal_is_exact() {
        let grid = RasterGrid::new(ext![5, 4]);
        let mut forward = RasterLineIter::new(grid, pos![-2, 1], pos![6, 1]);
        let mut backward = RasterLineIter::new(grid, pos![6, 1], pos![-2, 1]);
        for x in 0..5 {
            assert_eq!(forward.next().map(RasterElement::coord), Some(pos![x, 1]),);
            assert_eq!(backward.next().map(RasterElement::coord), Some(pos![4 - x, 1]),);
        }
        assert_eq!(forward.next(), None);
        assert_eq!(backward.next(), None);
    }
    #[test]
    fn coincident_endpoint_is_one_element() {
        let mut inside = RasterLineIter::new(GRID, pos![3, 4], pos![3, 4]);
        assert_eq!(inside.next(), Some(RasterElement::full(pos![3, 4])),);
        assert!(inside.is_finished());
        assert_eq!(inside.next(), None);
        let mut outside = RasterLineIter::new(GRID, pos![-1, 4], pos![-1, 4]);
        assert!(outside.is_finished());
        assert_eq!(outside.next(), None);
    }
    #[test]
    fn empty_and_trivially_disjoint_lines_finish_immediately() {
        let empty = RasterGrid::new(ext![0, u32::MAX]);
        let mut line = RasterLineIter::new(empty, pos![0, 0], pos![20, 20]);
        assert!(line.is_finished());
        assert_eq!(line.next(), None);
        let mut negative = RasterLineIter::new(GRID, pos![i64::MIN, 0], pos![i64::MIN + 1, 1]);
        assert!(negative.is_finished());
        assert_eq!(negative.next(), None);
    }
    #[test]
    fn iterator_contract() {
        fn assert_fused<I: IteratorFused>(_iter: I) {}
        let mut line = RasterLineIter::new(GRID, pos![1, 1], pos![5, 3]);
        assert_fused(line.clone());
        assert_eq!(line.size_hint(), (0, Some(5)));
        assert!(line.next().is_some());
        assert_eq!(line.size_hint(), (0, Some(4)));
        while line.next().is_some() {}
        assert!(line.is_finished());
        assert_eq!(line.size_hint(), (0, Some(0)));
        assert_eq!(line.next(), None);
        assert_eq!(line.next(), None);
    }
    #[test]
    fn lines_obey_structural_raster_invariants() {
        let grid = RasterGrid::new(ext![11, 11]);
        let origin = Position2::new([5_i64, 5]);
        let cases: [(Position2<i64>, Position2<i64>); 11] = [
            // Eight octants.
            (origin, Position2::new([9, 7])),
            (origin, Position2::new([7, 9])),
            (origin, Position2::new([3, 9])),
            (origin, Position2::new([1, 7])),
            (origin, Position2::new([1, 3])),
            (origin, Position2::new([3, 1])),
            (origin, Position2::new([7, 1])),
            (origin, Position2::new([9, 3])),
            // Axial and degenerate cases.
            (origin, Position2::new([10, 5])),
            (origin, Position2::new([5, 0])),
            (origin, origin),
        ];
        for (start, end) in cases {
            let dx = start.dim[0].abs_diff(end.dim[0]);
            let dy = start.dim[1].abs_diff(end.dim[1]);
            let major_axis = if dx >= dy { 0 } else { 1 };
            let major_delta = if dx >= dy { dx } else { dy };
            let expected_start = grid.checked_coord(start).unwrap();
            let expected_end = grid.checked_coord(end).unwrap();
            let mut line = RasterLineIter::new(grid, start, end);
            let mut previous: Option<Position2<u32>> = None;
            let mut count = 0;
            while let Some(element) = line.next() {
                let coord = element.coord();
                assert!(element.coverage().is_full());
                if count == 0 {
                    assert_eq!(coord, expected_start);
                }
                if let Some(previous) = previous {
                    let step_x = coord.dim[0].abs_diff(previous.dim[0]);
                    let step_y = coord.dim[1].abs_diff(previous.dim[1]);
                    // Every pair is adjacent and no coordinate is repeated.
                    assert!(step_x <= 1);
                    assert!(step_y <= 1);
                    assert_ne!(step_x + step_y, 0);
                    // Exactly one major-axis position is consumed per element.
                    assert_eq!(coord.dim[major_axis].abs_diff(previous.dim[major_axis]), 1,);
                }
                previous = Some(coord);
                count += 1;
            }
            assert_eq!(previous, Some(expected_end));
            assert_eq!(count, major_delta + 1);
            assert!(line.is_finished());
            assert_eq!(line.next(), None);
        }
    }
    #[test]
    fn clips_extreme_horizontal_line_to_grid_candidate_span() {
        let grid = RasterGrid::new(ext![5, 4]);
        let mut forward = RasterLineIter::new(grid, pos![i64::MIN, 1], pos![i64::MAX, 1]);
        assert_eq!(forward.size_hint(), (0, Some(5)));
        for x in 0..5 {
            assert_eq!(forward.next().map(RasterElement::coord), Some(pos![x, 1]),);
        }
        assert_eq!(forward.next(), None);
        let mut backward = RasterLineIter::new(grid, pos![i64::MAX, 1], pos![i64::MIN, 1]);
        assert_eq!(backward.size_hint(), (0, Some(5)));
        for x in (0..5).rev() {
            assert_eq!(backward.next().map(RasterElement::coord), Some(pos![x, 1]),);
        }
        assert_eq!(backward.next(), None);
    }
    #[test]
    fn clips_extreme_diagonal_without_losing_exact_state() {
        let grid = RasterGrid::new(ext![4, 4]);
        let mut line =
            RasterLineIter::new(grid, pos![i64::MIN, i64::MIN], pos![i64::MAX, i64::MAX]);
        assert_eq!(line.size_hint(), (0, Some(4)));
        for i in 0..4 {
            assert_eq!(line.next().map(RasterElement::coord), Some(pos![i, i]),);
        }
        assert_eq!(line.next(), None);
    }
}
