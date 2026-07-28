// devela/src/geom/affine/point/_test.rs

mod turn {
    use crate::{Point, Turn};

    macro_rules! point {
        ($t:ty; $x:expr, $y:expr) => {
            Point::<$t, 2> { coords: [$x as $t, $y as $t] }
        };
    }
    macro_rules! assert_turn_basics {
        ($t:ty) => {{
            let a = point![$t; 0, 0];
            let b = point![$t; 4, 0];
            let c = point![$t; 1, 3];
            let d = point![$t; 2, 0];
            let turn = a.turn(b, c);
            assert_eq!(turn, Turn::Left);
            assert_eq!(a.turn(c, b), Turn::Right);
            assert_eq!(a.turn(b, d), Turn::Collinear);
            // Cyclic permutations preserve the turn.
            assert_eq!(b.turn(c, a), turn);
            assert_eq!(c.turn(a, b), turn);
            // Odd permutations reverse it.
            assert_eq!(b.turn(a, c), turn.reversed());
            assert_eq!(c.turn(b, a), turn.reversed());
            // Repeated points are degenerate.
            assert_eq!(a.turn(a, c), Turn::Collinear);
            assert_eq!(a.turn(b, a), Turn::Collinear);
            assert_eq!(a.turn(b, b), Turn::Collinear);
            // Translation does not affect the result.
            let ta = point![$t; 10, 20];
            let tb = point![$t; 14, 20];
            let tc = point![$t; 11, 23];
            assert_eq!(ta.turn(tb, tc), turn);
        }};
    }
    macro_rules! assert_signed_extremes {
        ($t:ty) => {{
            let min = <$t>::MIN;
            let max = <$t>::MAX;
            let a = Point::<$t, 2> { coords: [min, min] };
            let b = Point::<$t, 2> { coords: [max, min] };
            let c = Point::<$t, 2> { coords: [min, max] };
            assert_eq!(a.turn(b, c), Turn::Left);
            assert_eq!(a.turn(c, b), Turn::Right);
            let middle = Point::<$t, 2> { coords: [0 as $t, 0 as $t] };
            let diagonal = Point::<$t, 2> { coords: [max, max] };
            assert_eq!(a.turn(middle, diagonal), Turn::Collinear);
        }};
    }
    macro_rules! assert_unsigned_extremes {
        ($t:ty) => {{
            let max = <$t>::MAX;
            let a = Point::<$t, 2> { coords: [0 as $t, 0 as $t] };
            let b = Point::<$t, 2> { coords: [max, 0 as $t] };
            let c = Point::<$t, 2> { coords: [0 as $t, max] };
            assert_eq!(a.turn(b, c), Turn::Left);
            assert_eq!(a.turn(c, b), Turn::Right);
            let diagonal_a = Point::<$t, 2> { coords: [0 as $t, 0 as $t] };
            let diagonal_b = Point::<$t, 2> { coords: [1 as $t, 1 as $t] };
            let diagonal_c = Point::<$t, 2> { coords: [max, max] };
            assert_eq!(diagonal_a.turn(diagonal_b, diagonal_c), Turn::Collinear);
        }};
    }

    #[test]
    fn turn_integer_basics() {
        assert_turn_basics!(i8);
        assert_turn_basics!(u8);
        assert_turn_basics!(i16);
        assert_turn_basics!(u16);
        assert_turn_basics!(i32);
        assert_turn_basics!(u32);
        assert_turn_basics!(i64);
        assert_turn_basics!(u64);
        assert_turn_basics!(isize);
        assert_turn_basics!(usize);
    }
    #[test]
    fn turn_signed_extremes() {
        assert_signed_extremes!(i8);
        assert_signed_extremes!(i16);
        assert_signed_extremes!(i32);
        assert_signed_extremes!(i64);
        assert_signed_extremes!(isize);
    }
    #[test]
    fn turn_unsigned_extremes() {
        assert_unsigned_extremes!(u8);
        assert_unsigned_extremes!(u16);
        assert_unsigned_extremes!(u32);
        assert_unsigned_extremes!(u64);
        assert_unsigned_extremes!(usize);
    }
    const _: () = {
        let a = Point::<i32, 2> { coords: [0, 0] };
        let b = Point::<i32, 2> { coords: [1, 0] };
        let c = Point::<i32, 2> { coords: [0, 1] };
        assert!(a.turn(b, c).is_left());
    };
}

