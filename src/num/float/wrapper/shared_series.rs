// devela::num::float::wrapper::shared_series
//
//! Shared methods implemented using Taylor Series.
//
// TOC
// - impl_float_shared_series!
// - CONST tables
// - fn helpers

#[allow(unused_imports)]
use super::super::shared_docs::*;
use crate::{Float, cfor, is, paste};

/// Implements methods independently of any features
///
/// $f:   the floating-point type.
/// $uf:  unsigned int type with the same bit-size.
/// $ue:  unsigned int type used for integer exponentiation and number of terms (u32).
/// $cap: the capability feature that enables the given implementation. E.g "_float_f32".
/// $cmp: the feature that enables some methods depending on Compare. E.g "_cmp_f32".
macro_rules! impl_float_shared_series {
    () => {
        impl_float_shared_series![
            (f32:u32, u32):"_float_f32":"_cmp_f32",
            (f64:u64, u32):"_float_f64":"_cmp_f64"
        ];
    };

    ($( ($f:ty:$uf:ty, $ue:ty) : $cap:literal : $cmp:literal ),+) => {
        $( impl_float_shared_series![@$f:$uf, $ue, $cap:$cmp]; )+
    };
    (@$f:ty:$uf:ty, $ue:ty, $cap:literal : $cmp:literal) => {
        #[doc = crate::doc_availability!(feature = $cap)]
        ///
        /// # *Common methods with or without `std` or `libm`*.
        ///   *Implemented using Taylor series.*
        #[cfg(feature = $cap )]
        // #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))]
        impl Float<$f> {
            /// Raises itself to the `y` floating point power using the Taylor series via the
            /// `exp` and `ln` functions.
            ///
            /// # Formulation
            #[doc = FORMULA_POWF_SERIES!()]
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            ///
            /// The terms for the exponential function are calculated using
            /// [`exp_series_terms`][Self::exp_series_terms] using $y\cdot\ln(x)$.
            pub const fn powf_series(self, y: $f, ln_x_terms: $ue) -> Float<$f> {
                let xabs = self.abs().0;
                if xabs == 0.0 {
                    is![Float(y).abs().0 == 0.0; Self::ONE; Self::ZERO]
                } else {
                    let ln_x = Float(xabs).ln_series(ln_x_terms).0;
                    let power = Float(y * ln_x);
                    let exp_y_terms = power.exp_series_terms();
                    let result = power.exp_series(exp_y_terms);
                    is![self.is_sign_negative(); Float(-result.0); result]
                }
            }

            /// Computes the exponential function $e^x$ using Taylor series expansion.
            ///
            /// # Formulation
            #[doc = FORMULA_EXP_SERIES!()]
            ///
            /// See also [`exp_series_terms`][Self::exp_series_terms].
            pub const fn exp_series(self, terms: $ue) -> Float<$f> {
                is![self.0 < 0.0; return Float(1.0 / Float(-self.0).exp_series(terms).0)];
                let (mut result, mut term) = (1.0, 1.0);
                let mut i = 1;
                while i <= terms {
                    term *= self.0 / i as $f;
                    result += term;
                    i += 1;
                }
                Float(result)
            }

            /// Determines the number of terms needed for [`exp_series`][Self::exp_series]
            /// to reach a stable result based on the input value.
            #[doc = TABLE_EXP_SERIES_TERMS!()]
            pub const fn exp_series_terms(self) -> $ue { paste! {
                Self::[<exp_series_terms_ $f>](self.0)
            }}

            /// Calculates $ e^x - 1 $ using the Taylor series expansion.
            ///
            /// # Formulation
            #[doc = FORMULA_EXP_M1_SERIES!()]
            ///
            /// See also [`exp_series_terms`][Self::exp_series_terms].
            pub const fn exp_m1_series(self, terms: $ue) -> Float<$f> {
                if self.0 < 0.0 {
                    Float(1.0 / Float(-self.0).exp_m1_series(terms).0)
                } else if self.0 > 0.001 {
                    Float(self.exp_series(terms).0 - 1.0)
                } else {
                    let (mut result, mut term, mut factorial) = (0.0, self.0, 1.0);
                    let mut i = 1;
                    while i <= terms {
                        result += term;
                        factorial *= (i + 1) as $f;
                        term *= self.0 / factorial;
                        i += 1;
                    }
                    Float(result)
                }
            }

            /// Calculates $ 2^x $ using the Taylor series expansion.
            ///
            /// # Formulation
            #[doc = FORMULA_EXP2_SERIES!()]
            ///
            /// The maximum values with a representable result are:
            /// 127 for `f32` and 1023 for `f64`.
            pub const fn exp2_series(self, terms: $ue) -> Float<$f> {
                let (mut result, mut term) = (1.0, self.0 * Self::LN_2.0);
                let mut n = 1;
                while n < terms {
                    result += term;
                    term *= self.0 * Self::LN_2.0 / (n as $f + 1.0);
                    n += 1;
                }
                Float(result)
            }

            /// Determines the number of terms needed for [`exp2_series`][Self::exp2_series]
            /// to reach a stable result based on the input value.
            #[doc = TABLE_EXP2_SERIES_TERMS!()]
            pub const fn exp2_series_terms(self) -> $ue { paste! {
                Self::[<exp2_series_terms_ $f>](self.0)
            }}

            /// Computes the natural logarithm of `self` using a Taylor-Mercator series expansion.
            ///
            /// This method is more efficient for values of `self` near 1. Values too
            /// small or too big could be impractical to calculate with precision.
            ///
            /// # Formulation
            #[doc = FORMULA_LN_SERIES!()]
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            pub const fn ln_series(self, terms: $ue) -> Float<$f> {
                if self.0 < 0.0 {
                    Self::NAN
                } else if self.0 > 0.0 {
                    let mut sum = 0.0;
                    let y = (self.0 - 1.0) / (self.0 + 1.0);
                    let mut y_pow = y;
                    cfor![i in 0..terms => {
                        sum += y_pow / (2 * i + 1) as $f;
                        y_pow *= y * y;
                    }];
                    Float(2.0 * sum)
                } else {
                    Self::NEG_INFINITY
                }
            }

            /// Computes the natural logarithm of `1 + self`
            /// using a Taylor-Mercator series expansion.
            ///
            /// This method is more efficient for values of `self` near 0.
            /// Values too small or too big could be impractical to calculate with precision.
            ///
            /// Returns `ln(1+self)` more accurately
            /// than if the operations were performed separately.
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            pub const fn ln_1p_series(self, terms: $ue) -> Float<$f> {
                if self.0 < -1.0 {
                    Self::NAN
                } else if self.0 > -1.0 {
                    let x1 = self.0 + 1.0;
                    let mut sum = 0.0;
                    let y = (x1 - 1.0) / (x1 + 1.0);
                    let mut y_pow = y;
                    cfor![i in 0..terms => {
                        sum += y_pow / (2 * i + 1) as $f;
                        y_pow *= y * y;
                    }];
                    Float(2.0 * sum)
                } else {
                    Self::NEG_INFINITY
                }
            }

            /// Computes the logarithm to the given `base` using the change of base formula.
            ///
            /// # Formulation
            #[doc = FORMULA_LOG_SERIES!()]
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            pub const fn log_series(self, base: $f, terms: $ue) -> Float<$f> {
                if base <= 0.0 {
                    Self::NAN
                // The logarithm with a base of 1 is undefined except when the argument is also 1.
                } else if Float(base - 1.0).abs().0 < Self::MEDIUM_MARGIN.0 { // + robust
                // } else if base == 1.0 { // good enough for direct input
                    #[expect(clippy::float_cmp, reason = "we've already checked it with a margin")]
                    { is![self.0 == 1.0; Self::NAN; Self::NEG_INFINITY] }
                } else {
                    Float(self.ln_series(terms).0 / base).ln_series(terms)
                }
            }

            /// Computes the base-2 logarithm using the change of base formula.
            ///
            /// # Formulation
            #[doc = FORMULA_LOG2_SERIES!()]
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            pub const fn log2_series(self, terms: $ue) -> Float<$f> {
                Float(self.ln_series(terms).0 / 2.0).ln_series(terms)
            }

            /// Computes the base-10 logarithm using the change of base formula.
            ///
            /// # Formulation
            #[doc = FORMULA_LOG10_SERIES!()]
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            pub const fn log10_series(self, terms: $ue) -> Float<$f> {
                Float(self.ln_series(terms).0 / 10.0).ln_series(terms)
            }

            /// Determines the number of terms needed for [`exp2_series`][Self::exp2_series]
            /// to reach a stable result based on the input value.
            #[doc = TABLE_LN_SERIES_TERMS!()]
            #[must_use]
            pub const fn ln_series_terms(self) -> $ue { paste! {
                Self::[<ln_series_terms_ $f>](self.0)
            }}

            /// The sine calculated using Taylor series expansion.
            ///
            /// # Formulation
            #[doc = FORMULA_SIN_SERIES!()]
            ///
            /// This Taylor series converges relatively quickly and uniformly
            /// over the entire domain.
            #[doc = TABLE_SIN_SERIES_TERMS!()]
            // NOTE: OPTIMIZED
            pub const fn sin_series(self, terms: u32) -> Float<$f> {
                const PI: $f = Float::<$f>::PI.0;
                const TAU: $f = Float::<$f>::TAU.0;
                if terms == 0 { return Self::ZERO; } // Early exit for trivial cases
                let x = self.0 % TAU; // Reduce angle to [-π, π] while preserving sign
                let x = is![x > PI; x - TAU; is![x < -PI; x + TAU; x ]];
                let x_sq = x * x;
                let (mut sin, mut term, mut factorial) = (x, x, 1.0);
                let mut i = 1;
                while i < terms {
                    term *= -x_sq;
                    factorial *= ((2*i + 1) * (2*i)) as $f;  // (2n+1)! progression
                    sin += term / factorial;
                    i += 1;
                }
                Float(sin)
            }

            /// Computes the cosine using taylor series expansion.
            ///
            /// # Formulation
            #[doc = FORMULA_COS_SERIES!()]
            ///
            /// This Taylor series converges relatively quickly and uniformly
            /// over the entire domain.
            #[doc = TABLE_COS_SERIES_TERMS!()]
            // NOTE: OPTIMIZED
            pub const fn cos_series(self, terms: $ue) -> Float<$f> {
                if terms == 0 { return Float::<$f>::ONE; } // Early exit for trivial cases
                let x = self.0.abs() % Float::<$f>::TAU.0;
                let x_sq = x * x;
                let (mut cos, mut term, mut factorial) = (1.0, 1.0, 1.0);
                let mut i = 1;
                while i < terms {
                    term *= -x_sq;
                    factorial *= ((2*i - 1) * (2*i)) as $f;
                    cos += term / factorial;
                    i += 1;
                }
                Float(cos)
            }

            /// Computes the sine and the cosine using Taylor series expansion.
            pub const fn sin_cos_series(self, terms: $ue) -> (Float<$f>, Float<$f>) {
                (self.sin_series(terms), self.cos_series(terms))
            }

            /// Computes the tangent using Taylor series expansion of sine and cosine.
            ///
            /// # Formulation
            #[doc = FORMULA_TAN_SERIES!()]
            ///
            /// The tangent function has singularities and is not defined for
            /// `cos(x) = 0`. This function clamps `self` within an appropriate range
            /// to avoid such issues.
            ///
            /// The Taylor series for sine and cosine converge relatively quickly
            /// and uniformly over the entire domain.
            #[doc = TABLE_TAN_SERIES_TERMS!()]
            // NOTE: OPTIMIZED
            pub const fn tan_series(self, terms: $ue) -> Float<$f> {
                const PI: $f = Float::<$f>::PI.0;
                const HALF_PI: $f = Float::<$f>::FRAC_PI_2.0;
                const THRESHOLD: $f = 1e-6;
                let x = self.0 % PI; // Reduce angle to [-π/2, π/2]
                let x = is![x > HALF_PI; x - PI; is![x < -HALF_PI; x + PI; x]];
                // Handle near-asymptote cases using absolute difference
                let dist_to_asymptote = (x.abs() - HALF_PI).abs();
                is![dist_to_asymptote < THRESHOLD; return Float(<$f>::INFINITY.copysign(x))];
                let (sin, cos) = Float(x).sin_cos_series(terms);
                Float(sin.0 / cos.0)
            }

            /// Computes the arcsine using Taylor series expansion.
            ///
            /// # Formulation
            #[doc = FORMULA_ASIN_SERIES!()]
            ///
            /// asin is undefined for $ |x| > 1 $ and in that case returns `NaN`.
            ///
            /// The series converges more slowly near the edges of the domain
            /// (i.e., as `self` approaches -1 or 1). For more accurate results,
            /// especially near these boundary values, a higher number of terms
            /// may be necessary.
            ///
            /// See also [`asin_series_terms`][Self::asin_series_terms].
            pub const fn asin_series(self, terms: $ue) -> Float<$f> {
                is![self.abs().0 > 1.0; return Self::NAN];
                let (mut asin_approx, mut multiplier, mut power_x) = (0.0, 1.0, self.0);
                let mut i = 0;
                while i < terms {
                    if i != 0 {
                        multiplier *= (2 * i - 1) as $f / (2 * i) as $f;
                        power_x *= self.0 * self.0;
                    }
                    asin_approx += multiplier * power_x / (2 * i + 1) as $f;
                    i += 1;
                }
                Float(asin_approx)
            }

            /// Determines the number of terms needed for [`asin_series`][Self::asin_series]
            /// to reach a stable result based on the input value.
            ///
            #[doc = TABLE_ASIN_SERIES_TERMS!()]
            #[must_use]
            pub const fn asin_series_terms(self) -> $ue { paste! {
                Self::[<asin_acos_series_terms_ $f>](self.0)
            }}

            /// Computes the arccosine using the Taylor expansion of arcsine.
            ///
            /// # Formulation
            #[doc = FORMULA_ACOS_SERIES!()]
            ///
            /// See the [`asin_series_terms`][Self#method.asin_series_terms] table for
            /// information about the number of `terms` needed.
            pub const fn acos_series(self, terms: $ue) -> Float<$f> {
                is![self.abs().0 > 1.0; return Self::NAN];
                Float(Self::FRAC_PI_2.0 - self.asin_series(terms).0)
            }

            /// Determines the number of terms needed for [`acos_series`][Self::acos_series]
            /// to reach a stable result based on the input value.
            ///
            /// The table is the same as [`asin_series_terms`][Self::asin_series_terms].
            #[must_use]
            pub const fn acos_series_terms(self) -> $ue { paste! {
                Self::[<asin_acos_series_terms_ $f>](self.0)
            }}

            /// Computes the arctangent using Taylor series expansion.
            ///
            /// # Formulation
            #[doc = FORMULA_ATAN_SERIES!()]
            ///
            /// The series converges more slowly near the edges of the domain
            /// (i.e., as `self` approaches -1 or 1). For more accurate results,
            /// especially near these boundary values, a higher number of terms
            /// may be necessary.
            ///
            /// See also [`atan_series_terms`][Self::atan_series_terms].
            pub const fn atan_series(self, terms: $ue) -> Float<$f> {
                if self.abs().0 > 1.0 {
                    if self.0 > 0.0 {
                        Float(Self::FRAC_PI_2.0 - Float(1.0 / self.0).atan_series(terms).0)
                    } else {
                        Float(-Self::FRAC_PI_2.0 - Float(1.0 / self.0).atan_series(terms).0)
                    }
                } else {
                    let (mut atan_approx, mut num, mut sign) = (Self::ZERO.0, self.0, Self::ONE.0);
                    let mut i = 0;
                    while i < terms {
                        if i > 0 {
                            num *= self.0 * self.0;
                            sign = -sign;
                        }
                        atan_approx += sign * num / (2.0 * i as $f + 1.0);
                        i += 1;
                    }
                    Float(atan_approx)
                }
            }

            /// Determines the number of terms needed for [`atan_series`][Self::atan_series]
            /// to reach a stable result based on the input value.
            #[doc = TABLE_ATAN_SERIES_TERMS!()]
            #[must_use]
            pub const fn atan_series_terms(self) -> $ue { paste! {
                Self::[<atan_series_terms_ $f>](self.0)
            }}

            /// Computes the four quadrant arctangent of `self` and `other`
            /// using Taylor series expansion.
            ///
            /// See also [`atan_series_terms`][Self::atan_series_terms].
            pub const fn atan2_series(self, other: $f, terms: $ue) -> Float<$f> {
                if other > 0.0 {
                    Float(self.0 / other).atan_series(terms)
                } else if self.0 >= 0.0 && other < 0.0 {
                    Float(Float(self.0 / other).atan_series(terms).0 + Self::PI.0)
                } else if self.0 < 0.0 && other < 0.0 {
                    Float(Float(self.0 / other).atan_series(terms).0 - Self::PI.0)
                } else if self.0 > 0.0 && other == 0.0 {
                    Float(Self::PI.0 / 2.0)
                } else if self.0 < 0.0 && other == 0.0 {
                    Float(-Self::PI.0 / 2.0)
                } else {
                    // self and other are both zero, undefined behavior
                    Self::NAN
                }
            }

            /// The hyperbolic sine calculated using Taylor series expansion
            /// via the exponent formula.
            ///
            /// # Formulation
            #[doc = FORMULA_SINH_SERIES!()]
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            pub const fn sinh_series(self, terms: $ue) -> Float<$f> {
                Float((self.exp_series(terms).0 - -self.exp_series(terms).0) / 2.0)
            }

            /// The hyperbolic cosine calculated using Taylor series expansion
            /// via the exponent formula.
            ///
            /// # Formulation
            #[doc = FORMULA_COSH_SERIES!()]
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            pub const fn cosh_series(self, terms: $ue) -> Float<$f> {
                Float((self.exp_series(terms).0 + -self.exp_series(terms).0) / 2.0)
            }

            /// Computes the hyperbolic tangent using Taylor series expansion of
            /// hyperbolic sine and cosine.
            ///
            /// # Formulation
            #[doc = FORMULA_TANH_SERIES!()]
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            pub const fn tanh_series(self, terms: $ue) -> Float<$f> {
                let sinh_approx = self.sinh_series(terms);
                let cosh_approx = self.cosh_series(terms);
                Float(sinh_approx.0 / cosh_approx.0)
            }

            /// Computes the inverse hyperbolic sine using the natural logarithm definition.
            ///
            /// # Formulation
            #[doc = FORMULA_ASINH_SERIES!()]
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            pub const fn asinh_series(self, terms: $ue) -> Float<$f> {
                let sqrt = Float(self.0 * self.0 + 1.0).sqrt_hybrid().0;
                Float(self.0 + sqrt).ln_series(terms)
            }

            /// Computes the inverse hyperbolic cosine using the natural logarithm definition.
            ///
            /// # Formulation
            #[doc = FORMULA_ACOSH_SERIES!()]
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            pub const fn acosh_series(self, terms: $ue) -> Float<$f> {
                if self.0 < 1.0 {
                    Self::NAN
                } else {
                    let sqrt = Float(self.0 * self.0 - 1.0).sqrt_hybrid().0;
                    Float(self.0 + sqrt).ln_series(terms)
                }
            }

            /// Computes the inverse hyperbolic tangent using the natural logarithm definition.
            ///
            /// # Formulation
            #[doc = FORMULA_ATANH_SERIES!()]
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            pub const fn atanh_series(self, terms: $ue) -> Float<$f> {
                if self.0 >= 1.0 {
                    Self::INFINITY
                } else if self.0 <= -1.0 {
                    Self::NEG_INFINITY
                } else {
                    Float(Float((self.0 + 1.0) / (1.0 - self.0)).ln_series(terms).0 * 0.5)
                }
            }
        }
    };
}
impl_float_shared_series!();

