// devela/src/geom/affine/point/turn.rs
//
//! Implements methods related to `Turn` and `PointSegmentRelation`.
//
// FUTURE: support 128-bit ops WAIT: u256

use crate::{Point, PointSegmentRelation, Turn, is};

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

macro_rules! impl_point_segment_relation_int {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Point<$t, 2> {
                /// Classifies this point relative to the directed segment `orig → dest`.
                ///
                /// Returns [`None`] when the segment is degenerate, meaning
                /// that its origin and destination coincide.
                #[must_use]
                pub const fn segment_relation(self, orig: Self, dest: Self)
                    -> Option<PointSegmentRelation> {
                    // A zero-length segment has neither a direction nor a
                    // meaningful supporting-line ordering.
                    if orig.coords[0] == dest.coords[0] && orig.coords[1] == dest.coords[1] {
                        return None;
                    }
                    match orig.turn(dest, self) {
                        Turn::Left => { return Some(PointSegmentRelation::Left); }
                        Turn::Right => { return Some(PointSegmentRelation::Right); }
                        Turn::Collinear => {}
                    }
                    // Along a non-degenerate collinear segment, either varying
                    // coordinate provides a monotonic parameter.
                    let (origin_axis, destination_axis, point_axis) =
                        if orig.coords[0] != dest.coords[0] {
                            (orig.coords[0], dest.coords[0], self.coords[0])
                        } else {
                            (orig.coords[1], dest.coords[1], self.coords[1])
                        };
                    if point_axis == origin_axis {
                        Some(PointSegmentRelation::Origin)
                    } else if point_axis == destination_axis {
                        Some(PointSegmentRelation::Destination)
                    } else if origin_axis < destination_axis {
                        if point_axis < origin_axis {
                            Some(PointSegmentRelation::Behind)
                        } else if point_axis > destination_axis {
                            Some(PointSegmentRelation::Beyond)
                        } else {
                            Some(PointSegmentRelation::Between)
                        }
                    } else if point_axis > origin_axis {
                        Some(PointSegmentRelation::Behind)
                    } else if point_axis < destination_axis {
                        Some(PointSegmentRelation::Beyond)
                    } else {
                        Some(PointSegmentRelation::Between)
                    }
                }
            }
        )+
    };
}
impl_point_segment_relation_int!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
