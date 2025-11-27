// devela::num::float::wrapper::shared
//
//! Defines all the shared, cross-platform public methods for `Float`.
//
// WAIT: [core_float_math](https://github.com/rust-lang/rust/issues/137578)

use crate::{
    _FloatInternals, Cmp, Float, FloatCategory, Sign, concat as cc, is, stringify as sfy, whilst,
};

/// Implements methods independently of any features
///
/// $f:   the floating-point type.
/// $uf:  unsigned int type with the same bit-size.
/// $ie:  signed int type used for integer exponentiation.
/// $ue:  unsigned int type used for integer exponentiation and number of terms (u32).
macro_rules! impl_float_shared {
    () => {
        impl_float_shared![ (f32:u32, i32, u32), (f64:u64, i32, u32)];
        // #[cfg(feature = "nightly_float")] // TODO
        // impl_float_shared![ (f16:u16, u32), (f128:u128, u32)];
    };

    ($( ($f:ty:$uf:ty, $ie:ty, $ue:ty)),+) => {
        $( impl_float_shared![@$f:$uf, $ie, $ue]; )+
    };
    (@$f:ty:$uf:ty, $ie:ty, $ue:ty) => {
        /// # *Common implementations with or without `std`*.
        impl Float<$f> {
            /// The largest integer less than or equal to itself.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_FLOOR!()]
            pub const fn const_floor(self) -> Float<$f> {
                let mut result = self.const_trunc().0;
                if self.0.is_sign_negative() && Float(self.0 - result).abs().0 > <$f>::EPSILON {
                    result -= 1.0;
                }
                Float(result)
            }

            /// The smallest integer greater than or equal to itself.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_CEIL!()]
            pub const fn const_ceil(self) -> Float<$f> {
                let mut result = self.const_trunc().0;
                if self.0.is_sign_positive() && Float(self.0 - result).abs().0 > <$f>::EPSILON {
                    result += 1.0;
                }
                Float(result)
            }

            /// The nearest integer to itself, default rounding
            ///
            /// This is the default [`round_ties_away`][Self::round_ties_away] implementation.
            pub const fn const_round(self) -> Float<$f> { self.const_round_ties_away() }

            /// The nearest integer to itself, rounding ties away from `0.0`.
            ///
            /// This is the default `round` method implementation, without `std`.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_ROUND_TIES_AWAY!()]
            pub const fn const_round_ties_away(self) -> Float<$f> {
                Float(self.0 +
                    Float(0.5 - 0.25 * <$f>::EPSILON).copysign(self.0).0)
                        .const_trunc()
            }

            /// Returns the nearest integer to `x`, rounding ties to the nearest even integer.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_ROUND_TIES_EVEN!()]
            pub const fn const_round_ties_even(self) -> Float<$f> {
                let r = self.const_round_ties_away();
                if r.0 % 2.0 == 0.0 {
                    r
                } else {
                    #[allow(clippy::float_cmp, reason = "IMPROVE")]
                    if Float(self.0 - r.0).abs().0 == 0.5 { // -0.5 < error_margin
                        Float(r.0 - self.signum().0)
                    } else {
                        r
                    }
                }
            }

            /// The integral part.
            /// This means that non-integer numbers are always truncated towards zero.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_TRUNC!()]
            ///
            /// This implementation uses bitwise manipulation to remove the fractional part
            /// of the floating-point number. The exponent is extracted, and a mask is
            /// created to remove the fractional part. The new bits are then used to create
            /// the truncated floating-point number.
            pub const fn const_trunc(self) -> Float<$f> {
                let bits = self.0.to_bits();
                const BIAS: $ie = _FloatInternals::<$f>::EXPONENT_BIAS as $ie;
                const SIG_BITS: $ie = _FloatInternals::<$f>::SIGNIFICAND_BITS as $ie;
                const EXP_MASK: $uf = (1 << _FloatInternals::<$f>::EXPONENT_BITS) - 1;

                #[allow(clippy::cast_possible_wrap)]
                let exponent = (((bits >> SIG_BITS) & EXP_MASK) as $ie) - BIAS;
                if exponent < 0 {
                    is![self.0.is_sign_positive(); Float(0.0); Float(-0.0)]
                } else if exponent < SIG_BITS {
                    let mask = !(((1 as $uf) << (SIG_BITS - exponent)) - 1);
                    let new_bits = bits & mask;
                    Float(<$f>::from_bits(new_bits))
                } else {
                    self
                }
            }

            /// Returns the nearest integer, rounding ties to the nearest odd integer.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_ROUND_TIES_ODD!()]
            #[allow(clippy::float_cmp, reason = "RETHINK (self.0 - r.0).abs() == 0.5")]
            pub const fn round_ties_odd(self) -> Float<$f> {
                let r = self.const_round_ties_away();
                is![r.0 % 2.0 != 0.0; r ;
                    is![(self.0 - r.0).abs() == 0.5; Float(r.0 + self.0.signum()); r]]
            }

            /// Returns the floating point category of the number.
            pub const fn classify(self) -> FloatCategory { self.0.classify() }

            /// The fractional part.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_FRACT!()]
            pub const fn const_fract(self) -> Float<$f> { Float(self.0 - self.const_trunc().0) }

            /// The integral and fractional parts.
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_SPLIT!()]
            pub const fn const_split(self) -> (Float<$f>, Float<$f>) {
                let trunc = self.const_trunc();
                (trunc, Float(self.0 - trunc.0))
            }

            /// A number that represents its sign, propagating `NaN`.
            pub const fn signum(self) -> Float<$f> { Float(self.0.signum()) }
            #[allow(missing_docs)]
            #[deprecated(since = "0.23.0", note = "use `signum()` instead")]
            pub const fn const_signum(self) -> Float<$f> { self.signum() }
            // if self.0.is_nan() { Float(<$f>::NAN) } else { Self::ONE.copysign(self.0) }

            /// A number composed of the magnitude of itself and the `sign` of other.
            pub const fn copysign(self, sign: $f) -> Float<$f> { Float(self.0.copysign(sign)) }
            #[allow(missing_docs)]
            #[deprecated(since = "0.23.0", note = "use `copysign()` instead")]
            pub const fn const_copysign(self, sign: $f) -> Float<$f> { self.copysign(sign) }
            // const SIGN_MASK: $uf = <$uf>::MAX / 2 + 1;
            // const VALUE_MASK: $uf = <$uf>::MAX / 2;
            // let sign_bit = sign.to_bits() & SIGN_MASK;
            // let value_bits = self.0.to_bits() & VALUE_MASK;
            // Float(<$f>::from_bits(value_bits | sign_bit))

            /// Returns the [`Sign`].
            pub const fn sign(self) -> Sign {
                if self.is_sign_positive() { Sign::Positive } else { Sign::Negative }
            }

            /// Returns the [`Sign`].
            pub const fn sign_nonzero(self) -> Sign {
                if self.is_zero() {
                    Sign::Zero
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
            pub const fn mul_add_fallback(self, mul: $f, add: $f) -> Float<$f> {
                Float(self.0 * mul + add)
            }

            /// The euclidean division.
            // NOTE: [incorrect computations](https://github.com/rust-lang/rust/issues/107904)
            pub const fn div_euclid(self, other: $f) -> Float<$f> {
                let q = Float(self.0 / other).const_trunc().0;
                if self.0 % other < 0.0 {
                    is![other > 0.0; Float(q - 1.0); Float(q + 1.0)]
                } else {
                    Float(q)
                }
            }

            /// The least non-negative remainder of `self` % `other`.
            // NOTE: [yield inconsistent results](https://github.com/rust-lang/rust/issues/111405)
            pub const fn rem_euclid(self, other: $f) -> Float<$f> {
                let r = self.0 % other;
                is![r < 0.0; Float(r + Float(other).abs().0); Float(r)]
            }

            /// Returns `self` between `[min..=max]` scaled to a new range `[u..=v]`.
            ///
            /// Also known as *remap*.
            ///
            /// Values of `self` outside of `[min..=max]` are not clamped
            /// and will result in extrapolation.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_SCALE!()]
            /// # Examples
            /// ```
            /// # use devela::Float;
            #[doc = cc!["assert_eq![Float(45_", sfy![$f], ").scale(0., 360., 0., 1.), 0.125];"]]
            #[doc = cc!["assert_eq![Float(45_", sfy![$f], ").scale(0., 360., -1., 1.), -0.75];"]]
            #[doc = cc!["assert_eq![Float(0.125_", sfy![$f], ").scale(0., 1., 0., 360.), 45.];"]]
            #[doc = cc!["assert_eq![Float(-0.75_", sfy![$f], ").scale(-1., 1., 0., 360.), 45.];"]]
            /// ```
            pub const fn scale(self, min: $f, max: $f, u: $f, v: $f) -> Float<$f> {
                Float((v - u) * (self.0 - min) / (max - min) + u)
            }

            /// Calculates a linearly interpolated value between `u..=v`
            /// based on `self` as a percentage between `[0..=1]`.
            ///
            /// Values of `self` outside `[0..=1]` are not clamped
            /// and will result in extrapolation.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_LERP!()]
            /// # Example
            /// ```
            /// # use devela::Float;
            #[doc = cc!["assert_eq![Float(0.5_", sfy![$f], ").lerp(40., 80.), 60.];"]]
            // TODO more examples extrapolated
            /// ```
            pub const fn lerp(self, u: $f, v: $f) -> Float<$f> {
                Float((1.0 - self.0) * u + self.0 * v)
            }

            /// $ 1 / \sqrt{x} $ the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            pub const fn fisr(self) -> Float<$f> {
                let (mut i, three_halfs, x2) = (self.0.to_bits(), 1.5, self.0 * 0.5);
                i = _FloatInternals::<$f>::FISR_MAGIC - (i >> 1);
                let y = <$f>::from_bits(i);
                Float(y * (three_halfs - (x2 * y * y)))
            }

            /// $ \sqrt{x} $ The square root calculated calling both [`sqrt_fisr`] and [`sqrt_nr`].
            ///
            /// [`sqrt_fisr`]: Self::sqrt_fisr
            /// [`sqrt_nr`]: Self::sqrt_nr
            pub const fn sqrt_hybrid(self) -> Float<$f> {
                is![self.0 < 0.0; return Self::NAN; is![self.0 == 0.0; return Self::ZERO]];
                let y = self.fisr().0; // fast path using fisr + newton refinement
                let mut x = self.0 * y; // initial estimate: x ~= sqrt(n)
                // 1 newton iteration some times can be enough:
                // Float(0.5 * (x + self.0 / x))
                // But 2 iterations gives much better precision:
                whilst![_i in 0..2; x = 0.5 * (x + self.0 / x)];
                Float(x)
            }

            /// $ \sqrt{x} $ The square root calculated using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            pub const fn sqrt_nr(self) -> Float<$f> {
                if self.0 < 0.0 {
                    Self::NAN
                } else if self.0 == 0.0 {
                    Self::ZERO
                } else {
                    let mut guess = self.0;
                    let mut guess_next = 0.5 * (guess + self.0 / guess);
                    while Self(guess - guess_next).abs().0 > _FloatInternals::<$f>::NR_TOLERANCE {
                        guess = guess_next;
                        guess_next = 0.5 * (guess + self.0 / guess);
                    }
                    Float(guess_next)
                }
            }

            /// $ \sqrt{x} $ the square root calculated using the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            pub const fn sqrt_fisr(self) -> Float<$f> { Float(1.0 / self.fisr().0) }

            /// The hypothenuse (the euclidean distance) using the
            /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_HYPOT_FISR!()]
            pub const fn hypot_fisr(self, y: $f) -> Float<$f> {
                Float(self.0 * self.0 + y * y).sqrt_fisr()
            }

            /// The hypothenuse (the euclidean distance) using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_HYPOT_NR!()]
            pub const fn hypot_nr(self, y: $f) -> Float<$f> {
                Float(self.0 * self.0 + y * y).sqrt_nr()
            }

            /// $ \sqrt\[3\]{x} $ The cubic root calculated using the
            /// [Newton-Raphson method](https://en.wikipedia.org/wiki/Newton%27s_method).
            pub const fn cbrt_nr(self) -> Float<$f> {
                is![self.0 == 0.0; return self];
                let mut guess = self.0;
                loop {
                    let next_guess = (2.0 * guess + self.0 / (guess * guess)) / 3.0;
                    if Float(next_guess - guess).abs().0 < _FloatInternals::<$f>::NR_TOLERANCE {
                        break Float(next_guess);
                    }
                    guess = next_guess;
                }
            }

            /// The factorial of the integer value `x`.
            ///
            /// The maximum values with a representable result are:
            /// 34 for `f32` and 170 for `f64`.
            ///
            /// Note that precision is poor for large values.
            pub const fn factorial(x: $ue) -> Float<$f> {
                let mut result = Self::ONE.0;
                whilst![i in 1..{x+1}; result *= i as $f];
                Float(result)
            }

            /// The absolute value of `self`.
            pub const fn abs(self) -> Float<$f> {
                // let mask = <$uf>::MAX / 2;
                // Float(<$f>::from_bits(self.0.to_bits() & mask))
                Self(self.0.abs())
            }

            /// The negative absolute value of `self` (sets its sign to be negative).
            pub const fn neg_abs(self) -> Float<$f> {
                if self.is_sign_negative() { self } else { self.flip_sign() }
            }

            /// Flips its sign.
            pub const fn flip_sign(self) -> Float<$f> {
                let sign_bit_mask = <$uf>::MAX / 2 + 1;
                Float(<$f>::from_bits(self.0.to_bits() ^ sign_bit_mask))
            }

            /// Returns the least number smaller than self.
            pub const fn next_down(self) -> Float<$f> { Self(self.0.next_down()) }

            /// Returns the least number greater than self.
            pub const fn next_up(self) -> Float<$f> { Self(self.0.next_up()) }

            /// Takes the reciprocal (inverse), $1/x$.
            pub const fn recip(self) -> Float<$f> { Self(self.0.recip()) }

            /// Converts radians to degrees.
            pub const fn to_degrees(self) -> Float<$f> { Self(self.0.to_degrees()) }

            /// Converts degrees to radians.
            pub const fn to_radians(self) -> Float<$f> { Self(self.0.to_radians()) }

            /// Returns itself clamped between `min` and `max`, ignoring `NaN`.
            ///
            /// # Example
            /// ```
            /// # use devela::Float;
            #[doc = cc!["assert_eq![Float(50.0_", sfy![$f], ").clamp(40., 80.), 50.];"]]
            #[doc = cc!["assert_eq![Float(100.0_", sfy![$f], ").clamp(40., 80.), 80.];"]]
            #[doc = cc!["assert_eq![Float(10.0_", sfy![$f], ").clamp(40., 80.), 40.];"]]
            /// ```
            /// See also: [`clamp_nan`][Self::clamp_nan], [`clamp_total`][Self::clamp_total].
            pub const fn clamp(self, min: $f, max: $f) -> Float<$f> {
                // self.max(min).min(max)
                Self(self.0.clamp(min, max))
            }

            /// The maximum between itself and `other`, ignoring `NaN`.
            pub const fn max(self, other: $f) -> Float<$f> {
                // if self.0.is_nan() || self.0 < other { Float(other) } else { self }
                Self(self.0.max(other))
            }

            /// The minimum between itself and other, ignoring `NaN`.
            pub const fn min(self, other: $f) -> Float<$f> {
                // if other.is_nan() || self.0 < other { self } else { Float(other) }
                Self(self.0.min(other))
            }
            ///
            #[deprecated(since = "0.23.0", note = "use `clamp()` instead")]
            pub const fn const_clamp(self, min: $f, max: $f) -> Float<$f> { self.clamp(min, max) }
            ///
            #[deprecated(since = "0.23.0", note = "use `max()` instead")]
            pub const fn const_max(self, other: $f) -> Float<$f> { self.max(other) }
            ///
            #[deprecated(since = "0.23.0", note = "use `min()` instead")]
            pub const fn const_min(self, other: $f) -> Float<$f> { self.min(other) }

            /// Returns itself clamped between `min` and `max`, using total order.
            ///
            /// # Example
            /// ```
            /// # use devela::Float;
            #[doc = cc!["assert_eq![Float(50.0_", sfy![$f], ").clamp_total(40., 80.), 50.];"]]
            #[doc = cc!["assert_eq![Float(100.0_", sfy![$f], ").clamp_total(40., 80.), 80.];"]]
            #[doc = cc!["assert_eq![Float(10.0_", sfy![$f], ").clamp_total(40., 80.), 40.];"]]
            /// ```
            /// See also: [`clamp`][Self::clamp], [`clamp_nan`][Self::clamp_nan].
            pub const fn clamp_total(self, min: $f, max: $f) -> Float<$f> {
                Float(Cmp(self.0).clamp(min, max))
            }

            /// Returns the maximum between itself and `other`, using total order.
            ///
            /// See also: [`max_nan`][Self::max_nan].
            pub const fn max_total(self, other: $f) -> Float<$f> {
                Float(Cmp(self.0).max(other))
            }

            /// Returns the minimum between itself and `other`, using total order.
            ///
            /// See also: [`min_nan`][Self::min_nan].
            pub const fn min_total(self, other: $f) -> Float<$f> {
                Float(Cmp(self.0).min(other))
            }

            /// Returns itself clamped between `min` and `max`, propagating `NaN`.
            ///
            /// # Example
            /// ```
            /// # use devela::Float;
            #[doc = cc!["assert_eq![Float(50.0_", sfy![$f], ").clamp_nan(40., 80.), 50.];"]]
            #[doc = cc!["assert_eq![Float(100.0_", sfy![$f], ").clamp_nan(40., 80.), 80.];"]]
            #[doc = cc!["assert_eq![Float(10.0_", sfy![$f], ").clamp_nan(40., 80.), 40.];"]]
            /// ```
            /// See also: [`clamp`][Self::clamp], [`clamp_total`][Self::clamp_total].
            pub const fn clamp_nan(self, min: $f, max: $f) -> Float<$f> {
                self.max_nan(min).min_nan(max)
            }

            /// Returns the maximum between itself and `other`, propagating `Nan`.
            ///
            /// # Example
            /// ```
            /// # use devela::Float;
            #[doc = cc!["assert_eq![Float(50.0_", sfy![$f], ").max_nan(80.), 80.];"]]
            #[doc = cc!["assert_eq![Float(100.0_", sfy![$f], ").max_nan(80.), 100.];"]]
            /// ```
            /// See also: [`max_total`][Self::max_total].
            // WAIT: [float_minimum_maximum](https://github.com/rust-lang/rust/issues/91079)
            #[expect(clippy::float_cmp, reason = "TODO:CHECK:IMPROVE?")]
            pub const fn max_nan(self, other: $f) -> Float<$f> {
                if self.0 > other {
                    self
                } else if self.0 < other {
                    Float(other)
                } else if self.0 == other {
                    is![self.is_sign_positive() && other.is_sign_negative(); self; Float(other)]
                } else {
                    // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
                    Float(self.0 + other)
                }
            }

            /// Returns the minimum between itself and `other`, propagating `Nan`.
            ///
            /// # Example
            /// ```
            /// # use devela::Float;
            #[doc = cc!["assert_eq![Float(50.0_", sfy![$f], ").min_nan(80.), 50.];"]]
            #[doc = cc!["assert_eq![Float(100.0_", sfy![$f], ").min_nan(80.), 80.];"]]
            /// ```
            /// See also: [`min_total`][Self::min_total].
            // WAIT: [float_minimum_maximum](https://github.com/rust-lang/rust/issues/91079)
            #[expect(clippy::float_cmp, reason = "TODO:CHECK:IMPROVE?")]
            pub const fn min_nan(self, other: $f) -> Float<$f> {
                if self.0 < other {
                    self
                } else if self.0 > other {
                    Float(other)
                } else if self.0 == other {
                    is![self.is_sign_negative() && other.is_sign_positive(); self; Float(other)]
                } else {
                    // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
                    Float(self.0 + other)
                }
            }

            /// Calculates the middle point of `self` and `other`.
            ///
            /// This returns `NaN` when either argument is `NaN`,
            /// or if a combination of `+inf` and `-inf` is provided as arguments.
            pub const fn midpoint(self, other: $f) -> Float<$f> {
                Self(self.0.midpoint(other))
            }

            /// Raises itself to the `p` integer power.
            pub const fn const_powi(self, p: $ie) -> Float<$f> {
                match p {
                    0 => Self::ONE,
                    1.. => {
                        let mut result = self.0;
                        whilst![_i in 1..p; result *= self.0];
                        Float(result)
                    }
                    _ => {
                        let mut result = self.0;
                        let abs_p = p.abs();
                        whilst![_i in 1..abs_p; result /= self.0];
                        Float(result)
                    }
                }
            }

            /// Evaluates a polynomial at the `self` point value, using [Horner's method].
            ///
            /// Expects a slice of `coefficients` $[a_n, a_{n-1}, ..., a_1, a_0]$
            /// representing the polynomial $ a_n * x^n + a_{n-1} * x^{(n-1)} + ... + a_1 * x + a_0 $.
            ///
            /// # Examples
            /// ```
            /// # use devela::Float;
            /// let coefficients = [2.0, -6.0, 2.0, -1.0];
            #[doc = cc!["assert_eq![Float(3.0_", sfy![$f], ").eval_poly(&coefficients), 5.0];"]]
            #[doc = cc!["assert_eq![Float(3.0_", sfy![$f], ").eval_poly(&[]), 0.0];"]]
            /// ```
            ///
            /// [Horner's method]: https://en.wikipedia.org/wiki/Horner%27s_method#Polynomial_evaluation_and_long_division
            // WAIT: [for-loops in constants](https://github.com/rust-lang/rust/issues/87575)
            pub const fn eval_poly(self, coefficients: &[$f]) -> Float<$f> {
                let coef = coefficients;
                match coef.len() {
                    0 => Float(0.0),
                    1 => Float(coef[0]),
                    _ => {
                        let mut result = coef[0];
                        // for &c in &coef[1..] { result = result * self.0 + c; }
                        whilst![i in 1..coef.len(); result = result * self.0 + coef[i]];
                        Float(result)
                    }
                }
            }

            /// Approximates the derivative of the 1D function `f` at `x` point using step size `h`.
            ///
            /// Uses the [finite difference method].
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_DERIVATIVE!()]
            ///
            // FLAG_DISABLED:nightly_autodiff
            // See also the [`autodiff`] attr macro, enabled with the `nightly_autodiff` cfg flag.
            //
            /// [finite difference method]: https://en.wikipedia.org/wiki/Finite_difference_method
            // [`autodiff`]: crate::autodiff
            pub fn derivative<F>(f: F, x: $f, h: $f) -> Float<$f>
            where
                F: Fn($f) -> $f,
            {
                Float((f(x + h) - f(x)) / h)
            }

            /// Approximates the integral of the 1D function `f` over the range `[x, y]`
            /// using `n` subdivisions.
            ///
            /// Uses the [Riemann Sum](https://en.wikipedia.org/wiki/Riemann_sum).
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_INTEGRATE!()]
            pub fn integrate<F>(f: F, x: $f, y: $f, n: usize) -> Float<$f>
            where
                F: Fn($f) -> $f,
            {
                let dx = (y - x) / n as $f;
                Float(
                    (0..n)
                    .map(|i| { let x = x + i as $f * dx; f(x) * dx })
                    .sum()
                )
            }

            /// Approximates the partial derivative of the 2D function `f` at point (`x`, `y`)
            /// with step size `h`, differentiating over `x`.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_PARTIAL_DERIVATIVE_X!()]
            pub fn partial_derivative_x<F>(f: F, x: $f, y: $f, h: $f) -> Float<$f>
            where
                F: Fn($f, $f) -> $f,
            {
                Float((f(x + h, y) - f(x, y)) / h)
            }

            /// Approximates the partial derivative of the 2D function `f` at point (`x`, `y`)
            /// with step size `h`, differentiating over `x`.
            ///
            /// # Formulation
            #[doc = crate::_FLOAT_FORMULA_PARTIAL_DERIVATIVE_Y!()]
            pub fn partial_derivative_y<F>(f: F, x: $f, y: $f, h: $f) -> Float<$f>
            where
                F: Fn($f, $f) -> $f,
            {
                Float((f(x, y + h) - f(x, y)) / h)
            }
        }
    };
}
impl_float_shared!();
