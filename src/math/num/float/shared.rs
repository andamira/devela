// devela::math::ops::float::shared

use super::Floating;
use crate::code::iif;

// Implements methods independently of any features
//
// $f: the floating-point type.
// $uf: unsigned int type with the same bit-size.
// $ue: unsigned int type used for integer exponentiation and number of terms (u32).
macro_rules! custom_impls {
    ($( ($f:ty:$uf:ty, $ue:ty) ),+) => { $( custom_impls![@$f:$uf, $ue]; )+ };
    (@$f:ty:$uf:ty, $ue:ty) => { $crate::code::paste! {
        /// # *Common implementations with or without `std` or `libm`*.
        /// # Features
        /// There are a few functions that can be *const* but only if either the
        /// `unsafe_math` or `unsafe_data` feature is enabled.
        impl Floating<$f> {
            /// Returns the nearest integer to `x`, rounding ties to the nearest even integer.
            // WAIT: https://github.com/rust-lang/rust/issues/96710
            #[must_use] #[inline]
            pub fn round_ties_even(x: $f) -> $f {
                let r = Self::round_ties_away(x);
                iif![r % 2.0 == 0.0; r ;
                    iif![Self::abs(x - r) == 0.5; r - Self::signum(x); r]]
            }

            /// Returns the nearest integer to `x`, rounding ties to the nearest odd integer.
            #[must_use] #[inline]
            pub fn round_ties_odd(x: $f) -> $f {
                let r = Self::round_ties_away(x);
                iif![r % 2.0 != 0.0; r ;
                    iif![Self::abs(x - r) == 0.5; r + Self::signum(x); r]]
            }

            /// Returns `true` if `x` is positive.
            #[must_use] #[inline]
            pub fn is_sign_positive(x: $f) -> bool { <$f>::is_sign_positive(x) }

            /// Returns `true` if `x` is negative.
            #[must_use] #[inline]
            pub fn is_sign_negative(x: $f) -> bool { <$f>::is_sign_negative(x) }

            /// Computes `(x * mul + add)` normally.
            #[must_use] #[inline]
            pub fn mul_add_fallback(x: $f, mul: $f, add: $f) -> $f { x * mul + add }

            /// The euclidean division.
            #[must_use] #[inline]
            pub fn div_euclid(x: $f, y: $f) -> $f {
                let q = Self::trunc(x / y);
                iif![x % y < 0.0; return iif![y > 0.0; q - 1.0; q + 1.0]]; q
            }

            /// The least nonnegative remainder of `x` % `y`.
            #[must_use] #[inline]
            pub fn rem_euclid(x: $f, y: $f) -> $f {
                let r = x % y; iif![r < 0.0; r + Self::abs(y); r]
            }

            /// Raises `x` to the `y` floating point power using the Taylor series via the
            /// `exp` and `ln` functins.
            ///
            /// $$ \large x^y = e^{y \cdot \ln(x)} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            ///
            /// The terms for the exponential function are calculated using
            /// [`exp_series_terms`][Self::exp_series_terms] using $y\cdot\ln(x)$.
            #[must_use] #[inline]
            pub fn powf_series(x: $f, y: $f, ln_x_terms: $ue) -> $f {
                let xabs = Self::abs(x);
                if xabs == 0.0 {
                    iif![Self::abs(y) == 0.0; 1.0; 0.0]
                } else {
                    let ln_x = Self::ln_series(xabs, ln_x_terms);
                    let power = y * ln_x;
                    let exp_y_terms = Self::exp_series_terms(power);
                    let result = Self::exp_series(power, exp_y_terms);
                    iif![x.is_sign_negative(); -result; result]
                }
            }

            /// $ \sqrt{x} $ The square root calculated using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            #[must_use] #[inline]
            pub fn sqrt_nr(x: $f) -> $f {
                if x < 0.0 {
                    $f::NAN
                } else if x == 0.0 {
                    0.0
                } else {
                    let mut guess = x;
                    let mut guess_next = 0.5 * (guess + x / guess);
                    while Self::abs(guess - guess_next) > Self::NR_TOLERANCE {
                        guess = guess_next;
                        guess_next = 0.5 * (guess + x / guess);
                    }
                    guess_next
                }
            }

            /// $ \sqrt{x} $ the square root calculated using the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            #[must_use] #[inline]
            pub fn sqrt_fisr(x: $f) -> $f { 1.0 / Self::fisr(x) }

            /// $ 1 / \sqrt{x} $ the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            #[must_use] #[inline]
            pub fn fisr(x: $f) -> $f {
                let (mut i, three_halfs, x2) = (x.to_bits(), 1.5, x * 0.5);
                i = Self::FISR_MAGIC - (i >> 1);
                let y = <$f>::from_bits(i);
                y * (three_halfs - (x2 * y * y))
            }

            /// $ \sqrt[3]{x} $ The cubic root calculated using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            #[must_use] #[inline]
            pub fn cbrt_nr(x: $f) -> $f {
                iif![x == 0.0; return 0.0];
                let mut guess = x;
                loop {
                    let next_guess = (2.0 * guess + x / (guess * guess)) / 3.0;
                    if Self::abs(next_guess - guess) < Self::NR_TOLERANCE {
                        break next_guess;
                    }
                    guess = next_guess;
                }
            }

            /// The hypothenuse (the euclidean distance) using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            ///
            /// $$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$
            #[must_use] #[inline]
            pub fn hypot_nr(x: $f, y: $f) -> $f { Self::sqrt_nr(x * x + y * y) }

            /// The hypothenuse (the euclidean distance) using the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            ///
            /// $$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$
            #[must_use] #[inline]
            pub fn hypot_fisr(x: $f, y: $f) -> $f { Self::sqrt_fisr(x * x + y * y) }

            /// Computes the exponential function $e^x$ using Taylor series expansion.
            ///
            /// $$ e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
            /// For values $ x < 0 $ it uses the identity: $$ e^x = \frac{1}{e^-x} $$
            ///
            /// See also [`exp_series_terms`][Self::exp_series_terms].
            #[must_use] #[inline]
            pub fn exp_series(x: $f, terms: $ue) -> $f {
                iif![x < 0.0; return 1.0 / Self::exp_series(-x, terms)];
                let (mut result, mut term) = (1.0, 1.0);
                for i in 1..=terms {
                    term *= x / i as $f;
                    result += term;
                }
                result
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
            #[must_use] #[inline(always)]
            pub fn exp_series_terms(x: $f) -> $ue { Self::[<exp_series_terms_ $f>](x) }

            /// Calculates $ e^x - 1 $ using the Taylor series expansion.
            ///
            /// $$ e^x -1 = x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
            /// For values $ x < 0 $ it uses the identity: $$ e^x -1 = -\frac{1}{e^{-x}+1} $$
            /// For values $ x > 0.001 $ it uses [`exp_series`][Self::exp_series].
            ///
            /// See also [`exp_series_terms`][Self::exp_series_terms].
            #[must_use] #[inline]
            pub fn exp_m1_series(x: $f, terms: $ue) -> $f {
                if x < 0.0 {
                    1.0 / Self::exp_m1_series(-x, terms)
                } else if x > 0.001 {
                    Self::exp_series(x, terms) - 1.0
                } else {
                    let (mut result, mut term, mut factorial) = (0.0, x, 1.0);
                    for i in 1..=terms {
                        result += term;
                        factorial *= (i + 1) as $f;
                        term *= x / factorial;
                    }
                    result
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
            #[must_use] #[inline]
            pub fn exp2_series(x: $f, terms: $ue) -> $f {
                let (mut result, mut term) = (1.0, x * Self::LN_2);
                for n in 1..terms {
                    result += term;
                    term *= x * Self::LN_2 / (n as $f + 1.0);
                }
                result
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
            #[must_use] #[inline(always)]
            pub fn exp2_series_terms(x: $f) -> $ue { Self::[<exp2_series_terms_ $f>](x) }

            /// Computes the natural logarithm of `x` using a Taylor-Mercator series expansion.
            ///
            /// This method is more efficient for values of `x` near 1. Values too
            /// small or too big could be impractical to calculate with precision.
            ///
            /// $$
            /// \ln(x) = 2 \left( \frac{x-1}{x+1} + \frac{1}{3} \left( \frac{x-1}{x+1} \right)^3 +
            /// \frac{1}{5} \left( \frac{x-1}{x+1} \right)^5 + \cdots \right)
            /// $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use] #[inline]
            pub fn ln_series(x: $f, terms: $ue) -> $f {
                if x < 0.0 {
                    $f::NAN
                } else if x > 0.0 {
                    let mut sum = 0.0;
                    let y = (x - 1.0) / (x + 1.0);
                    let mut y_pow = y;
                    for i in 0..terms {
                        sum += y_pow / (2 * i + 1) as $f;
                        y_pow *= y * y;
                    }
                    2.0 * sum
                } else {
                    $f::NEG_INFINITY
                }
            }

            /// Computes the natural logarithm of `1 + x` using a Taylor-Mercator series expansion.
            ///
            /// This method is more efficient for values of `x` near 0. Values too
            /// small or too big could be impractical to calculate with precision.
            ///
            /// Returns `ln(1+x)` more accurately than if the operations were performed separately.
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use] #[inline]
            pub fn ln_1p_series(x: $f, terms: $ue) -> $f {
                if x < -1.0 {
                    $f::NAN
                } else if x > -1.0 {
                    let x1 = x + 1.0;
                    let mut sum = 0.0;
                    let y = (x1 - 1.0) / (x1 + 1.0);
                    let mut y_pow = y;
                    for i in 0..terms {
                        sum += y_pow / (2 * i + 1) as $f;
                        y_pow *= y * y;
                    }
                    2.0 * sum
                } else {
                    $f::NEG_INFINITY
                }
            }

            /// Computes the logarithm of `x` to the given `base` using the change of base formula.
            ///
            /// $$ \log_{\text{base}}(x) = \frac{\ln(x)}{\ln(\text{base})} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use] #[inline]
            pub fn log_series(x: $f, base: $f, terms: $ue) -> $f {
                if base <= 0.0 {
                    $f::NAN
                } else if base == 1.0 {
                    iif![x == 1.0; $f::NAN; $f::NEG_INFINITY]
                } else {
                    Self::ln_series(x, terms) / Self::ln_series(base, terms)
                }
            }

            /// Computes the base-2 logarithm of `x` using the change of base formula.
            ///
            /// $$ \log_{2}(x) = \frac{\ln(x)}{\ln(2)} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use] #[inline]
            pub fn log2_series(x: $f, terms: $ue) -> $f {
                Self::ln_series(x, terms) / Self::ln_series(2.0, terms)
            }

            /// Computes the base-10 logarithm of `x` using the change of base formula.
            ///
            /// $$ \log_{10}(x) = \frac{\ln(x)}{\ln(10)} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use] #[inline]
            pub fn log10_series(x: $f, terms: $ue) -> $f {
                Self::ln_series(x, terms) / Self::ln_series(10.0, terms)
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
            #[must_use] #[inline(always)]
            pub fn ln_series_terms(x: $f) -> $ue {
                Self::[<ln_series_terms_ $f>](x)
            }

            /// The factorial of the integer value `x`.
            ///
            /// The maximum values with a representable result are:
            /// 34 for `f32` and 170 for `f64`.
            ///
            /// Note that precision is poor for large values.
            #[must_use] #[inline]
            pub fn factorial(x: $ue) -> $f {
                let mut result = 1.0;
                for i in 1..=x {
                    result *= i as $f;
                }
                result
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
            #[must_use] #[inline]
            pub fn sin_series(x: $f, terms: $ue) -> $f {
                let x = Self::clamp(x, -Self::PI, Self::PI);
                let (mut sin, mut term, mut factorial) = (x, x, 1.0);
                for i in 1..terms {
                    term *= -x * x;
                    factorial *= ((2 * i + 1) * (2 * i)) as $f;
                    sin += term / factorial;
                }
                sin
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
            #[must_use] #[inline]
            pub fn cos_series(x: $f, terms: $ue) -> $f {
                let x = Self::clamp(x, -Self::PI, Self::PI);
                let (mut cos, mut term, mut factorial) = (1.0, 1.0, 1.0);
                for i in 1..terms {
                    term *= -x * x;
                    factorial *= ((2 * i - 1) * (2 * i)) as $f;
                    cos += term / factorial;
                }
                cos
            }

            /// Computes the sine and the cosine using Taylor series expansion.
            #[must_use] #[inline]
            pub fn sin_cos_series(x: $f, terms: $ue) -> ($f, $f) {
                (Self::sin_series(x, terms), Self::cos_series(x, terms))
            }

            /// Computes the tangent using Taylor series expansion of sine and cosine.
            ///
            /// $$ \tan(x) = \frac{\sin(x)}{\cos(x)} $$
            ///
            /// The tangent function has singularities and is not defined for
            /// `cos(x) = 0`. This function clamps `x` within an appropriate range
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
            #[must_use] #[inline]
            pub fn tan_series(x: $f, terms: $ue) -> $f {
                let x = Self::clamp(x, -Self::PI / 2.0 + 0.0001, Self::PI / 2.0 - 0.0001);
                let (sin, cos) = Self::sin_cos_series(x, terms);
                iif![Self::abs(cos) < 0.0001; return $f::MAX];
                sin / cos
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
            /// (i.e., as `x` approaches -1 or 1). For more accurate results,
            /// especially near these boundary values, a higher number of terms
            /// may be necessary.
            ///
            /// See also [`asin_series_terms`][Self::asin_series_terms].
            #[must_use] #[inline]
            pub fn asin_series(x: $f, terms: $ue) -> $f {
                iif![Self::abs(x) > 1.0; return $f::NAN];
                let (mut asin_approx, mut multiplier, mut power_x) = (0.0, 1.0, x);
                for i in 0..terms {
                    if i != 0 {
                        multiplier *= (2 * i - 1) as $f / (2 * i) as $f;
                        power_x *= x * x;
                    }
                    asin_approx += multiplier * power_x / (2 * i + 1) as $f;
                }
                asin_approx
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
            #[must_use] #[inline(always)]
            pub fn asin_series_terms(x: $f) -> $ue { Self::[<asin_acos_series_terms_ $f>](x) }

            /// Computes the arccosine using the Taylor expansion of arcsine.
            ///
            /// $$ \arccos(x)=2π-arcsin(x) $$
            ///
            /// See the [`asin_series_terms`][Self#method.asin_series_terms] table for
            /// information about the number of `terms` needed.
            #[must_use] #[inline]
            pub fn acos_series(x: $f, terms: $ue) -> $f {
                iif![Self::abs(x) > 1.0; return $f::NAN];
                Self::FRAC_PI_2 - Self::asin_series(x, terms)
            }

            /// Determines the number of terms needed for [`acos_series`][Self::acos_series]
            /// to reach a stable result based on the input value.
            ///
            /// The table is the same as [`asin_series_terms`][Self::asin_series_terms].
            #[must_use] #[inline(always)]
            pub fn acos_series_terms(x: $f) -> $ue { Self::[<asin_acos_series_terms_ $f>](x) }

            /// Computes the arctangent using Taylor series expansion.
            ///
            /// $$ \arctan(x) = x - \frac{x^3}{3} + \frac{x^5}{5} - \frac{x^7}{7} + \cdots $$
            ///
            /// For values $ |x| > 1 $ it uses the identity:
            /// $$ \arctan(x) = \frac{\pi}{2} - \arctan(\frac{1}{x}) $$
            ///
            /// The series converges more slowly near the edges of the domain
            /// (i.e., as `x` approaches -1 or 1). For more accurate results,
            /// especially near these boundary values, a higher number of terms
            /// may be necessary.
            ///
            /// See also [`atan_series_terms`][Self::atan_series_terms].
            #[must_use] #[inline]
            pub fn atan_series(x: $f, terms: $ue) -> $f {
                if Self::abs(x) > 1.0 {
                    if x > 0.0 {
                        Self::FRAC_PI_2 - Self::atan_series(1.0 / x, terms)
                    } else {
                        -Self::FRAC_PI_2 - Self::atan_series(1.0 / x, terms)
                    }
                } else {
                    let (mut atan_approx, mut num, mut sign) = (0.0, x, 1.0);
                    for i in 0..terms {
                        if i > 0 {
                            num *= x * x;
                            sign = -sign;
                        }
                        atan_approx += sign * num / (2 * i + 1) as $f;
                    }
                    atan_approx
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
            #[must_use] #[inline(always)]
            pub fn atan_series_terms(x: $f) -> $ue { Self::[<atan_series_terms_ $f>](x) }

            /// Computes the four quadrant arctangent of `x` and `y` using Taylor series expansion.
            ///
            /// See also [`atan_series_terms`][Self::atan_series_terms].
            #[must_use] #[inline]
            pub fn atan2_series(x: $f, y: $f, terms: $ue) -> $f {
                if y > 0.0 {
                    Self::atan_series(x / y, terms)
                } else if x >= 0.0 && y < 0.0 {
                    Self::atan_series(x / y, terms) + Self::PI
                } else if x < 0.0 && y < 0.0 {
                    Self::atan_series(x / y, terms) - Self::PI
                } else if x > 0.0 && y == 0.0 {
                    Self::PI / 2.0
                } else if x < 0.0 && y == 0.0 {
                    -Self::PI / 2.0
                } else {
                    // x and y are both zero, undefined behavior
                    $f::NAN
                }
            }

            /// The hyperbolic sine calculated using Taylor series expansion
            /// via the exponent formula.
            ///
            /// $$ \sinh(x) = \frac{e^x - e^{-x}}{2} $$
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            #[must_use] #[inline]
            pub fn sinh_series(x: $f, terms: $ue) -> $f {
                (Self::exp_series(x, terms) - Self::exp_series(-x, terms)) / 2.0
            }

            /// The hyperbolic cosine calculated using Taylor series expansion
            /// via the exponent formula.
            ///
            /// $$ \cosh(x) = \frac{e^x + e^{-x}}{2} $$
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            #[must_use] #[inline]
            pub fn cosh_series(x: $f, terms: $ue) -> $f {
                (Self::exp_series(x, terms) + Self::exp_series(-x, terms)) / 2.0
            }

            /// Computes the hyperbolic tangent using Taylor series expansion of
            /// hyperbolic sine and cosine.
            ///
            /// $$ \tanh(x) = \frac{\sinh(x)}{\cosh(x)} $$
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            #[must_use] #[inline]
            pub fn tanh_series(x: $f, terms: $ue) -> $f {
                let sinh_approx = Self::sinh_series(x, terms);
                let cosh_approx = Self::cosh_series(x, terms);
                sinh_approx / cosh_approx
            }

            /// Computes the inverse hyperbolic sine using the natural logarithm definition.
            ///
            /// $$ \text{asinh}(x) = \ln(x + \sqrt{x^2 + 1}) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use] #[inline]
            pub fn asinh_series(x: $f, terms: $ue) -> $f {
                let sqrt = Self::sqrt_nr(x * x + 1.0);
                Self::ln_series(x + sqrt, terms)
            }

            /// Computes the inverse hyperbolic cosine using the natural logarithm definition.
            ///
            /// $$ \text{acosh}(x) = \ln(x + \sqrt{x^2 - 1}) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use] #[inline]
            pub fn acosh_series(x: $f, terms: $ue) -> $f {
                if x < 1.0 {
                    $f::NAN
                } else {
                    let sqrt = Self::sqrt_nr(x * x - 1.0);
                    Self::ln_series(x + sqrt, terms)
                }
            }

            /// Computes the inverse hyperbolic tangent using the natural logarithm definition.
            ///
            /// $$ \text{atanh}(x) = \frac{1}{2} \ln\left(\frac{1 + x}{1 - x}\right) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[must_use] #[inline]
            pub fn atanh_series(x: $f, terms: $ue) -> $f {
                if x >= 1.0 {
                    $f::INFINITY
                } else if x <= -1.0 {
                    $f::NEG_INFINITY
                } else {
                    0.5 * Self::ln_series((1.0 + x) / (1.0 - x), terms)
                }
            }

            /// The absolute value of `x` in constant-time.
            ///
            /// This is a separate function from [`abs`][Self::abs] because we also want to have
            /// the possibly more efficient `std` and `libm` implementations.
            #[cfg_attr(feature = "nightly",
                doc(cfg(any(feature = "unsafe_data", feature = "unsafe_math"))))]
            #[inline] #[must_use] #[cfg(any(feature = "unsafe_data", feature = "unsafe_math"))]
            pub const fn const_abs(x: $f) -> $f {
                let mask = <$uf>::MAX / 2;
                unsafe {
                    let bits: $uf = core::mem::transmute(x);
                    core::mem::transmute(bits & mask)
                }
            }

            /// Flips the sign of `x`.
            /// # Features
            /// This function will only be `const` if either the `unsafe_data`
            /// or `unsafe_math` feature is enabled.
            #[inline] #[must_use] #[cfg(any(feature = "unsafe_data", feature = "unsafe_math"))]
            pub const fn flip_sign(x: $f) -> $f {
                let mask = <$uf>::MAX / 2 + 1;
                unsafe {
                    let bits: $uf = core::mem::transmute(x);
                    core::mem::transmute(bits ^ mask)
                }
            }
            #[inline] #[must_use] #[allow(missing_docs)]
            #[cfg(not(any(feature = "unsafe_data", feature = "unsafe_math")))]
            pub fn flip_sign(x: $f) -> $f {
                let mask = <$uf>::MAX / 2 + 1;
                <$f>::from_bits(x.to_bits() ^ mask)
            }

            /// Returns the clamped value, ignoring `NaN`.
            #[must_use] #[inline(always)]
            pub fn clamp(value: $f, min: $f, max: $f) -> $f {
                Self::min(Self::max(value, min), max)
            }

            /// Returns the clamped value, using total order.
            /// # Features
            /// This function will only be `const` if either the `unsafe_data`
            /// or `unsafe_math` feature is enabled.
            #[inline] #[must_use] #[cfg(any(feature = "unsafe_data", feature = "unsafe_math"))]
            pub const fn clamp_total(value: $f, min: $f, max: $f) -> $f {
                $crate::data::Comparing(value).clamp(min, max)
            }
            #[inline] #[must_use] #[allow(missing_docs)]
            #[cfg(not(any(feature = "unsafe_data", feature = "unsafe_math")))]
            pub fn clamp_total(value: $f, min: $f, max: $f) -> $f {
                $crate::data::Comparing(value).clamp(min, max)
            }

            /// Returns the maximum of two numbers using total order.
            #[inline] #[must_use] #[cfg(any(feature = "unsafe_data", feature = "unsafe_math"))]
            pub const fn max_total(x: $f, y: $f) -> $f { $crate::data::Comparing(x).max(y) }
            #[inline] #[must_use] #[allow(missing_docs)]
            #[cfg(not(any(feature = "unsafe_data", feature = "unsafe_math")))]
            pub fn max_total(x: $f, y: $f) -> $f { $crate::data::Comparing(x).max(y) }

            /// Returns the minimum of two numbers using total order.
            #[inline] #[must_use] #[cfg(any(feature = "unsafe_data", feature = "unsafe_math"))]
            pub const fn min_total(x: $f, y: $f) -> $f { $crate::data::Comparing(x).min(y) }
            #[inline] #[must_use] #[allow(missing_docs)]
            #[cfg(not(any(feature = "unsafe_data", feature = "unsafe_math")))]
            pub fn min_total(x: $f, y: $f) -> $f { $crate::data::Comparing(x).min(y) }

            /// Returns the clamped `x` value, propagating `NaN`.
            #[must_use] #[inline(always)]
            pub fn clamp_nan(value: $f, min: $f, max: $f) -> $f {
                Self::min_nan(Self::max_nan(value, min), max)
            }

            /// Returns the maximum of two numbers, propagating `NaN`.
            // WAIT: https://github.com/rust-lang/rust/issues/91079
            #[must_use] #[inline(always)]
            pub fn max_nan(x: $f, y: $f) -> $f {
                if x > y {
                    x
                } else if y > x {
                    y
                } else if x == y {
                    if x.is_sign_positive() && y.is_sign_negative() { x } else { y }
                } else {
                    x + y
                }
            }
            /// Returns the minimum of two numbers, propagating `NaN`.
            #[must_use] #[inline(always)]
            // WAIT: https://github.com/rust-lang/rust/issues/91079
            pub fn min_nan(x: $f, y: $f) -> $f {
                if x < y {
                    x
                } else if y < x {
                    y
                } else if x == y {
                    if x.is_sign_negative() && y.is_sign_positive() { x } else { y }
                } else {
                    // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
                    x + y
                }
            }
        }
    }};
}
custom_impls![(f32:u32, u32), (f64:u64, u32)];

/* private helpers */

#[rustfmt::skip]
impl Floating<f32> {
    #[must_use] #[inline]
    pub(super) fn asin_acos_series_terms_f32(x: f32) -> u32 {
        let abs_a = Self::abs(x);
        if abs_a <= 0.1 { 5
        } else if abs_a <= 0.3 { 7
        } else if abs_a <= 0.5 { 10
        } else if abs_a <= 0.7 { 18
        } else if abs_a <= 0.9 { 47
        } else if abs_a <= 0.99 { 333
        } else { 1989 // computed for 0.999
        }
    }
    #[must_use] #[inline]
    pub(super) fn atan_series_terms_f32(x: f32) -> u32 {
        let abs_a = Self::abs(x);
        if abs_a <= 0.1 { 5
        } else if abs_a <= 0.3 { 7
        } else if abs_a <= 0.5 { 12
        } else if abs_a <= 0.7 { 20
        } else if abs_a <= 0.9 { 61
        } else if abs_a <= 0.99 { 518
        } else { 4151 // computed for 0.999
        }
    }
    #[must_use] #[inline]
    pub(super) fn exp_series_terms_f32(x: f32) -> u32 {
        let abs_a = Self::abs(x);
        if abs_a <= 0.001 { 3
        } else if abs_a <= 0.1 { 6
        } else if abs_a <= 1.0 { 11
        } else if abs_a <= 10.0 { 32
        } else if abs_a <= 20.0 { 49
        } else if abs_a <= 50.0 { 92
        } else { 143 // computed for max computable value f32::MAX.ln()
        }
    }
    #[must_use] #[inline]
    pub(super) fn exp2_series_terms_f32(x: f32) -> u32 {
        let abs_a = Self::abs(x);
        if abs_a <= 0.3 { 8
        } else if abs_a <= 3.0 { 15
        } else if abs_a <= 7.0 { 22
        } else if abs_a <= 15.0 { 34
        } else if abs_a <= 31.0 { 52
        } else if abs_a <= 63.0 { 84
        } else { 144 // computed for max computable value f64::MAX.ln()
        }
    }
    #[must_use] #[inline]
    pub(super) fn ln_series_terms_f32(x: f32) -> u32 {
        let x = Self::abs(x);
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
impl Floating<f64> {
    #[must_use] #[inline]
    pub(super) fn asin_acos_series_terms_f64(x: f64) -> u32 {
        let abs_a = Self::abs(x);
        if abs_a <= 0.1 { 9
        } else if abs_a <= 0.3 { 15
        } else if abs_a <= 0.5 { 24
        } else if abs_a <= 0.7 { 44
        } else if abs_a <= 0.9 { 134
        } else if abs_a <= 0.99 { 1235
        } else { 10768 // computed for 0.999
        }
    }
    #[must_use] #[inline]
    pub(super) fn atan_series_terms_f64(x: f64) -> u32 {
        let abs_a = Self::abs(x);
        if abs_a <= 0.1 { 9
        } else if abs_a <= 0.3 { 15
        } else if abs_a <= 0.5 { 26
        } else if abs_a <= 0.7 { 47
        } else if abs_a <= 0.9 { 152
        } else if abs_a <= 0.99 { 1466
        } else { 13604 // computed for 0.999
        }
    }
    #[must_use] #[inline]
    pub(super) fn exp_series_terms_f64(x: f64) -> u32 {
        let abs_a = Self::abs(x);
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
    #[must_use] #[inline]
    pub(super) fn exp2_series_terms_f64(x: f64) -> u32 {
        let abs_a = Self::abs(x);
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
    #[must_use] #[inline]
    pub(super) fn ln_series_terms_f64(x: f64) -> u32 {
        let x = Self::abs(x);
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
