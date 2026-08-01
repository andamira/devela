// geom/affine/point/turn/float.rs

use crate::{Point, PointSegmentRelation, Turn, is, lets, slice, whilst};

impl Point<f32, 2> {
    /// Returns the exact planar turn formed by `(self, b, c)`.
    ///
    /// Returns `None` if any coordinate is NaN or infinite.
    #[must_use]
    pub const fn try_turn(self, b: Self, c: Self) -> Option<Turn> {
        lets! { [ax, ay] = self.coords,  [bx, by] = b.coords,  [cx, cy] = c.coords }
        if !ax.is_finite()
            || !ay.is_finite()
            || !bx.is_finite()
            || !by.is_finite()
            || !cx.is_finite()
            || !cy.is_finite()
        {
            return None;
        }
        // Widen before every operation.
        let ab_x = bx as f64 - ax as f64;
        let ab_y = by as f64 - ay as f64;
        let ac_x = cx as f64 - ax as f64;
        let ac_y = cy as f64 - ay as f64;
        Some(turn_from_sign(ab_x * ac_y - ab_y * ac_x))
    }
}
#[rustfmt::skip]
const fn turn_from_sign(value: f64) -> Turn {
    if value > 0.0 { Turn::Left } else if value < 0.0 { Turn::Right } else { Turn::Collinear }
}

/* f64 */

impl Point<f64, 2> {
    /// Returns the robust planar turn formed by `(self, b, c)`.
    ///
    /// Returns `None` if any coordinate is NaN or infinite, or if an
    /// intermediate difference or product exceeds the supported `f64` range.
    ///
    /// The determinant sign is evaluated adaptively, using ordinary
    /// arithmetic for well-separated inputs and floating-point expansions
    /// for near-collinear inputs.
    #[must_use]
    pub const fn try_turn(self, b: Self, c: Self) -> Option<Turn> {
        lets! { [ax, ay] = self.coords,  [bx, by] = b.coords,  [cx, cy] = c.coords }
        if !ax.is_finite()
            || !ay.is_finite()
            || !bx.is_finite()
            || !by.is_finite()
            || !cx.is_finite()
            || !cy.is_finite()
        {
            return None;
        }
        orient2d_f64(ax, ay, bx, by, cx, cy)
    }
}

const EPSILON: f64 = 1.110_223_024_625_156_5e-16;
const RESULT_ERR_BOUND: f64 = (3.0 + 8.0 * EPSILON) * EPSILON;
const CCW_ERR_BOUND_A: f64 = (3.0 + 16.0 * EPSILON) * EPSILON;
const CCW_ERR_BOUND_B: f64 = (2.0 + 12.0 * EPSILON) * EPSILON;
const CCW_ERR_BOUND_C: f64 = (9.0 + 64.0 * EPSILON) * EPSILON * EPSILON;

