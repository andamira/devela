// devela/src/geom/affine/point/turn/_test.rs

mod float_turn {
    use crate::{Point, PointSegmentRelation as Relation, Turn};

    #[test]
    fn f32_turns() {
        let a = Point::<f32, 2> { coords: [0.0, 0.0] };
        let b = Point::<f32, 2> { coords: [4.0, 0.0] };
        assert_eq!(a.try_turn(b, Point { coords: [2.0, 3.0] }), Some(Turn::Left));
        assert_eq!(a.try_turn(b, Point { coords: [2.0, -3.0] }), Some(Turn::Right));
        assert_eq!(a.try_turn(b, Point { coords: [2.0, 0.0] }), Some(Turn::Collinear));
    }
    #[test]
    fn f64_turns() {
        let a = Point::<f64, 2> { coords: [0.0, 0.0] };
        let b = Point::<f64, 2> { coords: [4.0, 0.0] };
        let c = Point::<f64, 2> { coords: [2.0, 3.0] };
        let turn = a.try_turn(b, c);
        assert_eq!(turn, Some(Turn::Left));
        assert_eq!(b.try_turn(c, a), turn);
        assert_eq!(c.try_turn(a, b), turn);
        assert_eq!(a.try_turn(c, b), Some(Turn::Right));
    }
    #[test]
    fn f64_resolves_near_collinearity() {
        let a = Point::<f64, 2> {
            coords: [f64::from_bits(0xBFF4_4110_0FDF_88E2), f64::from_bits(0xBFE9_1ABA_D188_0594)],
        };
        let b = Point::<f64, 2> {
            coords: [f64::from_bits(0xC008_2C8F_70E2_B89E), f64::from_bits(0xC004_FB67_6350_95F9)],
        };
        let c = Point::<f64, 2> {
            coords: [f64::from_bits(0x3FCB_A857_A392_4DA7), f64::from_bits(0x3FE8_8AFC_E812_597E)],
        };

        let naive = (b.coords[0] - a.coords[0]) * (c.coords[1] - a.coords[1])
            - (b.coords[1] - a.coords[1]) * (c.coords[0] - a.coords[0]);

        assert_eq!(naive, 0.0);
        assert_eq!(a.try_turn(b, c), Some(Turn::Right));

        // The point-segment relation inherits the robust turn classification.
        assert_eq!(c.segment_relation(a, b), Some(Relation::Right));
    }
    #[test]
    fn non_finite_has_no_turn() {
        let finite = Point::<f64, 2> { coords: [0.0, 0.0] };
        let infinite = Point::<f64, 2> { coords: [f64::INFINITY, 0.0] };
        let nan = Point::<f64, 2> { coords: [f64::NAN, 0.0] };
        assert_eq!(finite.try_turn(infinite, finite), None);
        assert_eq!(finite.try_turn(nan, finite), None);
    }
}

mod float_point_segment {
    use crate::{Point, PointSegmentRelation as Relation};

    macro_rules! point {
        ($t:ty; $x:expr, $y:expr) => {
            Point::<$t, 2> { coords: [$x as $t, $y as $t] }
        };
    }