crate::CONST! { pub(crate),
TABLE_EXP_SERIES_TERMS = "
The following table shows the required number of `terms` needed
to reach the most precise result for both `f32` and `f64`:
```txt
  value     t_f32  t_f64
-------------------------
± 0.001 →       3     5
± 0.100 →       6     10
± 1.000 →      11     18
± 10.000 →     32     46
± 20.000 →     49     68
± 50.000 →     92    119
± 88.722 →    143    177  (max for f32 == f32::MAX.ln())
± 150.000 →   ---    261
± 300.000 →   ---    453
± 500.000 →   ---    692
± 709.782 →   ---    938  (max for f64 == f64:MAX.ln())
```";
TABLE_EXP2_SERIES_TERMS = "
The following table shows the required number of `terms` needed
to reach the most precise result for both `f32` and `f64`:
```txt
  value     t_f32  t_f64
-------------------------
± 0.3 →        8     13
± 3.0 →       15     25
± 7.0 →       22     34
± 15.0 →      34     49
± 31.0 →      52     71
± 63.0 →      84    110
± 127.999 →  144    178 (max for f32)
± 255.0 →    ---    298
± 511.0 →    ---    520
± 1023.999 → ---    939 (max for f64)
```";
TABLE_LN_SERIES_TERMS = "
The following table shows the required number of `terms` needed
to reach the most precise result for both `f32` and `f64`:
```txt
  value      t_f32  t_f64
--------------------------
± 0.00001 →  81181 536609
± 0.0001 →   12578  59174
± 0.001 →     1923   6639
± 0.01. →      245    720
± 0.1 →         32     80
± 0.5 →          8     17
± 1. →           1      1
± 2. →           8     17
± 10. →         32     80
± 100. →       245    720
± 1000. →     1923   6639
± 10000. →   12578  59174
± 100000. →  81181 536609
/// ```
```";
TABLE_SIN_SERIES_TERMS = "
The following table shows the required number of `terms` needed
to reach the most precise result for both `f32` and `f64`:
```txt
  value     t_f32  t_f64
-------------------------
± 0.001 →      3      4
± 0.100 →      4      6
± 0.300 →      5      7
± 0.500 →      5      8
± 0.700 →      6      9
± 0.900 →      6     10
± 0.999 →      6     10
```";
TABLE_COS_SERIES_TERMS = "
The following table shows the required number of `terms` needed
to reach the most precise result for both `f32` and `f64`:
```txt
  value     t_f32  t_f64
-------------------------
± 0.001 →      3      4
± 0.100 →      4      6
± 0.300 →      5      8
± 0.500 →      6      9
± 0.700 →      6     10
± 0.900 →      7     10
± 0.999 →      7     11
```";
TABLE_TAN_SERIES_TERMS = "
The following table shows the required number of `terms` needed
to reach the most precise result for both `f32` and `f64`:
```txt
  value     t_f32  t_f64
-------------------------
± 0.001 →      3      4
± 0.100 →      4      6
± 0.300 →      5      8
± 0.500 →      6      9
± 0.700 →      6     10
± 0.900 →      7     10
± 0.999 →      7     11
```";
TABLE_ASIN_SERIES_TERMS = "
The following table shows the required number of `terms` needed
to reach the most precise result for both `f32` and `f64`:
```txt
  value     t_f32  t_f64
-------------------------
± 0.001 →      3      4
± 0.100 →      5      9
± 0.300 →      7     15
± 0.500 →     10     24
± 0.700 →     18     44
± 0.900 →     47    134
± 0.990 →    333   1235
± 0.999 →   1989  10768
```";
TABLE_ATAN_SERIES_TERMS = "
The following table shows the required number of `terms` needed
to reach the most precise result for both `f32` and `f64`:
```txt
  value     t_f32  t_f64
-------------------------
± 0.001 →      3      4
± 0.100 →      5      9
± 0.300 →      7     15
± 0.500 →     12     26
± 0.700 →     20     47
± 0.900 →     61    152
± 0.990 →    518   1466
± 0.999 →   4151  13604
```";
}

#[rustfmt::skip]
#[cfg(feature = "_float_f32")]
impl Float<f32> {
    #[must_use]
    pub(super) const fn asin_acos_series_terms_f32(x: f32) -> u32 {
        let abs_a = Float(x).abs().0;
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
        let abs_a = Float(x).abs().0;
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
        let abs_a = Float(x).abs().0;
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
        let abs_a = Float(x).abs().0;
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
        let x = Float(x).abs().0;
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
        let abs_a = Float(x).abs().0;
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
        let abs_a = Float(x).abs().0;
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
        let abs_a = Float(x).abs().0;
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
        let abs_a = Float(x).abs().0;
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
        let x = Float(x).abs().0;
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