mod point_segment_relation {
    use crate::{Point, PointSegmentRelation as Relation, Turn};

    macro_rules! point {
        ($t:ty; $x:expr, $y:expr) => {
            Point::<$t, 2> { coords: [$x as $t, $y as $t] }
        };
    }
    macro_rules! assert_relations {
        ($t:ty) => {{
            // Horizontal segment directed to the right.
            let origin = point![$t; 2, 2];
            let destination = point![$t; 6, 2];
            let cases = [
                (point![$t; 4, 5], Relation::Left),
                (point![$t; 4, 0], Relation::Right),
                (point![$t; 0, 2], Relation::Behind),
                (origin, Relation::Origin),
                (point![$t; 4, 2], Relation::Between),
                (destination, Relation::Destination),
                (point![$t; 8, 2], Relation::Beyond),
            ];
            for (point, expected) in cases {
                assert_eq!(
                    point.segment_relation(origin, destination),
                    Some(expected),
                );
                assert_eq!(
                    point.segment_relation(destination, origin),
                    Some(expected.reversed()),
                );
            }
            // Vertical segment directed downward. This exercises both the
            // fallback axis and descending coordinate order.
            let origin = point![$t; 3, 7];
            let destination = point![$t; 3, 2];
            let cases = [
                (point![$t; 5, 4], Relation::Left),
                (point![$t; 1, 4], Relation::Right),
                (point![$t; 3, 9], Relation::Behind),
                (origin, Relation::Origin),
                (point![$t; 3, 4], Relation::Between),
                (destination, Relation::Destination),
                (point![$t; 3, 0], Relation::Beyond),
            ];
            for (point, expected) in cases {
                assert_eq!(
                    point.segment_relation(origin, destination),
                    Some(expected),
                );
                assert_eq!(
                    point.segment_relation(destination, origin),
                    Some(expected.reversed()),
                );
            }
            // Degenerate segments have no directed-segment relation.
            let collapsed = point![$t; 4, 4];
            assert_eq!(
                collapsed.segment_relation(collapsed, collapsed),
                None,
            );
            assert_eq!(
                point![$t; 5, 4].segment_relation(collapsed, collapsed),
                None,
            );
        }};
    }

    #[test]
    fn integer_relations() {
        assert_relations!(i8);
        assert_relations!(u8);
        assert_relations!(i16);
        assert_relations!(u16);
        assert_relations!(i32);
        assert_relations!(u32);
        assert_relations!(i64);
        assert_relations!(u64);
        assert_relations!(isize);
        assert_relations!(usize);
    }
    #[test]
    fn relation_properties() {
        let relations = [
            Relation::Left,
            Relation::Right,
            Relation::Behind,
            Relation::Origin,
            Relation::Between,
            Relation::Destination,
            Relation::Beyond,
        ];
        for relation in relations {
            assert_eq!(relation.reversed().reversed(), relation);
        }
        assert_eq!(Relation::Left.as_turn(), Turn::Left);
        assert_eq!(Relation::Right.as_turn(), Turn::Right);
        for relation in [
            Relation::Behind,
            Relation::Origin,
            Relation::Between,
            Relation::Destination,
            Relation::Beyond,
        ] {
            assert_eq!(relation.as_turn(), Turn::Collinear);
            assert!(relation.is_collinear());
        }
        assert!(Relation::Origin.is_on_segment());
        assert!(Relation::Between.is_on_segment());
        assert!(Relation::Destination.is_on_segment());
        assert!(!Relation::Behind.is_on_segment());
        assert!(!Relation::Beyond.is_on_segment());
        assert!(!Relation::Left.is_on_segment());
        assert!(!Relation::Right.is_on_segment());
    }
    const _: () = {
        let origin = Point::<i32, 2> { coords: [0, 0] };
        let destination = Point::<i32, 2> { coords: [4, 0] };
        let point = Point::<i32, 2> { coords: [2, 0] };
        assert!(matches!(point.segment_relation(origin, destination), Some(Relation::Between),));
    };
}