    macro_rules! assert_float_relations {
        ($t:ty) => {{
            // Horizontal segment directed rightward.
            let origin = point![$t; 2.0, 2.0];
            let destination = point![$t; 6.0, 2.0];
            let cases = [
                (point![$t; 4.0, 5.0], Relation::Left),
                (point![$t; 4.0, 0.0], Relation::Right),
                (point![$t; 0.0, 2.0], Relation::Behind),
                (origin, Relation::Origin),
                (point![$t; 4.0, 2.0], Relation::Between),
                (destination, Relation::Destination),
                (point![$t; 8.0, 2.0], Relation::Beyond),
            ];
            for (point, expected) in cases {
                assert_eq!(point.segment_relation(origin, destination), Some(expected));
                assert_eq!(point.segment_relation(destination, origin), Some(expected.reversed()));
            }
            // Vertical segment directed downward.
            let origin = point![$t; 3.0, 7.0];
            let destination = point![$t; 3.0, 2.0];
            let cases = [
                (point![$t; 5.0, 4.0], Relation::Left),
                (point![$t; 1.0, 4.0], Relation::Right),
                (point![$t; 3.0, 9.0], Relation::Behind),
                (origin, Relation::Origin),
                (point![$t; 3.0, 4.0], Relation::Between),
                (destination, Relation::Destination),
                (point![$t; 3.0, 0.0], Relation::Beyond),
            ];
            for (point, expected) in cases {
                assert_eq!(point.segment_relation(origin, destination), Some(expected));
                assert_eq!(point.segment_relation(destination, origin), Some(expected.reversed()));
            }
            let collapsed = point![$t; 4.0, 4.0];
            assert_eq!(collapsed.segment_relation(collapsed, collapsed), None);
            assert_eq!(point![$t; 5.0, 4.0] .segment_relation(collapsed, collapsed), None);
        }};
    }
    #[test]
    fn floating_relations() {
        assert_float_relations!(f32);
        assert_float_relations!(f64);
    }
    #[test]
    fn non_finite_relation_is_undefined() {
        let origin = Point::<f64, 2> { coords: [0.0, 0.0] };
        let destination = Point::<f64, 2> { coords: [4.0, 0.0] };
        assert_eq!(Point { coords: [f64::NAN, 0.0] }.segment_relation(origin, destination), None,);
        assert_eq!(
            Point { coords: [2.0_f64, 0.0] }
                .segment_relation(origin, Point { coords: [f64::INFINITY, 0.0] }),
            None
        );
    }
    const _: () = {
        let origin = Point::<f32, 2> { coords: [0.0, 0.0] };
        let destination = Point::<f32, 2> { coords: [4.0, 0.0] };
        let point = Point::<f32, 2> { coords: [2.0, 0.0] };
        assert!(matches!(point.segment_relation(origin, destination), Some(Relation::Between)));
    };
}

mod int_turn {
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

    macro_rules! assert_turn_product_signs {
        ($t:ty) => {{
            let a = point![$t; 4, 4];
            // Positive lhs, negative rhs.
            assert_eq!(a.turn(point![$t; 5, 5], point![$t; 3, 5]), Turn::Left);
            // Negative lhs, positive rhs.
            assert_eq!(a.turn(point![$t; 3, 5], point![$t; 5, 5]), Turn::Right);
            // Both products negative, lhs magnitude greater.
            assert_eq!(a.turn(point![$t; 6, 3], point![$t; 5, 2]), Turn::Right);
            // Both products negative, rhs magnitude greater.
            assert_eq!(a.turn(point![$t; 5, 2], point![$t; 6, 3]), Turn::Left);
            // Equal negative products.
            assert_eq!(a.turn(point![$t; 5, 3], point![$t; 6, 2]), Turn::Collinear);
        }};
    }

    #[test]
    fn turn_product_sign_paths() {
        assert_turn_product_signs!(i8);
        assert_turn_product_signs!(u8);
        assert_turn_product_signs!(i16);
        assert_turn_product_signs!(u16);
        assert_turn_product_signs!(i32);
        assert_turn_product_signs!(u32);
        assert_turn_product_signs!(i64);
        assert_turn_product_signs!(u64);
        assert_turn_product_signs!(isize);
        assert_turn_product_signs!(usize);
    }
    #[test]
    fn turn_wide_product_cancellation() {
        let max = u64::MAX;
        let a = Point::<u64, 2> { coords: [0, 0] };
        let b = Point::<u64, 2> { coords: [max, max - 1] };
        let c = Point::<u64, 2> { coords: [max - 1, max - 2] };
        assert_eq!(a.turn(b, c), Turn::Right);
        assert_eq!(a.turn(c, b), Turn::Left);
        let min = i64::MIN;
        let max = i64::MAX;
        let a = Point::<i64, 2> { coords: [min, min] };
        let b = Point::<i64, 2> { coords: [max, max - 1] };
        let c = Point::<i64, 2> { coords: [max - 1, max - 2] };
        assert_eq!(a.turn(b, c), Turn::Right);
        assert_eq!(a.turn(c, b), Turn::Left);
    }
}

mod int_point_segment {
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
