// devela/src/geom/affine/point/turn.rs
//
//!
//
// FUTURE: support 128-bit ops WAIT: u256

use crate::{Point, Turn, is};

macro_rules! define_turn_int {
    ($name:ident, $sint:ty, $uint:ty) => {
        const fn $name(ax: $sint, ay: $sint, bx: $sint, by: $sint, cx: $sint, cy: $sint) -> Turn {
            let ab_x = bx - ax;
            let ab_y = by - ay;
            let ac_x = cx - ax;
            let ac_y = cy - ay;
            let ab_x_mag: $uint = is! { ab_x < 0, ab_x.wrapping_neg() as $uint, ab_x as $uint };
            let ab_y_mag: $uint = is! { ab_y < 0, ab_y.wrapping_neg() as $uint, ab_y as $uint };
            let ac_x_mag: $uint = is! { ac_x < 0, ac_x.wrapping_neg() as $uint, ac_x as $uint };
            let ac_y_mag: $uint = is! { ac_y < 0, ac_y.wrapping_neg() as $uint, ac_y as $uint };

            // determinant = ab_x * ac_y - ab_y * ac_x
            let lhs_magnitude = ab_x_mag * ac_y_mag;
            let rhs_magnitude = ab_y_mag * ac_x_mag;
            let lhs_negative = lhs_magnitude != 0 && ((ab_x < 0) != (ac_y < 0));
            let rhs_negative = rhs_magnitude != 0 && ((ab_y < 0) != (ac_x < 0));

            if lhs_negative != rhs_negative {
                if lhs_negative { Turn::Right } else { Turn::Left }
            } else if lhs_magnitude == rhs_magnitude {
                Turn::Collinear
            } else if lhs_negative {
                // (-lhs) - (-rhs) = rhs - lhs
                if lhs_magnitude > rhs_magnitude { Turn::Right } else { Turn::Left }
            } else if lhs_magnitude > rhs_magnitude {
                Turn::Left
            } else {
                Turn::Right
            }
        }
    };
}
macro_rules! impl_point_turn_int {
    ($helper:ident, $wide:ty => $($t:ty),+ $(,)?) => {
        $(
            impl Point<$t, 2> {
                /// Returns the planar turn formed by the ordered points
                /// `(self, b, c)`.
                ///
                /// The result is computed exactly without arithmetic overflow.
                #[must_use]
                pub const fn turn(self, b: Self, c: Self) -> Turn {
                    $helper(
                        self.coords[0] as $wide,
                        self.coords[1] as $wide,
                        b.coords[0] as $wide,
                        b.coords[1] as $wide,
                        c.coords[0] as $wide,
                        c.coords[1] as $wide,
                    )
                }
            }
        )+
    };
}
define_turn_int!(turn_i16, i16, u16);
define_turn_int!(turn_i32, i32, u32);
define_turn_int!(turn_i64, i64, u64);
define_turn_int!(turn_i128, i128, u128);

impl_point_turn_int!(turn_i16, i16 => i8, u8);
impl_point_turn_int!(turn_i32, i32 => i16, u16);
impl_point_turn_int!(turn_i64, i64 => i32, u32);
impl_point_turn_int!(turn_i128, i128 => i64, u64);

#[cfg(target_pointer_width = "16")]
impl_point_turn_int!(turn_i32, i32 => isize, usize);
#[cfg(target_pointer_width = "32")]
impl_point_turn_int!(turn_i64, i64 => isize, usize);
#[cfg(target_pointer_width = "64")]
impl_point_turn_int!(turn_i128, i128 => isize, usize);

#[cfg(test)]
mod tests {
    use super::{Point, Turn};

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
