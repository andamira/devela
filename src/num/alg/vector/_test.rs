// devela/src/num/alg/vector/mod.rs

use crate::{Vector, assert_approx_eq_all};

const INT_A: Vector<i32, 3> = Vector::new([2, -3, 4]);
const INT_B: Vector<i32, 3> = Vector::new([-5, 7, 1]);

const INT_ADD: Vector<i32, 3> = INT_A.add(INT_B);
const INT_SUB: Vector<i32, 3> = INT_A.sub(INT_B);
const INT_NEG: Vector<i32, 3> = INT_A.neg();
const INT_MUL: Vector<i32, 3> = INT_A.mul_scalar(3);
const INT_DIV: Vector<i32, 3> = INT_A.div_scalar(2);
const INT_DOT: i32 = INT_A.dot(&INT_B);
const INT_CROSS: Vector<i32, 3> = INT_A.cross(INT_B);

const INT_CHECKED_NEG: Option<Vector<i32, 3>> = INT_A.checked_neg();
const INT_CHECKED_DIV: Option<Vector<i32, 3>> = INT_A.checked_div_scalar(2);

const NEG_OVERFLOW: Option<Vector<i8, 3>> = Vector::new([1, i8::MIN, -1]).checked_neg();
const DIV_BY_ZERO: Option<Vector<i8, 3>> = Vector::new([8_i8, -12, 4]).checked_div_scalar(0);
const DIV_OVERFLOW: Option<Vector<i8, 3>> = Vector::new([1, i8::MIN, -1]).checked_div_scalar(-1);

const FLOAT_VECTOR: Vector<f64, 3> = Vector::new([3.0, 4.0, 12.0]);
const FLOAT_MAGNITUDE: f64 = FLOAT_VECTOR.magnitude();
const FLOAT_MAGNITUDE_SQ: f64 = FLOAT_VECTOR.magnitude_sq();
const FLOAT_MAGNITUDE_UNSCALED: f64 = FLOAT_VECTOR.magnitude_unscaled();
const FLOAT_NORMALIZED: Option<Vector<f64, 3>> = FLOAT_VECTOR.try_normalize();

