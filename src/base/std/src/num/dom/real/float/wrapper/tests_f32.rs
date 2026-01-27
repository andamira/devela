// devela::num::dom::real::float::wrapper::tests
//
//! Tests for `Float<f32>`.
//

use super::definition::Float;
use crate::assert_approx_eq_all;

#[test]
fn sqrt_basic() {
    let test_values = [
        0.0_f32, 0.5, 1.0, 2.0, 10.0,
        1e-6, // fisr: 1.0016958 vs std: 1.0
             // 1e6, f32::MIN_POSITIVE, f32::MAX, // NOTE require higher tolerance
    ];
    for &x in &test_values {
        let nr = Float(x).sqrt_nr().0;
        let fisr = Float(x).sqrt_fisr().0; // 1e-3
        let std = x.sqrt(); // TEST
        assert_approx_eq_all!(tolerance: 1e-9, nr, std);
        assert_approx_eq_all!(tolerance: 1e-2, fisr, std); // looser tolerance for FISR
    }
}