const fn orient2d_f64(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> Option<Turn> {
    // Anchoring at c is algebraically equivalent to: (b - a) × (c - a)
    // but is the conventional robust-predicate formulation.
    lets! { ac_x = ax - cx,  ac_y = ay - cy,  bc_x = bx - cx,  bc_y = by - cy }
    if !ac_x.is_finite() || !ac_y.is_finite() || !bc_x.is_finite() || !bc_y.is_finite() {
        return None;
    }
    lets! { det_left = ac_x * bc_y,  det_right = ac_y * bc_x }
    is! { !det_left.is_finite() || !det_right.is_finite(), return None }
    // Opposite signs determine the result immediately
    // and avoid a potentially overflowing subtraction.
    let det_sum = if det_left > 0.0 {
        is! { det_right <= 0.0, return Some(Turn::Left) }
        det_left + det_right
    } else if det_left < 0.0 {
        is! { det_right >= 0.0, return Some(Turn::Right) }
        -det_left - det_right
    } else {
        return Some(turn_from_sign(-det_right));
    };
    is! { !det_sum.is_finite(), return None }
    let det = det_left - det_right;
    let error_bound = CCW_ERR_BOUND_A * det_sum;
    is! { det >= error_bound || -det >= error_bound, return Some(turn_from_sign(det)) }
    Some(turn_from_sign(orient2d_adapt(ax, ay, bx, by, cx, cy, det_sum)))
}
const fn orient2d_adapt(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64, det_sum: f64) -> f64 {
    lets! { ac_x = ax - cx,  ac_y = ay - cy,  bc_x = bx - cx,  bc_y = by - cy }
    let (left_hi, left_lo) = two_product(ac_x, bc_y);
    let (right_hi, right_lo) = two_product(ac_y, bc_x);
    let [b0, b1, b2, b3] = two_two_diff(left_hi, left_lo, right_hi, right_lo);
    let base = [b0, b1, b2, b3];
    let mut determinant = estimate(&base);
    let mut error_bound = CCW_ERR_BOUND_B * det_sum;
    is! { determinant >= error_bound || -determinant >= error_bound, return determinant }
    let ac_x_tail = two_diff_tail(ax, cx, ac_x);
    let ac_y_tail = two_diff_tail(ay, cy, ac_y);
    let bc_x_tail = two_diff_tail(bx, cx, bc_x);
    let bc_y_tail = two_diff_tail(by, cy, bc_y);
    if ac_x_tail == 0.0 && ac_y_tail == 0.0 && bc_x_tail == 0.0 && bc_y_tail == 0.0 {
        return determinant;
    }
    error_bound = CCW_ERR_BOUND_C * det_sum + RESULT_ERR_BOUND * determinant.abs();
    determinant += (ac_x * bc_y_tail + bc_y * ac_x_tail) - (ac_y * bc_x_tail + bc_x * ac_y_tail);
    is! { determinant >= error_bound || -determinant >= error_bound, return determinant }
    let first = product_difference_expansion(ac_x_tail, bc_y, ac_y_tail, bc_x);
    let mut expanded_1 = [0.0; 8];
    let expanded_1_len = expansion_sum(&base, &first, &mut expanded_1);
    let second = product_difference_expansion(ac_x, bc_y_tail, ac_y, bc_x_tail);
    let mut expanded_2 = [0.0; 12];
    let expanded_2_len =
        expansion_sum(slice![&expanded_1, ..expanded_1_len], &second, &mut expanded_2);
    let third = product_difference_expansion(ac_x_tail, bc_y_tail, ac_y_tail, bc_x_tail);
    let mut exact = [0.0; 16];
    let exact_len = expansion_sum(slice![&expanded_2, ..expanded_2_len], &third, &mut exact);
    // Expansion components are ordered by increasing magnitude,
    // so the final component carries the exact sign.
    exact[exact_len - 1]
}
const fn two_product(a: f64, b: f64) -> (f64, f64) {
    let high = a * b;
    #[cfg(feature = "std")]
    let low = a.mul_add(b, -high);
    #[cfg(not(feature = "std"))]
    let low = two_product_tail(a, b, high);
    (high, low)
}
#[cfg(not(feature = "std"))]
const fn two_product_tail(a: f64, b: f64, product: f64) -> f64 {
    let (a_high, a_low) = split_f64(a);
    let (b_high, b_low) = split_f64(b);
    let error_1 = product - a_high * b_high;
    let error_2 = error_1 - a_low * b_high;
    let error_3 = error_2 - a_high * b_low;
    a_low * b_low - error_3
}
#[cfg(not(feature = "std"))]
const fn split_f64(value: f64) -> (f64, f64) {
    // 2^ceil(53 / 2) + 1
    const SPLITTER: f64 = 134_217_729.0;
    let split = SPLITTER * value;
    let large = split - value;
    let high = split - large;
    let low = value - high;
    (high, low)
}

const fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let high = a + b;
    let b_virtual = high - a;
    let a_virtual = high - b_virtual;
    let b_roundoff = b - b_virtual;
    let a_roundoff = a - a_virtual;
    (high, a_roundoff + b_roundoff)
}
const fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    let high = a + b;
    let low = b - (high - a);
    (high, low)
}
const fn two_diff(a: f64, b: f64) -> (f64, f64) {
    let high = a - b;
    (high, two_diff_tail(a, b, high))
}
const fn two_diff_tail(a: f64, b: f64, high: f64) -> f64 {
    let b_virtual = a - high;
    let a_virtual = high + b_virtual;
    let b_roundoff = b_virtual - b;
    let a_roundoff = a - a_virtual;
    a_roundoff + b_roundoff
}
const fn two_one_diff(a_high: f64, a_low: f64, b: f64) -> (f64, f64, f64) {
    let (middle, low) = two_diff(a_low, b);
    let (high, middle) = two_sum(a_high, middle);
    (high, middle, low)
}
const fn two_two_diff(a_high: f64, a_low: f64, b_high: f64, b_low: f64) -> [f64; 4] {
    let (middle, carry, low) = two_one_diff(a_high, a_low, b_low);
    let (high, upper, lower) = two_one_diff(middle, carry, b_high);
    [low, lower, upper, high]
}
const fn product_difference_expansion(a: f64, b: f64, c: f64, d: f64) -> [f64; 4] {
    let (ab_high, ab_low) = two_product(a, b);
    let (cd_high, cd_low) = two_product(c, d);
    two_two_diff(ab_high, ab_low, cd_high, cd_low)
}
const fn estimate(expansion: &[f64]) -> f64 {
    let mut sum = 0.0;
    whilst! { index in 0..expansion.len(); {
        sum += expansion[index];
    }}
    sum
}
const fn expansion_sum(left: &[f64], right: &[f64], output: &mut [f64]) -> usize {
    debug_assert!(!left.is_empty());
    debug_assert!(!right.is_empty());
    debug_assert!(output.len() >= left.len() + right.len());
    let mut left_index = 0;
    let mut right_index = 0;
    let mut output_index = 0;
    let mut left_now = left[0];
    let mut right_now = right[0];
    let mut accumulator = if magnitude_precedes(left_now, right_now) {
        left_index += 1;
        left_now
    } else {
        right_index += 1;
        right_now
    };
    if left_index < left.len() && right_index < right.len() {
        left_now = left[left_index];
        right_now = right[right_index];
        let (next, residual) = if magnitude_precedes(left_now, right_now) {
            left_index += 1;
            fast_two_sum(left_now, accumulator)
        } else {
            right_index += 1;
            fast_two_sum(right_now, accumulator)
        };
        accumulator = next;
        if residual != 0.0 {
            output[output_index] = residual;
            output_index += 1;
        }
        while left_index < left.len() && right_index < right.len() {
            left_now = left[left_index];
            right_now = right[right_index];
            let (next, residual) = if magnitude_precedes(left_now, right_now) {
                left_index += 1;
                two_sum(accumulator, left_now)
            } else {
                right_index += 1;
                two_sum(accumulator, right_now)
            };
            accumulator = next;
            if residual != 0.0 {
                output[output_index] = residual;
                output_index += 1;
            }
        }
    }
    while left_index < left.len() {
        let (next, residual) = two_sum(accumulator, left[left_index]);
        accumulator = next;
        left_index += 1;
        if residual != 0.0 {
            output[output_index] = residual;
            output_index += 1;
        }
    }
    while right_index < right.len() {
        let (next, residual) = two_sum(accumulator, right[right_index]);
        accumulator = next;
        right_index += 1;
        if residual != 0.0 {
            output[output_index] = residual;
            output_index += 1;
        }
    }
    if accumulator != 0.0 || output_index == 0 {
        output[output_index] = accumulator;
        output_index += 1;
    }
    output_index
}
const fn magnitude_precedes(a: f64, b: f64) -> bool {
    // Equivalent to |a| <= |b| without calculating absolute values.
    (b > a) == (b > -a)
}

