// devela::num::float::shared
//!
//

use crate::num::{Float, Sign};

/* shared */

#[test]
fn round_ties_even() {
    assert_eq!(Float(2.51_f32).round_ties_even(), 3.0);
    assert_eq!(Float(2.50_f32).round_ties_even(), 2.0);
    assert_eq!(Float(2.49_f32).round_ties_even(), 2.0);
}
#[test]
fn round_ties_odd() {
    assert_eq!(Float(2.51_f32).round_ties_odd(), 3.0);
    assert_eq!(Float(2.50_f32).round_ties_odd(), 3.0);
    assert_eq!(Float(2.49_f32).round_ties_odd(), 2.0);
}
#[test]
fn sign() {
    assert_eq!(Float(0.0_f32).sign(), Sign::Positive);
    assert_eq!(Float(-0.0_f32).sign(), Sign::Negative);
    assert_eq!(Float(1.0_f32).sign(), Sign::Positive);
    assert_eq!(Float(-1.0_f32).sign(), Sign::Negative);
}
#[test]
fn sign_nonzero() {
    assert_eq!(Float(0.0_f32).sign_nonzero(), Sign::None);
    assert_eq!(Float(-0.0_f32).sign_nonzero(), Sign::None);
    assert_eq!(Float(1.0_f32).sign_nonzero(), Sign::Positive);
    assert_eq!(Float(-1.0_f32).sign_nonzero(), Sign::Negative);
}
#[test]
fn is_sign_positive() {
    assert_eq!(Float(0.0_f32).is_sign_positive(), true);
    assert_eq!(Float(-0.0_f32).is_sign_positive(), false);
}
#[test]
fn is_sign_negative() {
    assert_eq!(Float(0.0_f32).is_sign_negative(), false);
    assert_eq!(Float(-0.0_f32).is_sign_negative(), true);
}
#[test]
fn is_sign_positive_nonzero() {
    assert_eq!(Float(0.0_f32).is_sign_positive_nonzero(), false);
    assert_eq!(Float(-0.0_f32).is_sign_positive_nonzero(), false);
    assert_eq!(Float(1.0_f32).is_sign_positive_nonzero(), true); //
    assert_eq!(Float(-1.0_f32).is_sign_positive_nonzero(), false);
}
#[test]
fn is_sign_negative_nonzero() {
    assert_eq!(Float(0.0_f32).is_sign_negative_nonzero(), false);
    assert_eq!(Float(-0.0_f32).is_sign_negative_nonzero(), false);
    assert_eq!(Float(1.0_f32).is_sign_negative_nonzero(), false);
    assert_eq!(Float(-1.0_f32).is_sign_negative_nonzero(), true); //
}
#[test]
fn is_zero() {
    assert_eq!(Float(0.0_f32).is_zero(), true);
    assert_eq!(Float(-0.0_f32).is_zero(), true);
    assert_eq!(Float(1.0_f32).is_zero(), false);
    assert_eq!(Float(-1.0_f32).is_zero(), false);
}
#[test]
fn mul_add_fallback() {
    assert_eq!(Float(2.0_f32).mul_add_fallback(3.0, 4.0), 10.0);
}
#[test]
fn div_euclid() {
    assert_eq!(Float(17.0_f32).div_euclid(3.0), 5.0);
    assert_eq!(Float(17.0_f32).div_euclid(-3.0), -5.0);
    assert_eq!(Float(-17.0_f32).div_euclid(3.0), -6.0);
    assert_eq!(Float(-17.0_f32).div_euclid(-3.0), 6.0);
}
#[test]
fn rem_euclid() {
    assert_eq!(Float(17.0_f32).rem_euclid(3.0), 2.0);
    assert_eq!(Float(17.0_f32).rem_euclid(-3.0), 2.0);
    assert_eq!(Float(-17.0_f32).rem_euclid(3.0), 1.0);
    assert_eq!(Float(-17.0_f32).rem_euclid(-3.0), 1.0);
}
#[test]
fn scale() {
    assert_eq![Float(45_f32).scale(0., 360., 0., 1.), 0.125];
}
#[test]
fn lerp() {
    assert_eq![Float(0.5_f32).lerp(40., 80.), 60.];
}
#[test]
fn eval_poly() {
    assert_eq![Float(3.0_f64).eval_poly(&[]), 0.0];
    assert_eq![Float(3.0_f64).eval_poly(&[0.]), 0.0];
    assert_eq![Float(3.0_f64).eval_poly(&[1.]), 1.0];
    assert_eq![Float(3.0_f64).eval_poly(&[2.0, -6.0, 2.0, -1.0]), 5.0];
}
