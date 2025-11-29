// devela_base_core::num::float::wrapper::tests_f32
//
//! Tests for `Float<f32>`.
//

use crate::{Float, Sign, assert_approx_eq_all};

/* shared */

#[test]
fn abs() {
    assert_eq!(Float(7.5_f32).abs(), 7.5);
    assert_eq!(Float(-7.5_f32).abs(), 7.5);
}
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
    assert_eq!(Float(0.0_f32).sign_nonzero(), Sign::Zero);
    assert_eq!(Float(-0.0_f32).sign_nonzero(), Sign::Zero);
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
    assert_eq![Float(3.0_f32).eval_poly(&[]), 0.0];
    assert_eq![Float(3.0_f32).eval_poly(&[0.]), 0.0];
    assert_eq![Float(3.0_f32).eval_poly(&[1.]), 1.0];
    assert_eq![Float(3.0_f32).eval_poly(&[2.0, -6.0, 2.0, -1.0]), 5.0];
}

/* shared series */

#[test]
fn sin_series() {
    const PI: f32 = Float::<f32>::PI.0;
    // (angle, expected sin)
    let test_cases = [
        (0.0, 0.0),
        (PI / 6.0, 0.5),                   // 30°
        (PI / 4.0, 2.0f32.sqrt() / 2.0),   // 45°
        (PI / 3.0, 3.0f32.sqrt() / 2.0),   // 60°
        (PI / 2.0, 1.0),                   // 90°
        (PI, 0.0),                         // 180°
        (3.0 * PI / 2.0, -1.0),            // 270°
        (2.0 * PI, 0.0),                   // 360°
        (1.0, 0.8414709848),               // Random value
        (4.0, -0.7568024953),              // In Q3
        (-PI / 4.0, -2.0f32.sqrt() / 2.0), // Negative
    ];
    for (angle, expected) in test_cases {
        let result = Float(angle).sin_series(12).0;
        assert_approx_eq_all![tolerance: 1e-6, result, expected];
    }
}
#[test]
fn cos() {
    const PI: f32 = Float::<f32>::PI.0;
    // (angle, expected cos)
    let test_cases = [
        (0.0, 1.0),
        (PI / 3.0, 0.5),                  // 60°
        (PI / 2.0, 0.0),                  // 90°
        (2.0 * PI / 3.0, -0.5),           // 120°
        (PI, -1.0),                       // 180°
        (4.0, -0.6536436209),             // Random > π
        (-PI / 4.0, 2.0f32.sqrt() / 2.0), // Negative
    ];
    for (angle, expected) in test_cases {
        let result = Float(angle).cos_series(12).0;
        assert_approx_eq_all![tolerance: 1e-6, result, expected];
    }
}
#[test]
fn tan() {
    const PI: f32 = Float::<f32>::PI.0;
    // (angle, expected tan)
    let test_cases = [
        // Special angles
        (0.0, 0.0),
        (PI / 6.0, 0.577350269), // 30°
        (PI / 4.0, 1.0),         // 45°
        (PI / 3.0, 1.732050808), // 60°
        // Quadrant boundaries
        // (PI / 2.0, f32::INFINITY), // 90° // TODO: IMPROVE assert_approx_eq_all
        (PI, 0.0),              // 180°
        (3.0 * PI / 4.0, -1.0), // 135°
        // Random values
        (0.5, 0.54630249),
        (1.0, 1.55740772),
        (1.5, 14.1014199),
        // Negative angles
        (-PI / 4.0, -1.0), // -45°
        (-0.5, -0.54630249),
    ];
    for (angle, expected) in test_cases {
        let result = Float(angle).tan_series(12).0;
        assert_approx_eq_all![tolerance: 1e-6, result, expected];
    }
}
#[test]
fn asin() {
    const PI: f32 = Float::<f32>::PI.0;
    // (angle, expected tan)
    let test_cases = [
        // Exact values
        (0.0_f32, 0.0),
        (0.5, PI / 6.0),         // 30°
        (0.707106781, PI / 4.0), // 45° (1/√2)
        (0.866025404, PI / 3.0), // 60° (√3/2)
        (1.0, PI / 2.0),         // 90°
        // // Practical values
        (0.258819045, 0.261799388), // sin(15°)
        (-0.5, -PI / 6.0),          // -30°
        // // Edge cases
        (1.0001, PI / 2.0),   // Should clamp to pi/2
        (-1.0001, -PI / 2.0), // Should clamp to -pi/2
    ];
    for (angle, expected) in test_cases {
        let angle = Float(angle);
        let result = angle.asin_series(angle.asin_series_terms()).0;
        assert_approx_eq_all![tolerance: 1e-6, result, expected];
    }
}