macro_rules! impl_point_segment_relation_float {
    ($($t:ty),+ $(,)?) => { $(
        impl Point<$t, 2> {
            /// Classifies this point relative to the directed segment `origin → destination`.
            ///
            /// Returns [`None`] when the segment is degenerate, any coordinate
            /// is non-finite, or the turn cannot be evaluated
            /// within the supported floating-point range.
            #[must_use]
            #[allow(clippy::float_cmp, reason = "intentional exact coordinate equality")]
            pub const fn segment_relation(
                self,
                origin: Self,
                destination: Self,
            ) -> Option<PointSegmentRelation> {
                match origin.try_turn(destination, self) {
                    Some(Turn::Left) => { return Some(PointSegmentRelation::Left); }
                    Some(Turn::Right) => { return Some(PointSegmentRelation::Right); }
                    Some(Turn::Collinear) => {}
                    None => return None,
                }
                // Since the points are collinear, either coordinate that
                // changes along the segment provides a monotonic parameter.
                let (origin_axis, destination_axis, point_axis) =
                    if origin.coords[0] != destination.coords[0] {
                        (origin.coords[0], destination.coords[0], self.coords[0])
                    } else if origin.coords[1] != destination.coords[1] {
                        (origin.coords[1], destination.coords[1], self.coords[1])
                    } else { return None; }; // The segment endpoints coincide.

                if point_axis == origin_axis { Some(PointSegmentRelation::Origin) }
                else if point_axis == destination_axis { Some(PointSegmentRelation::Destination) }
                else if origin_axis < destination_axis {
                    if point_axis < origin_axis { Some(PointSegmentRelation::Behind) }
                    else if point_axis > destination_axis { Some(PointSegmentRelation::Beyond) }
                    else { Some(PointSegmentRelation::Between) }
                }
                else if point_axis > origin_axis { Some(PointSegmentRelation::Behind) }
                else if point_axis < destination_axis { Some(PointSegmentRelation::Beyond) }
                else { Some(PointSegmentRelation::Between) }
            }
        }
    )+ };
}
impl_point_segment_relation_float!(f32, f64);
