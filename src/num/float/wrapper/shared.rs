// devela::num::float::wrapper::shared
//
//! Shared methods
//

use crate::{cfor, concat as cc, iif, paste, stringify as sfy, Float, Sign};
#[allow(unused_imports)]
use {super::shared_helpers::*, crate::ExtFloat};

#[doc = crate::doc_private!()]
/// Implements methods independently of any features
///
/// $f:   the floating-point type.
/// $uf:  unsigned int type with the same bit-size.
/// $ue:  unsigned int type used for integer exponentiation and number of terms (u32).
/// $cap: the capability feature that enables the given implementation. E.g "_float_f32".
/// $cmp: the feature that enables some methods depending on Compare. E.g "_cmp_f32".
macro_rules! custom_impls {
    () => {
        custom_impls![
            (f32:u32, u32):"_float_f32":"_cmp_f32",
            (f64:u64, u32):"_float_f64":"_cmp_f64"
        ];
    };

    ($( ($f:ty:$uf:ty, $ue:ty) : $cap:literal : $cmp:literal ),+) => {
        $( custom_impls![@$f:$uf, $ue, $cap:$cmp]; )+
    };
    (@$f:ty:$uf:ty, $ue:ty, $cap:literal : $cmp:literal) => {
        #[doc = crate::doc_availability!(feature = $cap)]
        ///
        /// # *Common implementations with or without `std` or `libm`*.
        #[cfg(feature = $cap )]
        // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Float<$f> {
            /// Returns the nearest integer, rounding ties to the nearest odd integer.
            #[must_use]
            pub fn round_ties_odd(self) -> Float<$f> {
                let r = self.round_ties_away();
                iif![r.0 % 2.0 != 0.0; r ;
                    iif![(self - r).abs() == 0.5; r + self.signum(); r]]
            }

            /// Returns the [`Sign`].
            #[must_use]
            pub const fn sign(self) -> Sign {
                if self.is_sign_positive() { Sign::Positive } else { Sign::Negative }
            }

            /// Returns the [`Sign`], returning [`None`][Sign::None] for zero
            #[must_use]
            pub const fn sign_nonzero(self) -> Sign {
                if self.is_zero() {
                    Sign::None
                } else if self.is_sign_positive() {
                    Sign::Positive
                } else {
                    Sign::Negative
                }
            }

            /// Returns `true` if `self` has a positive sign.
            #[must_use]
            pub const fn is_sign_positive(self) -> bool { self.0.is_sign_positive() }

            /// Returns `true` if `self` has a negative sign.
            #[must_use]
            pub const fn is_sign_negative(self) -> bool { self.0.is_sign_negative() }

            /// Returns `true` if `self` is 0.0 or -0.0.
            #[must_use]
            pub const fn is_zero(self) -> bool {
                let non_sign_bits_mask = !(<$uf>::MAX / 2 + 1);
                (self.0.to_bits() & non_sign_bits_mask) == 0
            }

            /// Returns `true` if `self` has a positive sign and is not zero.
            #[must_use]
            pub const fn is_sign_positive_nonzero(self) -> bool {
                !self.is_zero() && self.is_sign_positive()
            }

            /// Returns `true` if `self` has a negative sign and is not zero.
            #[must_use]
            pub const fn is_sign_negative_nonzero(self) -> bool {
                !self.is_zero() && self.is_sign_negative()
            }

            /// Computes `(x * mul + add)` normally.
            #[must_use]
            pub const fn mul_add_fallback(self, mul: $f, add: $f) -> Float<$f> {
                Float(self.0 * mul + add)
            }

            /// The euclidean division.
            // NOTE: [incorrect computations](https://github.com/rust-lang/rust/issues/107904)
            #[must_use]
            pub fn div_euclid(self, other: $f) -> Float<$f> {
                let q = (self.0 / other).trunc();
                if self.0 % other < 0.0 {
                    iif![other > 0.0; Float(q - 1.0); Float(q + 1.0)]
                } else {
                    Float(q)
                }
            }

            /// The least non-negative remainder of `self` % `other`.
            // NOTE: [yield inconsistent results](https://github.com/rust-lang/rust/issues/111405)
            // WAIT:1.85 [const_float_methods](https://github.com/rust-lang/rust/pull/133389)
            #[must_use]
            pub const fn rem_euclid(self, other: $f) -> Float<$f> {
                let r = self.0 % other;
                iif![r < 0.0; Float(r + Float(other).const_abs().0); Float(r)]
            }

            /// Returns `self` between `[min..=max]` scaled to a new range `[u..=v]`.
            ///
            /// Values of `self` outside of `[min..=max]` are not clamped
            /// and will result in extrapolation.
            ///
            /// # Formula
            /// $$
            /// \large \text{scale}(x, min, max, u, v) = (v - u) \frac{x - min}{max - min} + u
            /// $$
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Float;
            #[doc = cc!["assert_eq![Float(45_", sfy![$f], ").scale(0., 360., 0., 1.), 0.125];"]]
            #[doc = cc!["assert_eq![Float(45_", sfy![$f], ").scale(0., 360., -1., 1.), -0.75];"]]
            #[doc = cc!["assert_eq![Float(0.125_", sfy![$f], ").scale(0., 1., 0., 360.), 45.];"]]
            #[doc = cc!["assert_eq![Float(-0.75_", sfy![$f], ").scale(-1., 1., 0., 360.), 45.];"]]
            /// ```
            #[must_use]
            pub const fn scale(self, min: $f, max: $f, u: $f, v: $f) -> Float<$f> {
                Float((v - u) * (self.0 - min) / (max - min) + u)
            }

            /// Calculates a linearly interpolated value between `u..=v`
            /// based on `self` as a percentage between `[0..=1]`.
            ///
            /// Values of `self` outside `[0..=1]` are not clamped
            /// and will result in extrapolation.
            ///
            /// # Formula
            /// $$ \large \text{lerp}(x, u, v) = (1 - x) \cdot u + x \cdot v $$
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Float;
            #[doc = cc!["assert_eq![Float(0.5_", sfy![$f], ").lerp(40., 80.), 60.];"]]
            /// ```
            #[must_use]
            pub const fn lerp(self, u: $f, v: $f) -> Float<$f> {
                Float((1.0 - self.0) * u + self.0 * v)
            }

            /// Raises itself to the `y` floating point power using the Taylor series via the
            /// `exp` and `ln` functions.
            ///
            /// $$ \large x^y = e^{y \cdot \ln(x)} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            ///
            /// The terms for the exponential function are calculated using
            /// [`exp_series_terms`][Self::exp_series_terms] using $y\cdot\ln(x)$.
            // WAIT:1.85 [const_float_methods](https://github.com/rust-lang/rust/pull/133389)
            #[must_use]
            pub const fn powf_series(self, y: $f, ln_x_terms: $ue) -> Float<$f> {
                let xabs = self.const_abs().0;
                if xabs == 0.0 {
                    iif![Float(y).const_abs().0 == 0.0; Self::ONE; Self::ZERO]
                } else {
                    let ln_x = Float(xabs).ln_series(ln_x_terms).0;
                    let power = Float(y * ln_x);
                    let exp_y_terms = power.exp_series_terms();
                    let result = power.exp_series(exp_y_terms);
                    iif![self.is_sign_negative(); Float(-result.0); result]
                }
            }

            /// $ \sqrt{x} $ The square root calculated using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            #[must_use]
            pub const fn sqrt_nr(self) -> Float<$f> {
                if self.0 < 0.0 {
                    Self::NAN
                } else if self.0 == 0.0 {
                    Self::ZERO
                } else {
                    let mut guess = self.0;
                    let mut guess_next = 0.5 * (guess + self.0 / guess);
                    while Self(guess - guess_next).const_abs().0 > Self::NR_TOLERANCE {
                        guess = guess_next;
                        guess_next = 0.5 * (guess + self.0 / guess);
                    }
                    Float(guess_next)
                }
            }

            /// $ \sqrt{x} $ the square root calculated using the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            #[must_use]
            pub const fn sqrt_fisr(self) -> Float<$f> { Float(1.0 / self.fisr().0) }

            /// $ 1 / \sqrt{x} $ the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            #[must_use]
            pub const fn fisr(self) -> Float<$f> {
                let (mut i, three_halfs, x2) = (self.0.to_bits(), 1.5, self.0 * 0.5);
                i = Self::FISR_MAGIC - (i >> 1);
                let y = <$f>::from_bits(i);
                Float(y * (three_halfs - (x2 * y * y)))
            }

            /// $ \sqrt\[3\]{x} $ The cubic root calculated using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            #[must_use]
            pub const fn cbrt_nr(self) -> Float<$f> {
                iif![self.0 == 0.0; return self];
                let mut guess = self.0;
                loop {
                    let next_guess = (2.0 * guess + self.0 / (guess * guess)) / 3.0;
                    if Float(next_guess - guess).const_abs().0 < Self::NR_TOLERANCE {
                        break Float(next_guess);
                    }
                    guess = next_guess;
                }
            }

            /// The hypothenuse (the euclidean distance) using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            ///
            /// $$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$
            #[must_use]
            pub const fn hypot_nr(self, y: $f) -> Float<$f> {
                Float(self.0 * self.0 + y * y).sqrt_nr()
            }

            /// The hypothenuse (the euclidean distance) using the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            ///
            /// $$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$
            #[must_use]
            pub const fn hypot_fisr(self, y: $f) -> Float<$f> {
                Float(self.0 * self.0 + y * y).sqrt_fisr()
            }

            /// Computes the exponential function $e^x$ using Taylor series expansion.
            ///
            /// $$ e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
            /// For values $ x < 0 $ it uses the identity: $$ e^x = \frac{1}{e^-x} $$
            ///
            /// See also [`exp_series_terms`][Self::exp_series_terms].
            #[must_use]
            pub const fn exp_series(self, terms: $ue) -> Float<$f> {
                iif![self.0 < 0.0; return Float(1.0 / Float(-self.0).exp_series(terms).0)];
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
            ///
            /// The following table shows the required number of `terms` needed
            /// to reach the most precise result for both `f32` and `f64`:
            /// ```txt
            ///   value     t_f32  t_f64
            /// -------------------------
            /// ± 0.001 →       3     5
            /// ± 0.100 →       6     10
            /// ± 1.000 →      11     18
            /// ± 10.000 →     32     46
            /// ± 20.000 →     49     68
            /// ± 50.000 →     92    119
            /// ± 88.722 →    143    177  (max for f32 == f32::MAX.ln())
            /// ± 150.000 →   ---    261
            /// ± 300.000 →   ---    453
            /// ± 500.000 →   ---    692
            /// ± 709.782 →   ---    938  (max for f64 == f64:MAX.ln())
            /// ```
            #[must_use]
            pub const fn exp_series_terms(self) -> $ue { paste! {
                Self::[<exp_series_terms_ $f>](self.0)
            }}

            /// Calculates $ e^x - 1 $ using the Taylor series expansion.
            ///
            /// $$ e^x -1 = x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
            /// For values $ x < 0 $ it uses the identity: $$ e^x -1 = -\frac{1}{e^{-x}+1} $$
            /// For values $ x > 0.001 $ it uses [`exp_series`][Self::exp_series].
            ///
            /// See also [`exp_series_terms`][Self::exp_series_terms].
            #[must_use]
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
            /// The series based on the formula $ 2^x = e^{x \ln(2)} $ is:
            /// $$
            /// 2^x = 1 + x \ln(2) + \frac{(x \ln(2))^2}{2!} +
            /// \frac{(x \ln(2))^3}{3!} + \frac{(x \ln(2))^4}{4!} + \cdots
            /// $$
            ///
            /// The maximum values with a representable result are:
            /// 127 for `f32` and 1023 for `f64`.
            #[must_use]
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
            ///
            /// The following table shows the required number of `terms` needed
            /// to reach the most precise result for both `f32` and `f64`:
            /// ```txt
            ///   value     t_f32  t_f64
            /// -------------------------
            /// ± 0.3 →        8     13
            /// ± 3.0 →       15     25
            /// ± 7.0 →       22     34
            /// ± 15.0 →      34     49
            /// ± 31.0 →      52     71
            /// ± 63.0 →      84    110
            /// ± 127.999 →  144    178 (max for f32)
            /// ± 255.0 →    ---    298
            /// ± 511.0 →    ---    520
            /// ± 1023.999 → ---    939 (max for f64)
            /// ```
            #[must_use]
            pub const fn exp2_series_terms(self) -> $ue { paste! {
                Self::[<exp2_series_terms_ $f>](self.0)
            }}

            /// Computes the natural logarithm of `self` using a Taylor-Mercator series expansion.
            ///
            /// This method is more efficient for values of `self` near 1. Values too
            /// small or too big could be impractical to calculate with precision.
            ///
            /// $$
            /// \ln(x) = 2 \left( \frac{x-1}{x+1} + \frac{1}{3} \left( \frac{x-1}{x+1} \right)^3 +
            /// \frac{1}{5} \left( \frac{x-1}{x+1} \right)^5 + \cdots \right)
            /// $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use]
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
            #[must_use]
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
            /// $$ \log_{\text{base}}(x) = \frac{\ln(x)}{\ln(\text{base})} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use]
            pub const fn log_series(self, base: $f, terms: $ue) -> Float<$f> {
                if base <= 0.0 {
                    Self::NAN
                // The logarithm with a base of 1 is undefined except when the argument is also 1.
                } else if Float(base - 1.0).const_abs().0 < Self::MEDIUM_MARGIN.0 { // + robust
                // } else if base == 1.0 { // good enough for direct input
                    #[expect(clippy::float_cmp, reason = "we've already checked it with a margin")]
                    { iif![self.0 == 1.0; Self::NAN; Self::NEG_INFINITY] }
                } else {
                    Float(self.ln_series(terms).0 / base).ln_series(terms)
                }
            }

            /// Computes the base-2 logarithm using the change of base formula.
            ///
            /// $$ \log_{2}(x) = \frac{\ln(x)}{\ln(2)} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use]
            pub const fn log2_series(self, terms: $ue) -> Float<$f> {
                Float(self.ln_series(terms).0 / 2.0).ln_series(terms)
            }

            /// Computes the base-10 logarithm using the change of base formula.
            ///
            /// $$ \log_{10}(x) = \frac{\ln(x)}{\ln(10)} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use]
            pub const fn log10_series(self, terms: $ue) -> Float<$f> {
                Float(self.ln_series(terms).0 / 10.0).ln_series(terms)
            }

            /// Determines the number of terms needed for [`exp2_series`][Self::exp2_series]
            /// to reach a stable result based on the input value.
            ///
            /// The following table shows the required number of `terms` needed
            /// to reach the most precise result for both `f32` and `f64`:
            /// ```txt
            ///   value      t_f32  t_f64
            /// --------------------------
            /// ± 0.00001 →  81181 536609
            /// ± 0.0001 →   12578  59174
            /// ± 0.001 →     1923   6639
            /// ± 0.01. →      245    720
            /// ± 0.1 →         32     80
            /// ± 0.5 →          8     17
            /// ± 1. →           1      1
            /// ± 2. →           8     17
            /// ± 10. →         32     80
            /// ± 100. →       245    720
            /// ± 1000. →     1923   6639
            /// ± 10000. →   12578  59174
            /// ± 100000. →  81181 536609
            /// ```
            #[must_use]
            pub const fn ln_series_terms(self) -> $ue { paste! {
                Self::[<ln_series_terms_ $f>](self.0)
            }}

            /// The factorial of the integer value `x`.
            ///
            /// The maximum values with a representable result are:
            /// 34 for `f32` and 170 for `f64`.
            ///
            /// Note that precision is poor for large values.
            #[must_use]
            pub const fn factorial(x: $ue) -> Float<$f> {
                let mut result = Self::ONE.0;
                // for i in 1..=x { result *= i as $f; }
                let mut i = 1;
                while i <= x {
                    result *= i as $f;
                    i += 1;
                }
                Float(result)
            }

            /// The sine calculated using Taylor series expansion.
            ///
            /// $$ \sin(x) = x - \frac{x^3}{3!} + \frac{x^5}{5!} - \frac{x^7}{7!} + \cdots $$
            ///
            /// This Taylor series converges relatively quickly and uniformly
            /// over the entire domain.
            ///
            /// The following table shows the required number of `terms` needed
            /// to reach the most precise result for both `f32` and `f64`:
            /// ```txt
            ///   value     t_f32  t_f64
            /// -------------------------
            /// ± 0.001 →      3      4
            /// ± 0.100 →      4      6
            /// ± 0.300 →      5      7
            /// ± 0.500 →      5      8
            /// ± 0.700 →      6      9
            /// ± 0.900 →      6     10
            /// ± 0.999 →      6     10
            /// ```
            #[must_use]
            pub const fn sin_series(self, terms: $ue) -> Float<$f> {
                let x = self.clamp_nan(-Self::PI.0, Self::PI.0).0;
                let (mut sin, mut term, mut factorial) = (x, x, 1.0);
                let mut i = 1;
                while i < terms {
                    term *= -x * x;
                    factorial *= ((2 * i + 1) * (2 * i)) as $f;
                    sin += term / factorial;
                    i += 1;
                }
                Float(sin)
            }

            /// Computes the cosine using taylor series expansion.
            ///
            /// $$ \cos(x) = 1 - \frac{x^2}{2!} + \frac{x^4}{4!} - \frac{x^6}{6!} + \cdots $$
            ///
            /// This Taylor series converges relatively quickly and uniformly
            /// over the entire domain.
            ///
            /// The following table shows the required number of `terms` needed
            /// to reach the most precise result for both `f32` and `f64`:
            /// ```txt
            ///   value     t_f32  t_f64
            /// -------------------------
            /// ± 0.001 →      3      4
            /// ± 0.100 →      4      6
            /// ± 0.300 →      5      8
            /// ± 0.500 →      6      9
            /// ± 0.700 →      6     10
            /// ± 0.900 →      7     10
            /// ± 0.999 →      7     11
            /// ```
            #[must_use]
            pub const fn cos_series(self, terms: $ue) -> Float<$f> {
                let x = self.clamp_nan(-Self::PI.0, Self::PI.0).0;
                let (mut cos, mut term, mut factorial) = (1.0, 1.0, 1.0);
                let mut i = 1;
                while i < terms {
                    term *= -x * x;
                    factorial *= ((2 * i - 1) * (2 * i)) as $f;
                    cos += term / factorial;
                    i += 1;
                }
                Float(cos)
            }

            /// Computes the sine and the cosine using Taylor series expansion.
            #[must_use]
            pub const fn sin_cos_series(self, terms: $ue) -> (Float<$f>, Float<$f>) {
                (self.sin_series(terms), self.cos_series(terms))
            }

            /// Computes the tangent using Taylor series expansion of sine and cosine.
            ///
            /// $$ \tan(x) = \frac{\sin(x)}{\cos(x)} $$
            ///
            /// The tangent function has singularities and is not defined for
            /// `cos(x) = 0`. This function clamps `self` within an appropriate range
            /// to avoid such issues.
            ///
            /// The Taylor series for sine and cosine converge relatively quickly
            /// and uniformly over the entire domain.
            ///
            /// The following table shows the required number of `terms` needed
            /// to reach the most precise result for both `f32` and `f64`:
            /// ```txt
            ///   value     t_f32  t_f64
            /// -------------------------
            /// ± 0.001 →      3      4
            /// ± 0.100 →      4      6
            /// ± 0.300 →      5      8
            /// ± 0.500 →      6      9
            /// ± 0.700 →      6     10
            /// ± 0.900 →      7     10
            /// ± 0.999 →      7     11
            /// ```
            #[must_use]
            pub const fn tan_series(self, terms: $ue) -> Float<$f> {
                let x = self.clamp_nan(-Self::PI.0 / 2.0 + 0.0001, Self::PI.0 / 2.0 - 0.0001);
                let (sin, cos) = x.sin_cos_series(terms);
                iif![cos.const_abs().0 < 0.0001; return Self::MAX];
                Float(sin.0 / cos.0)
            }

            /// Computes the arcsine using Taylor series expansion.
            ///
            /// $$
            /// \arcsin(x) = x + \left( \frac{1}{2} \right) \frac{x^3}{3} +
            /// \left( \frac{1}{2} \cdot \frac{3}{4} \right) \frac{x^5}{5} +
            /// \left( \frac{1}{2} \cdot \frac{3}{4} \cdot \frac{5}{6} \right) \frac{x^7}{7} +
            /// \cdots
            /// $$
            ///
            /// asin is undefined for $ |x| > 1 $ and in that case returns `NaN`.
            ///
            /// The series converges more slowly near the edges of the domain
            /// (i.e., as `self` approaches -1 or 1). For more accurate results,
            /// especially near these boundary values, a higher number of terms
            /// may be necessary.
            ///
            /// See also [`asin_series_terms`][Self::asin_series_terms].
            #[must_use]
            pub const fn asin_series(self, terms: $ue) -> Float<$f> {
                iif![self.const_abs().0 > 1.0; return Self::NAN];
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
            /// The following table shows the required number of `terms` needed
            /// to reach the most precise result for both `f32` and `f64`:
            /// ```txt
            ///   value     t_f32  t_f64
            /// -------------------------
            /// ± 0.001 →      3      4
            /// ± 0.100 →      5      9
            /// ± 0.300 →      7     15
            /// ± 0.500 →     10     24
            /// ± 0.700 →     18     44
            /// ± 0.900 →     47    134
            /// ± 0.990 →    333   1235
            /// ± 0.999 →   1989  10768
            /// ```
            #[must_use]
            pub const fn asin_series_terms(self) -> $ue { paste! {
                Self::[<asin_acos_series_terms_ $f>](self.0)
            }}

            /// Computes the arccosine using the Taylor expansion of arcsine.
            ///
            /// $$ \arccos(x)=2π-arcsin(x) $$
            ///
            /// See the [`asin_series_terms`][Self#method.asin_series_terms] table for
            /// information about the number of `terms` needed.
            #[must_use]
            pub const fn acos_series(self, terms: $ue) -> Float<$f> {
                iif![self.const_abs().0 > 1.0; return Self::NAN];
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
            /// $$ \arctan(x) = x - \frac{x^3}{3} + \frac{x^5}{5} - \frac{x^7}{7} + \cdots $$
            ///
            /// For values $ |x| > 1 $ it uses the identity:
            /// $$ \arctan(x) = \frac{\pi}{2} - \arctan(\frac{1}{x}) $$
            ///
            /// The series converges more slowly near the edges of the domain
            /// (i.e., as `self` approaches -1 or 1). For more accurate results,
            /// especially near these boundary values, a higher number of terms
            /// may be necessary.
            ///
            /// See also [`atan_series_terms`][Self::atan_series_terms].
            #[must_use]
            pub const fn atan_series(self, terms: $ue) -> Float<$f> {
                if self.const_abs().0 > 1.0 {
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
            ///
            /// The following table shows the required number of `terms` needed
            /// to reach the most precise result for both `f32` and `f64`:
            /// ```txt
            ///   value     t_f32  t_f64
            /// -------------------------
            /// ± 0.001 →      3      4
            /// ± 0.100 →      5      9
            /// ± 0.300 →      7     15
            /// ± 0.500 →     12     26
            /// ± 0.700 →     20     47
            /// ± 0.900 →     61    152
            /// ± 0.990 →    518   1466
            /// ± 0.999 →   4151  13604
            /// ```
            #[must_use]
            pub const fn atan_series_terms(self) -> $ue { paste! {
                Self::[<atan_series_terms_ $f>](self.0)
            }}

            /// Computes the four quadrant arctangent of `self` and `other`
            /// using Taylor series expansion.
            ///
            /// See also [`atan_series_terms`][Self::atan_series_terms].
            #[must_use]
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
            /// $$ \sinh(x) = \frac{e^x - e^{-x}}{2} $$
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            #[must_use]
            pub const fn sinh_series(self, terms: $ue) -> Float<$f> {
                Float((self.exp_series(terms).0 - -self.exp_series(terms).0) / 2.0)
            }

            /// The hyperbolic cosine calculated using Taylor series expansion
            /// via the exponent formula.
            ///
            /// $$ \cosh(x) = \frac{e^x + e^{-x}}{2} $$
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            #[must_use]
            pub const fn cosh_series(self, terms: $ue) -> Float<$f> {
                Float((self.exp_series(terms).0 + -self.exp_series(terms).0) / 2.0)
            }

            /// Computes the hyperbolic tangent using Taylor series expansion of
            /// hyperbolic sine and cosine.
            ///
            /// $$ \tanh(x) = \frac{\sinh(x)}{\cosh(x)} $$
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            #[must_use]
            pub const fn tanh_series(self, terms: $ue) -> Float<$f> {
                let sinh_approx = self.sinh_series(terms);
                let cosh_approx = self.cosh_series(terms);
                Float(sinh_approx.0 / cosh_approx.0)
            }

            /// Computes the inverse hyperbolic sine using the natural logarithm definition.
            ///
            /// $$ \text{asinh}(x) = \ln(x + \sqrt{x^2 + 1}) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use]
            pub const fn asinh_series(self, terms: $ue) -> Float<$f> {
                let sqrt = Float(self.0 * self.0 + 1.0).sqrt_nr().0;
                Float(self.0 + sqrt).ln_series(terms)
            }

            /// Computes the inverse hyperbolic cosine using the natural logarithm definition.
            ///
            /// $$ \text{acosh}(x) = \ln(x + \sqrt{x^2 - 1}) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use]
            pub const fn acosh_series(self, terms: $ue) -> Float<$f> {
                if self.0 < 1.0 {
                    Self::NAN
                } else {
                    let sqrt = Float(self.0 * self.0 - 1.0).sqrt_nr().0;
                    Float(self.0 + sqrt).ln_series(terms)
                }
            }

            /// Computes the inverse hyperbolic tangent using the natural logarithm definition.
            ///
            /// $$ \text{atanh}(x) = \frac{1}{2} \ln\left(\frac{1 + x}{1 - x}\right) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use]
            pub const fn atanh_series(self, terms: $ue) -> Float<$f> {
                if self.0 >= 1.0 {
                    Self::INFINITY
                } else if self.0 <= -1.0 {
                    Self::NEG_INFINITY
                } else {
                    Float(Float((self.0 + 1.0) / (1.0 - self.0)).ln_series(terms).0 * 0.5)
                }
            }

            /// The absolute value of `self` in constant-time.
            ///
            /// This is a separate function from [`abs`][Self::abs] because we also want to have
            /// the possibly more efficient `std` and `libm` implementations.
            // WAIT:1.85 [const_float_methods](https://github.com/rust-lang/rust/pull/133389)
            #[must_use]
            pub const fn const_abs(self) -> Float<$f> {
                let mask = <$uf>::MAX / 2;
                Float(<$f>::from_bits(self.0.to_bits() & mask))
            }

            /// The negative absolute value of `self` (sets its sign to be negative).
            #[must_use]
            pub const fn neg_abs(self) -> Float<$f> {
                if self.is_sign_negative() { self } else { self.flip_sign() }
            }

            /// Flips its sign.
            #[must_use]
            pub const fn flip_sign(self) -> Float<$f> {
                let sign_bit_mask = <$uf>::MAX / 2 + 1;
                Float(<$f>::from_bits(self.0.to_bits() ^ sign_bit_mask))
            }

            /// Returns itself clamped between `min` and `max`, ignoring `NaN`.
            ///
            /// See also: [`clamp_nan`][Self::clamp_nan], [`clamp_total`][Self::clamp_total].
            #[must_use]
            pub fn clamp(self, min: $f, max: $f) -> Float<$f> { self.max(min).min(max) }

            /// Returns itself clamped between `min` and `max`, using total order.
            ///
            /// See also: [`clamp`][Self::clamp], [`clamp_nan`][Self::clamp_nan].
            #[must_use]
            #[cfg(feature = $cmp)]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cmp)))]
            pub const fn clamp_total(self, min: $f, max: $f) -> Float<$f> {
                Float(crate::Compare(self.0).clamp(min, max))
            }

            /// Returns the maximum between itself and `other`, using total order.
            ///
            /// See also: [`max_nan`][Self::max_nan].
            #[must_use]
            #[cfg(feature = $cmp)]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cmp)))]
            pub const fn max_total(self, other: $f) -> Float<$f> {
                Float(crate::Compare(self.0).max(other))
            }

            /// Returns the minimum between itself and `other`, using total order.
            ///
            /// See also: [`min_nan`][Self::min_nan].
            #[must_use]
            #[cfg(feature = $cmp)]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cmp)))]
            pub fn min_total(self, other: $f) -> Float<$f> {
                Float(crate::Compare(self.0).min(other))
            }

            /// Returns itself clamped between `min` and `max`, propagating `NaN`.
            ///
            /// See also: [`clamp`][Self::clamp], [`clamp_total`][Self::clamp_total].
            #[must_use]
            pub const fn clamp_nan(self, min: $f, max: $f) -> Float<$f> {
                self.max_nan(min).min_nan(max)
            }

            /// Returns the maximum between itself and `other`, propagating `Nan`.
            ///
            /// See also: [`max_total`][Self::max_total].
            // WAIT: [float_minimum_maximum](https://github.com/rust-lang/rust/issues/91079)
            #[must_use]
            #[expect(clippy::float_cmp, reason = "TODO:CHECK:IMPROVE?")]
            pub const fn max_nan(self, other: $f) -> Float<$f> {
                if self.0 > other {
                    self
                } else if self.0 < other {
                    Float(other)
                } else if self.0 == other {
                    iif![self.is_sign_positive() && other.is_sign_negative(); self; Float(other)]
                } else {
                    // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
                    Float(self.0 + other)
                }
            }

            /// Returns the minimum between itself and `other`, propagating `Nan`.
            ///
            /// See also: [`min_total`][Self::min_total].
            // WAIT: [float_minimum_maximum](https://github.com/rust-lang/rust/issues/91079)
            #[must_use]
            #[expect(clippy::float_cmp, reason = "TODO:CHECK:IMPROVE?")]
            pub const fn min_nan(self, other: $f) -> Float<$f> {
                if self.0 < other {
                    self
                } else if self.0 < other {
                    Float(other)
                } else if self.0 == other {
                    iif![self.is_sign_negative() && other.is_sign_positive(); self; Float(other)]
                } else {
                    // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
                    Float(self.0 + other)
                }
            }

            /// Evaluates a polynomial at the `self` point value, using [Horner's method].
            ///
            /// Expects a slice of `coefficients` $[a_n, a_{n-1}, ..., a_1, a_0]$
            /// representing the polynomial $ a_n * x^n + a_{n-1} * x^{(n-1)} + ... + a_1 * x + a_0 $.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Float;
            /// let coefficients = [2.0, -6.0, 2.0, -1.0];
            #[doc = cc!["assert_eq![Float(3.0_", sfy![$f], ").eval_poly(&coefficients), 5.0];"]]
            #[doc = cc!["assert_eq![Float(3.0_", sfy![$f], ").eval_poly(&[]), 0.0];"]]
            /// ```
            ///
            /// [Horner's method]: https://en.wikipedia.org/wiki/Horner%27s_method#Polynomial_evaluation_and_long_division
            // WAIT: [for-loops in constants](https://github.com/rust-lang/rust/issues/87575)
            #[must_use]
            pub const fn eval_poly(self, coefficients: &[$f]) -> Float<$f> {
                let coef = coefficients;
                match coef.len() {
                    0 => Float(0.0),
                    1 => Float(coef[0]),
                    _ => {
                        let mut result = coef[0];
                        // non-const version:
                        // for &c in &coef[1..] {
                        //     result = result * self.0 + c;
                        // }
                        // const version:
                        let mut i = 1;
                        while i < coef.len() {
                            result = result * self.0 + coef[i];
                            i += 1;
                        }
                        Float(result)
                    }
                }
            }
        }
    };
}
custom_impls!();
