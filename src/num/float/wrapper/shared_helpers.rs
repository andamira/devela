// devela::num::float::wrapper::shared
//
//! private helpers

use crate::Float;

#[rustfmt::skip]
#[cfg(feature = "_float_f32")]
impl Float<f32> {
    #[must_use]
    pub(super) const fn asin_acos_series_terms_f32(x: f32) -> u32 {
        let abs_a = Float(x).const_abs().0;
        if abs_a <= 0.1 { 5
        } else if abs_a <= 0.3 { 7
        } else if abs_a <= 0.5 { 10
        } else if abs_a <= 0.7 { 18
        } else if abs_a <= 0.9 { 47
        } else if abs_a <= 0.99 { 333
        } else { 1989 // computed for 0.999
        }
    }

    #[must_use]
    pub(super) const fn atan_series_terms_f32(x: f32) -> u32 {
        let abs_a = Float(x).const_abs().0;
        if abs_a <= 0.1 { 5
        } else if abs_a <= 0.3 { 7
        } else if abs_a <= 0.5 { 12
        } else if abs_a <= 0.7 { 20
        } else if abs_a <= 0.9 { 61
        } else if abs_a <= 0.99 { 518
        } else { 4151 // computed for 0.999
        }
    }

    #[must_use]
    pub(super) const fn exp_series_terms_f32(x: f32) -> u32 {
        let abs_a = Float(x).const_abs().0;
        if abs_a <= 0.001 { 3
        } else if abs_a <= 0.1 { 6
        } else if abs_a <= 1.0 { 11
        } else if abs_a <= 10.0 { 32
        } else if abs_a <= 20.0 { 49
        } else if abs_a <= 50.0 { 92
        } else { 143 // computed for max computable value f32::MAX.ln()
        }
    }

    #[must_use]
    pub(super) const fn exp2_series_terms_f32(x: f32) -> u32 {
        let abs_a = Float(x).const_abs().0;
        if abs_a <= 0.3 { 8
        } else if abs_a <= 3.0 { 15
        } else if abs_a <= 7.0 { 22
        } else if abs_a <= 15.0 { 34
        } else if abs_a <= 31.0 { 52
        } else if abs_a <= 63.0 { 84
        } else { 144 // computed for max computable value f64::MAX.ln()
        }
    }

    #[must_use]
    pub(super) const fn ln_series_terms_f32(x: f32) -> u32 {
        let x = Float(x).const_abs().0;
        let x = if x == 0.0 { return 0;
        } else if x <= 1. { 1. / x } else { x };

        if x <= 10. { 32
        } else if x <= 100. { 245
        } else if x <= 1_000. { 1_923
        } else if x <= 10_000. { 12_578
        } else if x <= 100_000. { 81_181
        } else if x <= 1_000_000. { 405_464
        } else if x <= 10_000_000. { 2_027_320 // from now one prev * 5 …
        } else if x <= 100_000_000. { 10_136_600
        } else if x <= 1_000_000_000. { 50_683_000
        } else { 253_415_000 }

        // 32 * 7 = 224
        // 245 * 7 = 1715
        // 1923 * 7 = 13461
        // 12578 * 7 = 88046
        // 81181 * 5 = 405905
    }
}

#[rustfmt::skip]
#[cfg(feature = "_float_f64")]
impl Float<f64> {
    #[must_use]
    pub(super) const fn asin_acos_series_terms_f64(x: f64) -> u32 {
        let abs_a = Float(x).const_abs().0;
        if abs_a <= 0.1 { 9
        } else if abs_a <= 0.3 { 15
        } else if abs_a <= 0.5 { 24
        } else if abs_a <= 0.7 { 44
        } else if abs_a <= 0.9 { 134
        } else if abs_a <= 0.99 { 1235
        } else { 10768 // computed for 0.999
        }
    }

    #[must_use]
    pub(super) const fn atan_series_terms_f64(x: f64) -> u32 {
        let abs_a = Float(x).const_abs().0;
        if abs_a <= 0.1 { 9
        } else if abs_a <= 0.3 { 15
        } else if abs_a <= 0.5 { 26
        } else if abs_a <= 0.7 { 47
        } else if abs_a <= 0.9 { 152
        } else if abs_a <= 0.99 { 1466
        } else { 13604 // computed for 0.999
        }
    }

    #[must_use]
    pub(super) const fn exp_series_terms_f64(x: f64) -> u32 {
        let abs_a = Float(x).const_abs().0;
        if abs_a <= 0.001 { 5
        } else if abs_a <= 0.1 { 10
        } else if abs_a <= 1.0 { 18
        } else if abs_a <= 10.0 { 46
        } else if abs_a <= 20.0 { 68
        } else if abs_a <= 50.0 { 119
        } else if abs_a <= 89.0 { 177
        } else if abs_a <= 150.0 { 261
        } else if abs_a <= 300.0 { 453
        } else if abs_a <= 500.0 { 692
        } else { 938 // computed for max computable value 709.782
        }
    }

    #[must_use]
    pub(super) const fn exp2_series_terms_f64(x: f64) -> u32 {
        let abs_a = Float(x).const_abs().0;
        if abs_a <= 0.3 { 13
        } else if abs_a <= 3.0 { 25
        } else if abs_a <= 7.0 { 34
        } else if abs_a <= 15.0 { 49
        } else if abs_a <= 31.0 { 71
        } else if abs_a <= 63.0 { 110
        } else if abs_a <= 128.0 { 178
        } else if abs_a <= 255.0 { 298
        } else if abs_a <= 511.0 { 520
        } else { 939 // computed for max computable value 1023.999
        }
    }
    
    #[must_use]
    pub(super) const fn ln_series_terms_f64(x: f64) -> u32 {
        let x = Float(x).const_abs().0;
        let x = if x == 0.0 { return 0;
        } else if x <= 1. { 1. / x } else { x };

        if x <= 10. { 80
        } else if x <= 100. { 720
        } else if x <= 1_000. { 6_639
        } else if x <= 10_000. { 59_174
        } else if x <= 100_000. { 536_609
        } else if x <= 1_000_000. { 4_817_404
        } else if x <= 10_000_000. { 43_356_636 // from now on prev * 9
        } else if x <= 100_000_000. { 390_209_724
        } else { 3_511_887_516 }
        // 80 * 9 = 720
        // 720 * 9 = 6480
        // 6639 * 9 = 59751
        // 59174 * 9 = 532566
    }
}
