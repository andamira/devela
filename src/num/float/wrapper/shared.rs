// devela::num::float::wrapper::shared
//
//! Shared methods
//

#[cfg(feature = "_-floats-_")]
use crate::{
    _core::{concat as cc, stringify as sfy},
    code::{iif, paste},
    num::{Compare, Float, Sign},
};
#[cfg(all(
    feature = "_-floats-_",
    not(feature = "safe_num"),
    feature = "unsafe_const"
))]
use core::mem::transmute;

// Implements methods independently of any features
//
// $f:   the floating-point type.
// $uf:  unsigned int type with the same bit-size.
// $ue:  unsigned int type used for integer exponentiation and number of terms (u32).
// $cap: the capability feature enables the given implementation. E.g "_f32".
macro_rules! custom_impls {
    ($( ($f:ty:$uf:ty, $ue:ty) : $cap:literal ),+) => {
        $( custom_impls![@$f:$uf, $ue, $cap]; )+
    };
    (@$f:ty:$uf:ty, $ue:ty, $cap:literal) => {
        /// # *Common implementations with or without `std` or `libm`*.
        /// # Features
        /// Several methods will only be *const* with the `unsafe_const` feature enabled.
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Float<$f> {
            /// Returns the nearest integer to `x`, rounding ties to the nearest even integer.
            // WAIT:1.77 [round_ties_even](https://github.com/rust-lang/rust/issues/96710)
            #[inline] #[must_use]
            pub fn round_ties_even(self) -> Float<$f> {
                let r = self.round_ties_away();
                iif![r.0 % 2.0 == 0.0; r;
                    iif![(self - r).abs() == 0.5; r - self.signum(); r]]
            }

            /// Returns the nearest integer, rounding ties to the nearest odd integer.
            #[inline] #[must_use]
            pub fn round_ties_odd(self) -> Float<$f> {
                let r = self.round_ties_away();
                iif![r.0 % 2.0 != 0.0; r ;
                    iif![(self - r).abs() == 0.5; r + self.signum(); r]]
            }

            /// Returns the [`Sign`].
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn sign(self) -> Sign {
                if self.is_sign_positive() { Sign::Positive } else { Sign::Negative }
            }
            /// Returns the [`Sign`].
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn sign(self) -> Sign {
                if self.is_sign_positive() { Sign::Positive } else { Sign::Negative }
            }
            /// Returns the [`Sign`], returning [`None`][Sign::None] for zero
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn sign_nonzero(self) -> Sign {
                if self.is_zero() {
                    Sign::None
                } else if self.is_sign_positive() {
                    Sign::Positive
                } else {
                    Sign::Negative
                }
            }
            /// Returns the [`Sign`], returning [`None`][Sign::None] for zero
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn sign_nonzero(self) -> Sign {
                if self.is_zero() {
                    Sign::None
                } else if self.is_sign_positive() {
                    Sign::Positive
                } else {
                    Sign::Negative
                }
            }

            /// Returns `true` if `self` has a positive sign.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn is_sign_positive(self) -> bool {
                // WAIT: [const_float_classify](https://github.com/rust-lang/rust/issues/72505)
                let bits: $uf = unsafe { transmute(self.0) };
                let sign_bit_mask = <$uf>::MAX / 2 + 1;
                (bits & sign_bit_mask) == 0 // if sign bit is not set it's a positive number or +0
            }
            /// Returns `true` if `self` has a positive sign.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn is_sign_positive(self) -> bool { self.0.is_sign_positive() }

            /// Returns `true` if `self` has a negative sign.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn is_sign_negative(self) -> bool {
                // WAIT: [const_float_classify](https://github.com/rust-lang/rust/issues/72505)
                let bits: $uf = unsafe { transmute(self.0) };
                let sign_bit_mask = <$uf>::MAX / 2 + 1;
                (bits & sign_bit_mask) != 0 // if sign bit is set it's a negative number or -0
            }
            /// Returns `true` if `self` has a negative sign.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn is_sign_negative(self) -> bool { self.0.is_sign_negative() }

            /// Returns `true` if `x` is 0.0 or -0.0.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn is_zero(self) -> bool {
                // WAIT: [const_float_bits_conv](https://github.com/rust-lang/rust/issues/72447)
                let bits: $uf = unsafe { transmute(self.0) };
                let non_sign_bits_mask = !(<$uf>::MAX / 2 + 1);
                (bits & non_sign_bits_mask) == 0
            }
            /// Returns `true` if `x` is 0.0 or -0.0.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn is_zero(self) -> bool {
                let non_sign_bits_mask = !(<$uf>::MAX / 2 + 1);
                (self.0.to_bits() & non_sign_bits_mask) == 0
            }

            /// Returns `true` if `x` has a positive sign and is not zero.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn is_sign_positive_nonzero(self) -> bool {
                !self.is_zero() && self.is_sign_positive()
            }
            /// Returns `true` if `x` has a positive sign and is not zero.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn is_sign_positive_nonzero(self) -> bool {
                !self.is_zero() && self.is_sign_positive()
            }

            /// Returns `true` if `x` has a negative sign and is not zero.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn is_sign_negative_nonzero(self) -> bool {
                !self.is_zero() && self.is_sign_negative()
            }
            /// Returns `true` if `x` has a negative sign and is not zero.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn is_sign_negative_nonzero(self) -> bool {
                !self.is_zero() && self.is_sign_negative()
            }

            /// Computes `(x * mul + add)` normally.
            #[inline] #[must_use]
            pub fn mul_add_fallback(self, mul: $f, add: $f) -> Float<$f> {
                self * mul + add
            }

            /// The euclidean division.
            #[inline] #[must_use]
            pub fn div_euclid(self, other: $f) -> Float<$f> {
                let q = (self / other).trunc();
                if self % other < 0.0 {
                    iif![other > 0.0; q - 1.0; q + 1.0]
                } else {
                    q
                }
            }

            /// The least nonnegative remainder of `self` % `other`.
            #[inline] #[must_use]
            pub fn rem_euclid(self, other: $f) -> Float<$f> {
                let r = self % other;
                iif![r < 0.0; r + Float(other).abs(); r]
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
            #[inline] #[must_use]
            pub fn scale(self, min: $f, max: $f, u: $f, v: $f) -> Float<$f> {
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
            #[doc = cc!{"assert_eq![Float(0.5_", sfy![$f], ").lerp(40., 80.), 60.];"}]
            /// ```
            #[inline] #[must_use]
            pub fn lerp(self, u: $f, v: $f) -> Float<$f> {
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
            #[inline] #[must_use]
            pub fn powf_series(self, y: $f, ln_x_terms: $ue) -> Float<$f> {
                let xabs = self.abs();
                if xabs == 0.0 {
                    iif![Float(y).abs() == 0.0; Self::ONE; Self::ZERO]
                } else {
                    let ln_x = xabs.ln_series(ln_x_terms).0;
                    let power = Float(y * ln_x);
                    let exp_y_terms = power.exp_series_terms();
                    let result = power.exp_series(exp_y_terms);
                    iif![self.is_sign_negative(); -result; result]
                }
            }

            /// $ \sqrt{x} $ The square root calculated using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            #[inline] #[must_use]
            pub fn sqrt_nr(self) -> Float<$f> {
                if self < 0.0 {
                    Self::NAN
                } else if self == 0.0 {
                    Self::ZERO
                } else {
                    let mut guess = self.0;
                    let mut guess_next = 0.5 * (guess + self.0 / guess);
                    while Self(guess - guess_next).abs() > Self::NR_TOLERANCE {
                        guess = guess_next;
                        guess_next = 0.5 * (guess + self.0 / guess);
                    }
                    Float(guess_next)
                }
            }

            /// $ \sqrt{x} $ the square root calculated using the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            #[inline] #[must_use]
            pub fn sqrt_fisr(self) -> Float<$f> { Float(1.0 / self.fisr().0) }

            /// $ 1 / \sqrt{x} $ the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            #[inline] #[must_use]
            pub fn fisr(self) -> Float<$f> {
                let (mut i, three_halfs, x2) = (self.0.to_bits(), 1.5, self.0 * 0.5);
                i = Self::FISR_MAGIC - (i >> 1);
                let y = <$f>::from_bits(i);
                Float(y * (three_halfs - (x2 * y * y)))
            }

            /// $ \sqrt[3]{x} $ The cubic root calculated using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            #[inline] #[must_use]
            pub fn cbrt_nr(self) -> Float<$f> {
                iif![self == 0.0; return self];
                let mut guess = self.0;
                loop {
                    let next_guess = (2.0 * guess + self.0 / (guess * guess)) / 3.0;
                    if Float(next_guess - guess).abs() < Self::NR_TOLERANCE {
                        break Float(next_guess);
                    }
                    guess = next_guess;
                }
            }

            /// The hypothenuse (the euclidean distance) using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            ///
            /// $$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$
            #[inline] #[must_use]
            pub fn hypot_nr(self, y: $f) -> Float<$f> { (self * self + y * y).sqrt_nr() }

            /// The hypothenuse (the euclidean distance) using the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            ///
            /// $$ \text{hypot}(x, y) = \sqrt{x^2 + y^2} $$
            #[inline] #[must_use]
            pub fn hypot_fisr(self, y: $f) -> Float<$f> { (self * self + y * y).sqrt_fisr() }

            /// Computes the exponential function $e^x$ using Taylor series expansion.
            ///
            /// $$ e^x = 1 + x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
            /// For values $ x < 0 $ it uses the identity: $$ e^x = \frac{1}{e^-x} $$
            ///
            /// See also [`exp_series_terms`][Self::exp_series_terms].
            #[inline] #[must_use]
            pub fn exp_series(self, terms: $ue) -> Float<$f> {
                iif![self < 0.0; return Float(1.0 / (-self).exp_series(terms).0)];
                let (mut result, mut term) = (1.0, 1.0);
                for i in 1..=terms {
                    term *= self.0 / i as $f;
                    result += term;
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
            #[inline] #[must_use]
            pub fn exp_series_terms(self) -> $ue { paste! {
                Self::[<exp_series_terms_ $f>](self.0)
            }}

            /// Calculates $ e^x - 1 $ using the Taylor series expansion.
            ///
            /// $$ e^x -1 = x + \frac{x^2}{2!} + \frac{x^3}{3!} + \frac{x^4}{4!} + \cdots $$
            /// For values $ x < 0 $ it uses the identity: $$ e^x -1 = -\frac{1}{e^{-x}+1} $$
            /// For values $ x > 0.001 $ it uses [`exp_series`][Self::exp_series].
            ///
            /// See also [`exp_series_terms`][Self::exp_series_terms].
            #[inline] #[must_use]
            pub fn exp_m1_series(self, terms: $ue) -> Float<$f> {
                if self < 0.0 {
                    Float(1.0 / (-self).exp_m1_series(terms).0)
                } else if self > 0.001 {
                    self.exp_series(terms)- 1.0
                } else {
                    let (mut result, mut term, mut factorial) = (0.0, self.0, 1.0);
                    for i in 1..=terms {
                        result += term;
                        factorial *= (i + 1) as $f;
                        term *= self.0 / factorial;
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
            #[inline] #[must_use]
            pub fn exp2_series(self, terms: $ue) -> Float<$f> {
                let (mut result, mut term) = (1.0, self.0 * Self::LN_2.0);
                for n in 1..terms {
                    result += term;
                    term *= self.0 * Self::LN_2.0 / (n as $f + 1.0);
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
            #[inline] #[must_use]
            pub fn exp2_series_terms(self) -> $ue { paste! {
                Self::[<exp2_series_terms_ $f>](self.0)
            }}

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
            #[inline] #[must_use]
            pub fn ln_series(self, terms: $ue) -> Float<$f> {
                if self < 0.0 {
                    Self::NAN
                } else if self > 0.0 {
                    let mut sum = 0.0;
                    let y = (self.0 - 1.0) / (self.0 + 1.0);
                    let mut y_pow = y;
                    for i in 0..terms {
                        sum += y_pow / (2 * i + 1) as $f;
                        y_pow *= y * y;
                    }
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
            #[inline] #[must_use]
            pub fn ln_1p_series(self, terms: $ue) -> Float<$f> {
                if self < -1.0 {
                    Self::NAN
                } else if self > -1.0 {
                    let x1 = self.0 + 1.0;
                    let mut sum = 0.0;
                    let y = (x1 - 1.0) / (x1 + 1.0);
                    let mut y_pow = y;
                    for i in 0..terms {
                        sum += y_pow / (2 * i + 1) as $f;
                        y_pow *= y * y;
                    }
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
            #[inline] #[must_use]
            pub fn log_series(self, base: $f, terms: $ue) -> Float<$f> {
                if base <= 0.0 {
                    Self::NAN
                } else if base == 1.0 {
                    iif![self == 1.0; Self::NAN; Self::NEG_INFINITY]
                } else {
                    self.ln_series(terms) / Float(base).ln_series(terms)
                }
            }

            /// Computes the base-2 logarithm using the change of base formula.
            ///
            /// $$ \log_{2}(x) = \frac{\ln(x)}{\ln(2)} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[inline] #[must_use]
            pub fn log2_series(self, terms: $ue) -> Float<$f> {
                self.ln_series(terms) / Float::<$f>(2.0).ln_series(terms)
            }

            /// Computes the base-10 logarithm using the change of base formula.
            ///
            /// $$ \log_{10}(x) = \frac{\ln(x)}{\ln(10)} $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[inline] #[must_use]
            pub fn log10_series(self, terms: $ue) -> Float<$f> {
                self.ln_series(terms) / Float::<$f>(10.0).ln_series(terms)
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
            #[inline] #[must_use]
            pub fn ln_series_terms(self) -> $ue { paste! {
                Self::[<ln_series_terms_ $f>](self.0)
            }}

            /// The factorial of the integer value `x`.
            ///
            /// The maximum values with a representable result are:
            /// 34 for `f32` and 170 for `f64`.
            ///
            /// Note that precision is poor for large values.
            #[inline] #[must_use]
            pub fn factorial(x: $ue) -> Float<$f> {
                let mut result = Self::ONE;
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
            #[inline] #[must_use]
            pub fn sin_series(self, terms: $ue) -> Float<$f> {
                let x = self.clamp(-Self::PI.0, Self::PI.0).0;
                let (mut sin, mut term, mut factorial) = (x, x, 1.0);
                for i in 1..terms {
                    term *= -x * x;
                    factorial *= ((2 * i + 1) * (2 * i)) as $f;
                    sin += term / factorial;
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
            #[inline] #[must_use]
            pub fn cos_series(self, terms: $ue) -> Float<$f> {
                let x = self.clamp(-Self::PI.0, Self::PI.0).0;
                let (mut cos, mut term, mut factorial) = (1.0, 1.0, 1.0);
                for i in 1..terms {
                    term *= -x * x;
                    factorial *= ((2 * i - 1) * (2 * i)) as $f;
                    cos += term / factorial;
                }
                Float(cos)
            }

            /// Computes the sine and the cosine using Taylor series expansion.
            #[inline] #[must_use]
            pub fn sin_cos_series(self, terms: $ue) -> (Float<$f>, Float<$f>) {
                (self.sin_series(terms), self.cos_series(terms))
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
            #[inline] #[must_use]
            pub fn tan_series(self, terms: $ue) -> Float<$f> {
                let x = self.clamp(-Self::PI.0 / 2.0 + 0.0001, Self::PI.0 / 2.0 - 0.0001);
                let (sin, cos) = x.sin_cos_series(terms);
                iif![cos.abs() < 0.0001; return Self::MAX];
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
            /// (i.e., as `x` approaches -1 or 1). For more accurate results,
            /// especially near these boundary values, a higher number of terms
            /// may be necessary.
            ///
            /// See also [`asin_series_terms`][Self::asin_series_terms].
            #[inline] #[must_use]
            pub fn asin_series(self, terms: $ue) -> Float<$f> {
                iif![self.abs() > 1.0; return Self::NAN];
                let (mut asin_approx, mut multiplier, mut power_x) = (0.0, 1.0, self.0);
                for i in 0..terms {
                    if i != 0 {
                        multiplier *= (2 * i - 1) as $f / (2 * i) as $f;
                        power_x *= self.0 * self.0;
                    }
                    asin_approx += multiplier * power_x / (2 * i + 1) as $f;
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
            #[inline] #[must_use]
            pub fn asin_series_terms(self) -> $ue { paste! {
                Self::[<asin_acos_series_terms_ $f>](self.0)
            }}

            /// Computes the arccosine using the Taylor expansion of arcsine.
            ///
            /// $$ \arccos(x)=2π-arcsin(x) $$
            ///
            /// See the [`asin_series_terms`][Self#method.asin_series_terms] table for
            /// information about the number of `terms` needed.
            #[inline] #[must_use]
            pub fn acos_series(self, terms: $ue) -> Float<$f> {
                iif![self.abs() > 1.0; return Self::NAN];
                Self::FRAC_PI_2 - self.asin_series(terms)
            }

            /// Determines the number of terms needed for [`acos_series`][Self::acos_series]
            /// to reach a stable result based on the input value.
            ///
            /// The table is the same as [`asin_series_terms`][Self::asin_series_terms].
            #[inline] #[must_use]
            pub fn acos_series_terms(self) -> $ue { paste! {
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
            /// (i.e., as `x` approaches -1 or 1). For more accurate results,
            /// especially near these boundary values, a higher number of terms
            /// may be necessary.
            ///
            /// See also [`atan_series_terms`][Self::atan_series_terms].
            #[inline] #[must_use]
            pub fn atan_series(self, terms: $ue) -> Float<$f> {
                if self.abs() > 1.0 {
                    if self > 0.0 {
                        Self::FRAC_PI_2 - Float(1.0 / self.0).atan_series(terms)
                    } else {
                        -Self::FRAC_PI_2 - Float(1.0 / self.0).atan_series(terms)
                    }
                } else {
                    let (mut atan_approx, mut num, mut sign) = (Self::ZERO, self, Self::ONE);
                    for i in 0..terms {
                        if i > 0 {
                            num *= self * self;
                            sign = -sign;
                        }
                        atan_approx += sign * num / (2.0 * i as $f + 1.0);
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
            #[inline] #[must_use]
            pub fn atan_series_terms(self) -> $ue { paste! {
                Self::[<atan_series_terms_ $f>](self.0)
            }}

            /// Computes the four quadrant arctangent of `self` and `other`
            /// using Taylor series expansion.
            ///
            /// See also [`atan_series_terms`][Self::atan_series_terms].
            #[inline] #[must_use]
            pub fn atan2_series(self, other: $f, terms: $ue) -> Float<$f> {
                if other > 0.0 {
                    (self / other).atan_series(terms)
                } else if self >= 0.0 && other < 0.0 {
                    (self / other).atan_series(terms) + Self::PI
                } else if self < 0.0 && other < 0.0 {
                    (self / other).atan_series(terms) - Self::PI
                } else if self > 0.0 && other == 0.0 {
                    Self::PI / 2.0
                } else if self < 0.0 && other == 0.0 {
                    -Self::PI / 2.0
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
            #[inline] #[must_use]
            pub fn sinh_series(self, terms: $ue) -> Float<$f> {
                (self.exp_series(terms) - -self.exp_series(terms)) / 2.0
            }

            /// The hyperbolic cosine calculated using Taylor series expansion
            /// via the exponent formula.
            ///
            /// $$ \cosh(x) = \frac{e^x + e^{-x}}{2} $$
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            #[inline] #[must_use]
            pub fn cosh_series(self, terms: $ue) -> Float<$f> {
                (self.exp_series(terms) + -self.exp_series(terms)) / 2.0
            }

            /// Computes the hyperbolic tangent using Taylor series expansion of
            /// hyperbolic sine and cosine.
            ///
            /// $$ \tanh(x) = \frac{\sinh(x)}{\cosh(x)} $$
            ///
            /// See the [`exp_series_terms`][Self#method.exp_series_terms] table for
            /// information about the number of `terms` needed.
            #[inline] #[must_use]
            pub fn tanh_series(self, terms: $ue) -> Float<$f> {
                let sinh_approx = self.sinh_series(terms);
                let cosh_approx = self.cosh_series(terms);
                sinh_approx / cosh_approx
            }

            /// Computes the inverse hyperbolic sine using the natural logarithm definition.
            ///
            /// $$ \text{asinh}(x) = \ln(x + \sqrt{x^2 + 1}) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[inline] #[must_use]
            pub fn asinh_series(self, terms: $ue) -> Float<$f> {
                let sqrt = (self * self + 1.0).sqrt_nr();
                (self + sqrt).ln_series(terms)
            }

            /// Computes the inverse hyperbolic cosine using the natural logarithm definition.
            ///
            /// $$ \text{acosh}(x) = \ln(x + \sqrt{x^2 - 1}) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[inline] #[must_use]
            pub fn acosh_series(self, terms: $ue) -> Float<$f> {
                if self < 1.0 {
                    Self::NAN
                } else {
                    let sqrt = (self * self - 1.0).sqrt_nr();
                    (self + sqrt).ln_series(terms)
                }
            }

            /// Computes the inverse hyperbolic tangent using the natural logarithm definition.
            ///
            /// $$ \text{atanh}(x) = \frac{1}{2} \ln\left(\frac{1 + x}{1 - x}\right) $$
            ///
            /// See also [`ln_series_terms`][Self::ln_series_terms].
            #[inline] #[must_use]
            pub fn atanh_series(self, terms: $ue) -> Float<$f> {
                if self >= 1.0 {
                    Self::INFINITY
                } else if self <= -1.0 {
                    Self::NEG_INFINITY
                } else {
                    ((self + 1.0) / (1.0 - self.0)).ln_series(terms) * 0.5
                }
            }

            /// The absolute value of `self` in constant-time.
            ///
            /// This is a separate function from [`abs`][Self::abs] because we also want to have
            /// the possibly more efficient `std` and `libm` implementations.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_const")))]
            pub const fn const_abs(self) -> Float<$f> {
                let mask = <$uf>::MAX / 2;
                unsafe {
                    let bits: $uf = transmute(self.0);
                    Float(transmute(bits & mask))
                }
            }

            /// The negative absolute value of `self` (sets its sign to be negative).
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn neg_abs(self) -> Float<$f> {
                if self.is_sign_negative() { self } else { self.flip_sign() }
            }
            /// The negative absolute value of `self` (sets its sign to be negative).
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn neg_abs(self) -> Float<$f> {
                if self.is_sign_negative() { self } else { self.flip_sign() }
            }

            /// Flips its sign.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn flip_sign(self) -> Float<$f> {
                let sign_bit_mask = <$uf>::MAX / 2 + 1;
                unsafe {
                    let bits: $uf = transmute(self.0);
                    Float(transmute(bits ^ sign_bit_mask))
                }
            }
            /// Flips its sign.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn flip_sign(self) -> Float<$f> {
                let sign_bit_mask = <$uf>::MAX / 2 + 1;
                Float(<$f>::from_bits(self.0.to_bits() ^ sign_bit_mask))
            }

            /// Returns itself clamped between `min` and `max`, ignoring `NaN`.
            #[inline] #[must_use]
            pub fn clamp(self, min: $f, max: $f) -> Float<$f> { self.max(min).min(max) }

            /// Returns itself clamped between `min` and `max`, using total order.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn clamp_total(self, min: $f, max: $f) -> Float<$f> {
                Float(Compare(self.0).clamp(min, max))
            }
            /// Returns itself clamped between `min` and `max`, using total order.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn clamp_total(self, min: $f, max: $f) -> Float<$f> {
                Float(Compare(self.0).clamp(min, max))
            }

            /// Returns the maximum between itself and `other`, using total order.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn max_total(self, other: $f) -> Float<$f> {
                Float(Compare(self.0).max(other))
            }
            /// Returns the maximum between itself and `other`, using total order.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn max_total(self, other: $f) -> Float<$f> {
                Float(Compare(self.0).max(other))
            }

            /// Returns the minimum between itself and `other`, using total order.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_const"))]
            pub const fn min_total(self, other: $f) -> Float<$f> {
                Float(Compare(self.0).min(other))
            }
            /// Returns the minimum between itself and `other`, using total order.
            /// # Features
            /// This function will only be `const` with the `unsafe_const` feature enabled.
            #[inline] #[must_use]
            #[cfg(any(feature = "safe_num", not(feature = "unsafe_const")))]
            pub fn min_total(self, other: $f) -> Float<$f> {
                Float(Compare(self.0).min(other))
            }

            /// Returns itself clamped between `min` and `max`, propagating `NaN`.
            #[inline] #[must_use]
            pub fn clamp_nan(self, min: $f, max: $f) -> Float<$f> {
                self.max_nan(min).min_nan(max)
            }

            /// Returns the maximum between itself and `other`, propagating `Nan`.
            // WAIT: [float_minimum_maximum](https://github.com/rust-lang/rust/issues/91079)
            #[inline] #[must_use]
            pub fn max_nan(self, other: $f) -> Float<$f> {
                if self > other {
                    self
                } else if self < other {
                    Float(other)
                } else if self == other {
                    iif![self.is_sign_positive() && other.is_sign_negative(); self; Float(other)]
                } else {
                    // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
                    Float(self.0 + other)
                }
            }
            /// Returns the minimum between itself and `other`, propagating `Nan`.
            #[inline] #[must_use]
            // WAIT: [float_minimum_maximum](https://github.com/rust-lang/rust/issues/91079)
            pub fn min_nan(self, other: $f) -> Float<$f> {
                if self < other {
                    self
                } else if self < other {
                    Float(other)
                } else if self == other {
                    iif![self.is_sign_negative() && other.is_sign_positive(); self; Float(other)]
                } else {
                    // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
                    Float(self.0 + other)
                }
            }
        }
    };
}
custom_impls![(f32:u32, u32):"_f32", (f64:u64, u32):"_f64"];

/* private helpers */

#[rustfmt::skip]
#[cfg(feature = "_f32")]
impl Float<f32> {
    #[inline] #[must_use]
    pub(super) fn asin_acos_series_terms_f32(x: f32) -> u32 {
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
    #[inline] #[must_use]
    pub(super) fn atan_series_terms_f32(x: f32) -> u32 {
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
    #[inline] #[must_use]
    pub(super) fn exp_series_terms_f32(x: f32) -> u32 {
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
    #[inline] #[must_use]
    pub(super) fn exp2_series_terms_f32(x: f32) -> u32 {
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
    #[inline] #[must_use]
    pub(super) fn ln_series_terms_f32(x: f32) -> u32 {
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
#[cfg(feature = "_f64")]
impl Float<f64> {
    #[inline] #[must_use]
    pub(super) fn asin_acos_series_terms_f64(x: f64) -> u32 {
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
    #[inline] #[must_use]
    pub(super) fn atan_series_terms_f64(x: f64) -> u32 {
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
    #[inline] #[must_use]
    pub(super) fn exp_series_terms_f64(x: f64) -> u32 {
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
    #[inline] #[must_use]
    pub(super) fn exp2_series_terms_f64(x: f64) -> u32 {
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
    #[inline] #[must_use]
    pub(super) fn ln_series_terms_f64(x: f64) -> u32 {
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