#[test]
fn representation_and_construction() {
    assert_eq!(size_of::<Vector<u32, 3>>(), size_of::<[u32; 3]>());
    assert_eq!(align_of::<Vector<u32, 3>>(), align_of::<[u32; 3]>());
    assert_eq!(Vector::new([1, 2, 3]).coords, [1, 2, 3]);
    assert_eq!(Vector::<i32, 3>::splat(7), Vector::new([7, 7, 7]));
    assert_eq!(Vector::<i32, 3>::ZERO, Vector::new([0, 0, 0]));
    assert_eq!(Vector::<i32, 3>::default(), Vector::new([0, 0, 0]));
    let empty = Vector::<i32, 0>::ZERO;
    assert_eq!(empty.coords, [0i32; 0]);
    assert_eq!(empty.dot(&empty), 0);
}
#[test]
fn primitive_integer_methods_are_const_capable() {
    assert_eq!(INT_ADD, Vector::new([-3, 4, 5]));
    assert_eq!(INT_SUB, Vector::new([7, -10, 3]));
    assert_eq!(INT_NEG, Vector::new([-2, 3, -4]));
    assert_eq!(INT_MUL, Vector::new([6, -9, 12]));
    assert_eq!(INT_DIV, Vector::new([1, -1, 2]));
    assert_eq!(INT_DOT, -27);
    assert_eq!(INT_CROSS, Vector::new([-31, -22, -1]));
    assert_eq!(INT_CHECKED_NEG, Some(Vector::new([-2, 3, -4])));
    assert_eq!(INT_CHECKED_DIV, Some(Vector::new([1, -1, 2])));
}
#[test]
fn checked_integer_methods_report_failure() {
    assert_eq!(NEG_OVERFLOW, None);
    assert_eq!(DIV_BY_ZERO, None);
    assert_eq!(DIV_OVERFLOW, None);
    assert_eq!(Vector::new([8_u8, 10]).checked_div_scalar(2), Some(Vector::new([4, 5])));
    assert_eq!(Vector::new([8_u8, 10]).checked_div_scalar(0), None);
}
#[test]
#[should_panic(expected = "attempt to divide a vector by zero")]
fn integer_division_by_zero_panics() {
    let _ = Vector::new([1_i32, 2, 3]).div_scalar(0);
}
#[test]
#[should_panic]
fn signed_integer_division_overflow_panics() {
    let _ = Vector::new([i8::MIN]).div_scalar(-1);
}
#[test]
fn floating_methods_are_const_capable() {
    assert_approx_eq_all![tolerance: 1e-12_f64, FLOAT_MAGNITUDE, 13.0];
    assert_approx_eq_all![tolerance: 1e-12_f64, FLOAT_MAGNITUDE_SQ, 169.0];
    assert_approx_eq_all![tolerance: 1e-12_f64, FLOAT_MAGNITUDE_UNSCALED, 13.0];
    let normalized = FLOAT_NORMALIZED.expect("nonzero finite vector");
    assert_approx_eq_all![
        tolerance: 1e-12_f64,
        normalized.coords[0],
        3.0 / 13.0
    ];
    assert_approx_eq_all![
        tolerance: 1e-12_f64,
        normalized.coords[1],
        4.0 / 13.0
    ];
    assert_approx_eq_all![
        tolerance: 1e-12_f64,
        normalized.coords[2],
        12.0 / 13.0
    ];
    assert_approx_eq_all![tolerance: 1e-12_f64, normalized.magnitude(), 1.0];
}
#[test]
fn scaled_magnitude_avoids_intermediate_overflow() {
    let vector = Vector::new([f64::MAX / 2.0, f64::MAX / 2.0]);
    assert!(vector.magnitude().is_finite());
    assert!(vector.magnitude_unscaled().is_infinite());
    assert!(vector.magnitude_sq().is_infinite());
}
#[test]
fn scaled_magnitude_avoids_intermediate_underflow() {
    let vector = Vector::new([f64::MIN_POSITIVE, f64::MIN_POSITIVE]);
    assert!(vector.magnitude() > 0.0);
    assert_eq!(vector.magnitude_unscaled(), 0.0);
    assert_eq!(vector.magnitude_sq(), 0.0);
    let smallest_subnormal = f64::from_bits(1);
    let subnormal = Vector::new([smallest_subnormal, 0.0]);
    assert_eq!(subnormal.magnitude(), smallest_subnormal);
}
#[test]
fn magnitude_and_normalization_handle_special_values() {
    let zero = Vector::<f64, 3>::ZERO;
    let empty = Vector::<f64, 0>::ZERO;
    let infinite = Vector::new([1.0, f64::INFINITY]);
    let nan = Vector::new([1.0, f64::NAN]);
    assert_eq!(zero.magnitude(), 0.0);
    assert_eq!(empty.magnitude(), 0.0);
    assert_eq!(infinite.magnitude(), f64::INFINITY);
    assert!(nan.magnitude().is_nan());
    assert_eq!(zero.try_normalize(), None);
    assert_eq!(empty.try_normalize(), None);
    assert_eq!(infinite.try_normalize(), None);
    assert_eq!(nan.try_normalize(), None);
}
#[test]
fn floating_division_follows_primitive_semantics() {
    let quotient = Vector::new([1.0_f64, -1.0, 0.0]).div_scalar(0.0);
    assert_eq!(quotient.coords[0], f64::INFINITY);
    assert_eq!(quotient.coords[1], f64::NEG_INFINITY);
    assert!(quotient.coords[2].is_nan());
}
#[test]
fn vector_operators() {
    let a = Vector::new([8_i32, 12, -4]);
    let b = Vector::new([2_i32, -3, 5]);
    assert_eq!(a + b, Vector::new([10, 9, 1]));
    assert_eq!(a - b, Vector::new([6, 15, -9]));
    assert_eq!(-a, Vector::new([-8, -12, 4]));
    assert_eq!(a * 2, Vector::new([16, 24, -8]));
    assert_eq!(a / 2, Vector::new([4, 6, -2]));
    let mut value = a;
    value += b;
    assert_eq!(value, Vector::new([10, 9, 1]));
    value -= b;
    assert_eq!(value, a);
    value *= 2;
    assert_eq!(value, Vector::new([16, 24, -8]));
    value /= 4;
    assert_eq!(value, Vector::new([4, 6, -2]));
}
#[cfg(feature = "int")]
#[test]
fn unsigned_integer_magnitudes() {
    let square = Vector::new([3_u32, 4]);
    assert_eq!(square.magnitude_floor(), 5);
    assert_eq!(square.magnitude_ceil(), 5);
    assert_eq!(square.magnitude_round(), 5);
    let nonsquare = Vector::new([2_u32, 3]);
    assert_eq!(nonsquare.magnitude_floor(), 3);
    assert_eq!(nonsquare.magnitude_ceil(), 4);
    assert_eq!(nonsquare.magnitude_round(), 4);
}
