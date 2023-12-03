// devela::math::ops::float
//
//! Floating point wrapper.
//
// TOC
// - define FloatExt trait
// - implement FloatExt
// - define Fp struct
// - implement Fp methods
//   - when std is enabled
//   - when libm is enabled
//   - whenever
//      - private helpers
//   - when neither std or libm are enabled
// - define Fp constants

#![cfg_attr(not(feature = "math"), allow(unused))]

use crate::meta::iif;

/* private constants */

const BIAS_F32: u32 = 127;
const BIAS_F64: u32 = 1023;
const EXPONENT_BITS_F32: u32 = 8;
const EXPONENT_BITS_F64: u32 = 11;
const SIGNIFICAND_BITS_F32: u32 = 23;
const SIGNIFICAND_BITS_F64: u32 = 52;

// Magic numbers for the fast inverse square root calculation:
// Lomont's single precision magic number
const FISR_MAGIC_F32: u32 = 0x5f37_59df;
// Lomont's double precision magic number
// const FISQRT_MAGIC_NUMBER_F64: u64 = 0x5fe6_ec85_e7de_30da;
// Matthew Robertson's double precision magic number
const FISR_MAGIC_F64: u64 = 0x5fe6_eb50_c7b5_37a9;
// Matthew Robertson's quadruple precision magic number
// const FISR_MAGIC_F128: u128 = 0x5ffe_6eb5_0c7b_537a_9cd9_f02e_504f_cfbf;

// Tolerances for the difference between successive guesses using the
// Newton-Raphson method for square root calculation:
const NR_TOLERANCE_F32: f32 = 1e-7;
const NR_TOLERANCE_F64: f64 = 1e-15;

/// Extension trait for floating-point types.
///
/// This trait is normally more convenient to use than the [`Fp`] struct.
///
/// `Fp` has a few more methods implemented if the `libm` feature is enabled,
/// and some of the methods are const if the `unsafe_math` feature is enabled.
///
/// Many methods are only available if either the `std` or `libm` features are enabled.
#[rustfmt::skip]
pub trait FloatExt: Sized {
    /// Bias value used in the exponent to allow representation of both positive
    /// and negative exponents.
    const BIAS: u32;
    /// Number of bits used to represent the exponent.
    const EXPONENT_BITS: u32;
    /// Number of explicit bits used to represent the significand (or mantissa).
    ///
    /// Note that std's `MANTISSA_DIGITS` floating-point constant equals
    /// `SIGNIFICAND_BITS + 1` since it accounts for an additional implicit leading bit,
    /// which is not stored but assumed in the standard floating-point representation.
    const SIGNIFICAND_BITS: u32;

    /// The largest integer less than or equal to `self`.
    #[must_use]
    fn floor(self) -> Self;
    /// The smallest integer greater than or equal to `self`.
    #[must_use]
    fn ceil(self) -> Self;
    /// Returns the nearest integer to `self`, default rounding, same as
    /// [`round_ties_away`][FloatExt::round_ties_away]
    #[must_use]
    fn round(self) -> Self;
    /// Returns the nearest integer to `self`, rounding ties away from `0.0`.
    #[must_use]
    fn round_ties_away(self) -> Self;
    /// Returns the nearest integer to `self`, rounding ties to the nearest even integer.
    #[must_use]
    fn round_ties_even(self) -> Self;
    /// Returns the nearest integer to `self`, rounding ties to the nearest odd integer.
    #[must_use]
    fn round_ties_odd(self) -> Self;
    /// The integral part.
    #[must_use]
    fn trunc(self) -> Self;
    /// The fractional part.
    #[must_use]
    fn fract(self) -> Self;
    /// Returns the integral and fractional parts.
    #[must_use]
    fn split(self) -> (Self, Self);
    /// The absolute value.
    #[must_use]
    fn abs(self) -> Self;
    /// Returns `true` if self has a positive sign.
    #[must_use]
    fn is_sign_positive(self) -> bool;
    /// Returns `true` if self has a negative sign.
    #[must_use]
    fn is_sign_negative(self) -> bool;
    /// A number that represents the sign of `self`.
    #[must_use]
    fn signum(self) -> Self;
    /// A number composed of a magnitude of `self` and the sign of `sign`.
    #[must_use]
    fn copysign(self, sign: Self) -> Self;
    /// Fused multiply-add. Computes `(self * mul) + add` with only one rounding error.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn mul_add(self, mul: Self, add: Self) -> Self;
    /// The euclidean division.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn div_euclid(self, rhs: Self) -> Self;
    /// The least nonnegative remainder of `self % rhs`.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn rem_euclid(self, rhs: Self) -> Self;
    /// Raises `self` to the `p` floating point power.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn powf(self, p: Self) -> Self;
    /// Raises `self` to the `p` integer power.
    #[must_use]
    fn powi(self, p: i32) -> Self;
    /// The square root.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`sqrt_nr`][Self::sqrt_nr].
    #[must_use]
    fn sqrt(self) -> Self;
    /// The square root calculated using the fast inverse square root algorithm.
    ///
    /// - <https://en.wikipedia.org/wiki/Fast_inverse_square_root>.
    #[must_use]
    fn sqrt_fisr(self) -> Self;
    /// The square root calculated using the Newton-Raphson method.
    ///
    /// - <https://en.wikipedia.org/wiki/Newton%27s_method>.
    #[must_use]
    fn sqrt_nr(self) -> Self;
    /// Returns `e^a` (the exponential function).
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn exp(self) -> Self;
    /// Returns `2^a`.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn exp2(self) -> Self;
    /// The exponential minus 1, more accurately.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn exp_m1(self) -> Self;
    /// The natural logarithm.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn ln(self) -> Self;
    /// The natural logarithm plus 1, more accurately.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn ln_1p(self) -> Self;
    /// The logarithm of the number with respect to an arbitrary `base`.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn log(self, base: Self) -> Self;
    /// The base 2 logarithm.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn log2(self) -> Self;
    /// The base 10 logarithm.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn log10(self) -> Self;
    /// The factorial.
    #[must_use]
    fn factorial(n: u32) -> Self;
    /// The cubic root.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn cbrt(self) -> Self;
    /// The hypothenuse (the euclidean distance).
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn hypot(self, rhs: Self) -> Self;
    /// The sine.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`sin_taylor`][Fp#method.sin_taylor] with 8 terms.
    #[must_use]
    fn sin(self) -> Self;
    /// The cosine.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`cos_taylor`][Fp#method.cos_taylor] with 8 terms.
    #[must_use]
    fn cos(self) -> Self;
    /// The tangent.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`tan_taylor`][Fp#method.tan_taylor] with 8 terms.
    #[must_use]
    fn tan(self) -> Self;
    /// The arc sine.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`asin_taylor`][Fp#method.asin_taylor] with the
    /// number of terms based on the given table.
    #[must_use]
    fn asin(self) -> Self;
    /// The arc cosine.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`acos_taylor`][Fp#method.acos_taylor] with the
    /// number of terms based on the given table.
    #[must_use]
    fn acos(self) -> Self;
    /// The arc tangent.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`atan_taylor`][Fp#method.atan_taylor] with the
    /// number of terms based on the given table.
    #[must_use]
    fn atan(self) -> Self;
    /// The arc tangent of two variables.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`atan2_taylor`][Fp#method.atan2_taylor] with the
    /// number of terms based on the given table.
    #[must_use]
    fn atan2(self, other: Self) -> Self;
    /// Returns both the sine and cosine.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn sin_cos(self) -> (Self, Self);
    /// The hyperbolic sine.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn sinh(self) -> Self;
    /// The hyperbolic cosine.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn cosh(self) -> Self;
    /// The hyperbolic tangent.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn tanh(self) -> Self;
    /// The inverse hyperbolic sine.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn asinh(self) -> Self;
    /// The inverse hyperbolic cosine.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn acosh(self) -> Self;
    /// The inverse hyperbolic tangent.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn atanh(self) -> Self;

    /// Returns the clamped value, ignoring `NaN`.
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;
    /// Returns the maximum of two numbers, ignoring `NaN`.
    #[must_use]
    fn max(self, other: Self) -> Self;
    /// Returns the minimum of two numbers, ignoring `NaN`.
    #[must_use]
    fn min(self, other: Self) -> Self;

    /// Returns the clamped value, propagating `NaN`.
    #[must_use]
    fn clamp_nan(self, min: Self, max: Self) -> Self;
    /// Returns the maximum of two numbers, propagating `NaN`.
    #[must_use]
    fn max_nan(self, other: Self) -> Self;
    /// Returns the minimum of two numbers, propagating `NaN`.
    #[must_use]
    fn min_nan(self, other: Self) -> Self;

    /// Returns the clamped value, using total order.
    #[must_use]
    fn clamp_total(self, min: Self, max: Self) -> Self;
    /// Returns the maximum of two numbers using total order.
    #[must_use]
    fn max_total(self, other: Self) -> Self;
    /// Returns the minimum of two numbers using total order.
    #[must_use]
    fn min_total(self, other: Self) -> Self;
}

macro_rules! impl_float_ext {
    // $f: the floating-point type.
    // $ue: unsigned int type with the same bit-size.
    // $ie: the integer type for integer exponentiation.
    ($( ($f:ty, $ue:ty|$ie:ty) ),+) => { $( impl_float_ext![@$f, $ue|$ie]; )+ };
    (@$f:ty, $ue:ty|$ie:ty) => {
        impl FloatExt for $f { $crate::meta::paste! {
            const BIAS: u32 = [<BIAS_ $f:upper>];
            const EXPONENT_BITS: u32 = [<EXPONENT_BITS_ $f:upper>];
            const SIGNIFICAND_BITS: u32 = [<SIGNIFICAND_BITS_ $f:upper>];

            #[inline(always)]
            fn floor(self) -> Self { Fp::<$f>::floor(self) }
            #[inline(always)]
            fn ceil(self) -> Self { Fp::<$f>::ceil(self) }
            #[inline(always)]
            fn round(self) -> Self { Fp::<$f>::round_ties_away(self) }
            #[inline(always)]
            fn round_ties_away(self) -> Self { Fp::<$f>::round_ties_away(self) }
            #[inline(always)]
            fn round_ties_even(self) -> Self { Fp::<$f>::round_ties_even(self) }
            #[inline(always)]
            fn round_ties_odd(self) -> Self { Fp::<$f>::round_ties_odd(self) }
            #[inline(always)]
            fn trunc(self) -> Self { Fp::<$f>::trunc(self) }
            #[inline(always)]
            fn fract(self) -> Self { Fp::<$f>::fract(self) }
            #[inline(always)]
            fn split(self) -> (Self, Self) { Fp::<$f>::split(self) }
            #[inline(always)]
            fn abs(self) -> Self { Fp::<$f>::abs(self) }
            #[inline(always)]
            fn is_sign_positive(self) -> bool { <$f>::is_sign_positive(self) }
            #[inline(always)]
            fn is_sign_negative(self) -> bool { <$f>::is_sign_negative(self) }
            #[inline(always)]
            fn signum(self) -> Self { Fp::<$f>::signum(self) }
            #[inline(always)]
            fn copysign(self, sign: Self) -> Self { Fp::<$f>::copysign(self, sign) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn mul_add(self, mul: Self, add: Self) -> Self { Fp::<$f>::mul_add(self, mul, add) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn div_euclid(self, rhs: Self) -> Self { Fp::<$f>::div_euclid(self, rhs) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn rem_euclid(self, rhs: Self) -> Self { Fp::<$f>::rem_euclid(self, rhs) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn powf(self, p: Self) -> Self { Fp::<$f>::powf(self, p) }
            #[inline(always)]
            fn powi(self, p: $ie) -> Self { Fp::<$f>::powi(self, p) }
            #[inline(always)]
            fn sqrt(self) -> Self { Fp::<$f>::sqrt(self) }
            #[inline(always)]
            fn sqrt_fisr(self) -> Self { Fp::<$f>::sqrt_fisr(self) }
            #[inline(always)]
            fn sqrt_nr(self) -> Self { Fp::<$f>::sqrt_nr(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn exp(self) -> Self { Fp::<$f>::exp(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn exp2(self) -> Self { Fp::<$f>::exp2(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn exp_m1(self) -> Self { Fp::<$f>::exp_m1(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn ln(self) -> Self { Fp::<$f>::ln(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn ln_1p(self) -> Self { Fp::<$f>::ln_1p(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn log(self, base: Self) -> Self { Fp::<$f>::log(self, base) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn log2(self) -> Self { Fp::<$f>::log2(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn log10(self) -> Self { Fp::<$f>::log10(self) }
            #[inline(always)]
            fn factorial(a: $ue) -> Self { Fp::<$f>::factorial(a) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn cbrt(self) -> Self { Fp::<$f>::cbrt(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn hypot(self, rhs: Self) -> Self { Fp::<$f>::hypot(self, rhs) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn sin(self) -> Self { Fp::<$f>::sin(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))] // alternative
            fn sin(self) -> Self { Fp::<$f>::sin_taylor(self, 8) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn cos(self) -> Self { Fp::<$f>::cos(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))] // alternative
            fn cos(self) -> Self { Fp::<$f>::cos_taylor(self, 8) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn tan(self) -> Self { Fp::<$f>::tan(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))] // alternative
            fn tan(self) -> Self { Fp::<$f>::tan_taylor(self, 8) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn asin(self) -> Self { Fp::<$f>::asin(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))] // alternative
            fn asin(self) -> Self { Fp::<$f>::asin_taylor(self, Fp::<$f>::asin_taylor_terms(self)) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn acos(self) -> Self { Fp::<$f>::acos(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))] // alternative
            fn acos(self) -> Self { Fp::<$f>::acos_taylor(self, Fp::<$f>::acos_taylor_terms(self)) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn atan(self) -> Self { Fp::<$f>::atan(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))] // alternative
            fn atan(self) -> Self { Fp::<$f>::atan_taylor(self, Fp::<$f>::atan_taylor_terms(self)) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn atan2(self, other: Self) -> Self { Fp::<$f>::atan2(self, other) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))] // alternative
            fn atan2(self, other: Self) -> Self {
                Fp::<$f>::atan2_taylor(self, other, Fp::<$f>::atan_taylor_terms(self))
            }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn sin_cos(self) -> (Self, Self) { Fp::<$f>::sin_cos(self) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn sinh(self) -> Self { Fp::<$f>::sinh(self) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn cosh(self) -> Self { Fp::<$f>::cosh(self) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn tanh(self) -> Self { Fp::<$f>::tanh(self) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn asinh(self) -> Self { Fp::<$f>::asinh(self) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn acosh(self) -> Self { Fp::<$f>::acosh(self) }
            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn atanh(self) -> Self { Fp::<$f>::atanh(self) }
            #[inline(always)]
            fn clamp(self, min: Self, max: Self) -> Self { Fp::<$f>::clamp(self, min, max) }
            #[inline(always)]
            fn max(self, other: Self) -> Self { Fp::<$f>::max(self, other) }
            #[inline(always)]
            fn min(self, other: Self) -> Self { Fp::<$f>::min(self, other) }
            #[inline(always)]
            fn clamp_nan(self, min: Self, max: Self) -> Self { Fp::<$f>::clamp_nan(self, min, max) }
            #[inline(always)]
            fn max_nan(self, other: Self) -> Self { Fp::<$f>::max_nan(self, other) }
            #[inline(always)]
            fn min_nan(self, other: Self) -> Self { Fp::<$f>::min_nan(self, other) }
            #[inline(always)]
            fn max_total(self, other: Self) -> Self { Fp::<$f>::max_total(self, other) }
            #[inline(always)]
            fn min_total(self, other: Self) -> Self { Fp::<$f>::min_total(self, other) }
            #[inline(always)]
            fn clamp_total(self, min: Self, max: Self) -> Self {
                Fp::<$f>::clamp_total(self, min, max)
            }
        }}
    }
}
impl_float_ext![(f32, u32 | i32), (f64, u32 | i32)];

/// Floating-point operations wrapper that can leverage `std` or `libm`.
///
/// It favors `std` style for method's names, but changes a few like `minimum`
/// for `min_nan` and `maximum` for `max_nan`.
///
/// If both the `libm` and `std` features are enabled the `libm` functions will
/// be used, since it contains more functions, namely:
/// - Gamma functions: [`gamma`][Fp#method.gamma], [`lgamma`][Fp#method.lgamma],
///   [`lgamma_r`][Fp#method.lgamma_r].
/// - Bessel functions:
///   [`j0`][Fp#method.j0], [`j1`][Fp#method.j1], [`jn`][Fp#method.jn],
///   [`y0`][Fp#method.y0], [`y1`][Fp#method.y1], [`yn`][Fp#method.yn].
/// - Error functions: [`erf`][Fp#method.erf], [`erfc`][Fp#method.erfc].
/// - [`exp10`][Fp#method.exp10].
///
/// See also the [`FloatExt`] trait.
#[derive(Debug, Clone, Copy)]
pub struct Fp<T>(core::marker::PhantomData<T>);

// macro helper for implementing methods for `Fp`, from `libm` or `std`.
//
// $lib: the library to use.
// $f: the floating-point type to support.
// $doc: an optional documentation string.
// $opfn: the original operation function name.
// $op: the new operation function name in Fp.
#[cfg(any(feature = "libm", feature = "std"))]
macro_rules! impl_fp {
    // Matches a wildcard floating-point type (f*).
    // Expands to specific floating-point types (f32, f64).
    ($lib:ident : f* : $($ops:tt)*) => {
        impl_fp![$lib : f32 : $($ops)*];
        impl_fp![$lib : f64 : $($ops)*];
    };
    // Matches a specific floating-point type and any number of operations.
    // Generates the impl block for Fp<$f> and calls the matching implementation.
    ($lib:ident : $f:ty : $($ops:tt)*) => { $crate::meta::paste! {
        #[doc = "# *This implementation block leverages the `" $lib "` feature.*"]
        impl Fp<$f> {
            impl_fp![@$lib : $f : $($ops)*];
        }
    }};
    // Matches multiple operations and uses recursion to process each one.
    (@$lib:ident : $f:ty : $($doc:literal)? $opfn:ident = $op:ident : $($arg:ident),*
     ; $($rest:tt)*) => {
        impl_fp![@$lib : $f : $($doc)? $opfn = $op : $($arg),*];
        impl_fp![@$lib : $f : $($rest)*];
    };
    // Matches a single operation and implements it using the `libm` library.
    (@libm : $f:ty : $($doc:literal)? $opfn:ident = $op:ident : $($arg:ident),*) => {
        $(#[doc = $doc])?
        #[inline(always)]
        pub fn $op($($arg: $f),*) -> $f {
            $crate::_dep::libm::Libm::<$f>::$opfn($($arg),*)
        }
    };
    // Matches a single operation and implements it using the `std` library.
    (@std : $f:ty : $($doc:literal)? $opfn:ident = $op:ident : $($arg:ident),*) => {
        $(#[doc = $doc])?
        #[inline(always)]
        pub fn $op($($arg: $f),*) -> $f {
            <$f>::$opfn($($arg),*)
        }
    };
}
#[cfg(any(feature = "libm", feature = "std"))]
use impl_fp;

#[cfg(all(not(feature = "libm"), feature = "std"))]
mod _std {
    use super::{impl_fp, Fp};
    // custom implementations are commented out:
    impl_fp![std:f*:
       "The largest integer less than or equal to `a`."
        floor = floor: a;
        "The smallest integer greater than or equal to `a`."
        ceil = ceil: a;
        "Returns the nearest integer to `a`, rounding ties away from `0.0`."
        round = round_ties_away: a;
        "The integral part."
        trunc = trunc: a;
        "The fractional part."
        fract = fract: a;
        // split == modf
        "The absolute value."
        abs = abs: a;
        "A number that represents the sign of `a`."
        signum = signum: a;
        "A number composed of a `magnitude` and a `sign`."
        copysign = copysign: magnitude, sign;
        "Fused multiply-add. Computes (a * b) + c with only one rounding error."
        mul_add = mul_add: a, b, c;
        "The euclidean division."
        div_euclid = div_euclid: a, b;
        "The least nonnegative remainder of `a` % `b`."
        rem_euclid = rem_euclid: a, b;
        "Raises `a` to the `p` floating point power."
        powf = powf: a, p;
        // powi
        "The square root."
        sqrt = sqrt: a;
        "Returns `e^a` (the exponential function)."
        exp = exp: a;
        "Returns `2^a`."
        exp2 = exp2: a;
        "The exponential minus 1, more accurately."
        exp_m1 = exp_m1: a;
        "The natural logarithm."
        ln = ln: a;
        "The natural logarithm plus 1, more accurately."
        ln_1p = ln_1p: a;
        "The logarithm of the number with respect to an arbitrary base."
        log = log: a, b;
        "The base 2 logarithm."
        log2 = log2: a;
        "The base 10 logarithm."
        log10 = log10: a;
        "The cubic root."
        cbrt = cbrt: a;
        "The hypothenuse (the euclidean distance)."
        hypot = hypot: a, b;
        "The sine."
        sin = sin: a;
        "The cosine."
        cos = cos: a;
        "The tangent."
        tan = tan: a;
        "The arc sine."
        asin = asin: a;
        "The arc cosine."
        acos = acos: a;
        "The arc tangent."
        atan = atan: a;
        "The arc tangent of two variables."
        atan2 = atan2: a, b;
        // sin_cos
        "The hyperbolic sine."
        sinh = sinh: a;
        "The hyperbolic cosine."
        cosh = cosh: a;
        "The hyperbolic tangent."
        tanh = tanh: a;
        "The inverse hyperbolic sine."
        asinh = asinh: a;
        "The inverse hyperbolic cosine."
        acosh = acosh: a;
        "The inverse hyperbolic tangent."
        atanh = atanh: a;

        "Returns the maximum of two numbers, ignoring `NaN`."
        max = max: a, b;
        "Returns the minimum of two numbers, ignoring `NaN`."
        min = min: a, b

        /* not implemented */
        // exp10: https://internals.rust-lang.org/t/enh-add-exp10-and-expf-base-x-f64-f32-methods-to-stdlib-to-symmetrize-api
        // gamma, ln_gamma: WAIT: https://github.com/rust-lang/rust/issues/99842
        // next_up, next_down: WAIT: https://github.com/rust-lang/rust/issues/91399
    ];

    // $f: the floating-point type.
    // $e: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $e:ty) ),+) => { $( custom_impls![@$f, $e]; )+ };
        (@$f:ty, $e:ty) => {
            /// # *Implementations using the `std` feature*.
            impl Fp<$f> {
                /// Raises `a` to the `p` integer power.
                #[inline(always)]
                pub fn powi(a: $f, p: $e) -> $f { <$f>::powi(a, p) }
                /// Both the sine and cosine.
                #[inline(always)]
                pub fn sin_cos(a: $f) -> ($f, $f) { <$f>::sin_cos(a) }
                /// Returns the integral and fractional parts.
                #[inline(always)]
                pub fn split(value: $f) -> ($f, $f) { (value.trunc(), value.fract()) }
            }
        };
    }
    custom_impls![(f32, i32), (f64, i32)];
}

#[cfg(feature = "libm")]
mod _libm {
    use super::{impl_fp, Fp};
    use crate::{_dep::libm::Libm, meta::iif};
    // custom implementations are commented out
    impl_fp![libm:f*:
        "The largest integer less than or equal to `a`."
        floor = floor: a;
        "The smallest integer greater than or equal to `a`."
        ceil = ceil: a;
        "Returns the nearest integer to `a`, rounding ties away from `0.0`."
        round = round_ties_away: a;
        "The integral part."
        trunc = trunc: a;
        // fract
        // split == modf
        "The absolute value."
        fabs = abs: a;
        // signum
        "Returns a number composed of a `magnitude` and a `sign`."
        copysign = copysign: a, b;
        "Fused multiply-add. Computes (a * b) + c with only one rounding error."
        fma = mul_add: a, b, c;
        // div_euclid
        // rem_euclid
        "Raises `a` to the `p` floating point power."
        pow = powf: a, p;
        // powi
        "Square root."
        sqrt = sqrt: a;
        "Returns `e^a` (the exponential function)."
        exp = exp: a;
        "Returns `2^a`."
        exp2 = exp2: a;
        "The exponential minus 1, more accurately."
        expm1 = exp_m1: a;
        // ln = ln: a;
        "The natural logarithm."
        log = ln: a;
        "The natural logarithm plus 1, more accurately."
        log1p = ln_1p: a;
        // log
        "The base 2 logarithm."
        log2 = log2: a;
        "The base 10 logarithm."
        log10 = log10: a;
        "The cubic root."
        cbrt = cbrt: a;
        "The hypothenuse (the euclidean distance)."
        hypot = hypot: a, b;
        "The sine."
        sin = sin: a;
        "The cosine."
        cos = cos: a;
        "The tangent."
        tan = tan: a;
        "The arc sine."
        asin = asin: a;
        "The arc cosine."
        acos = acos: a;
        "The arc tangent."
        atan = atan: a;
        "The arc tangent of two variables."
        atan2 = atan2: a, b;
        // sin_cos
        "The hyperbolic sine."
        sinh = sinh: a;
        "The hyperbolic cosine."
        cosh = cosh: a;
        "The hyperbolic tangent."
        tanh = tanh: a;
        "The inverse hyperbolic sine."
        asinh = asinh: a;
        "The inverse hyperbolic cosine."
        acosh = acosh: a;
        "The inverse hyperbolic tangent."
        atanh = atanh: a;

        "Returns the minimum of two numbers, ignoring `NaN`."
        fmax = max: a, b;
        "Returns the minimum of two numbers, ignoring `NaN`."
        fmin = min: a, b;

        /* only in libm */

        "Returns `10^a`."
        exp10 = exp10: a;
        "The gamma function. Generalizes the factorial function to complex numbers."
        tgamma = gamma : a;
        "The natural logarithm of the absolute value of the gamma function."
        lgamma = lgamma : a;
        "The error function."
        erf = erf: a;
        "The complementary error function (1 - erf)."
        erfc = erfc: a;
        "The bessel function of the first kind, of order 0."
        j0 = j0: a;
        "The bessel function of the first kind, of order 1."
        j1 = j1: a;
        // jn
        "The bessel function of the second kind, of order 0."
        y0 = y0: a;
        "The bessel function of the second kind, of order 1."
        y1 = y1: a
        // yn
    ];
    // $f: the floating-point type.
    // $e: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $e:ty) ),+) => { $( custom_impls![@$f, $e]; )+ };
        (@$f:ty, $e:ty) => {
            /// # *Implementations using the `libm` feature*.
            impl Fp<$f> {
                /// Returns the fractional part.
                #[must_use]
                #[inline(always)]
                pub fn fract(a: $f) -> $f { a - Libm::<$f>::trunc(a) }
                /// The integral and fractional parts.
                #[must_use]
                #[inline(always)]
                pub fn split(value: $f) -> ($f, $f) { Libm::<$f>::modf(value) }
                /// A number that represents the sign of `a`, propagating `NaN`.
                #[must_use]
                #[inline(always)]
                pub fn signum(a: $f) -> $f {
                    iif![a.is_nan(); <$f>::NAN; Libm::<$f>::copysign(1.0, a)]
                }
                /// The euclidean division.
                #[must_use]
                #[inline(always)]
                pub fn div_euclid(a: $f, b: $f) -> $f {
                    let q = Self::trunc(a / b);
                    iif![a % b < 0.0; return iif![b > 0.0; q - 1.0; q + 1.0]]; q
                }
                /// The least nonnegative remainder of `a` % `b`.
                #[must_use]
                #[inline(always)]
                pub fn rem_euclid(a: $f, b: $f) -> $f {
                    let r = a % b; iif![r < 0.0; r + Self::abs(b); r]
                }
                /// Raises `a` to the `p` integer power.
                #[must_use]
                #[inline(always)]
                pub fn powi(a: $f, p: $e) -> $f { Self::powf(a, p as $f) }
                /// The logarithm of the number with respect to an arbitrary base.
                #[must_use]
                #[inline(always)]
                pub fn log(value: $f, base: $f) -> $f { Self::ln(base) / Self::ln(value) }
                /// The sine and cosine.
                #[must_use]
                #[inline(always)]
                pub fn sin_cos(a: $f) -> ($f, $f) { Libm::<$f>::sincos(a) }

                // NOTE: implemented manually in _either
                //
                // /// Returns the clamped a value, propagating `NaN`.
                // #[must_use]
                // #[inline(always)]
                // pub fn clamp_nan(value: $f, min: $f, max: $f) -> $f {
                //     Self::min_nan(Self::max_nan(value, min), max)
                // }
                // /// Returns the maximum of two numbers, propagating `NaN`.
                // #[must_use]
                // #[inline(always)]
                // pub fn max_nan(a: $f, b: $f) -> $f {
                //     iif![a.is_nan() || b.is_nan(); <$f>::NAN; Libm::<$f>::fmax(a, b)]
                // }
                // /// Returns the minimum of two numbers, propagating `NaN`.
                // #[must_use]
                // #[inline(always)]
                // pub fn min_nan(a: $f, b: $f) -> $f {
                //     iif![a.is_nan() || b.is_nan(); <$f>::NAN; Libm::<$f>::fmin(a, b)]
                // }

                /* only in libm */

                /// The natural logarithm of the absolute value of the gamma function,
                /// plus its sign.
                #[must_use]
                #[inline(always)]
                pub fn lgamma_r(a: $f) -> ($f, $e) { Libm::<$f>::lgamma_r(a) }
                /// Bessel function of the first kind, of order `n`.
                #[must_use]
                #[inline(always)]
                pub fn jn(n: $e, a: $f) -> $f { Libm::<$f>::jn(n, a) }
                /// Bessel function of the second kind, of order `n`.
                #[must_use]
                #[inline(always)]
                pub fn yn(n: $e, a: $f) -> $f { Libm::<$f>::yn(n, a) }
            }
        };
    }
    custom_impls![(f32, i32), (f64, i32)];
}

mod _whenever {
    #![allow(missing_docs)]

    use super::*;

    // $f: the floating-point type.
    // $ue: unsigned int type with the same bit-size.
    // $ie: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $ue:ty, $ie:ty) ),+) => { $( custom_impls![@$f, $ue, $ie]; )+ };
        (@$f:ty, $ue:ty, $ie:ty) => { $crate::meta::paste! {
            /// # *Common implementations with or without `std` or `libm`*.
            ///
            /// Total order const fns will only be `const` if the `unsafe_math` feature is enabled.
            impl Fp<$f> {
                /// Returns the nearest integer to `a`, rounding ties to the nearest even integer.
                // WAIT: https://github.com/rust-lang/rust/issues/96710
                #[must_use]
                #[inline]
                pub fn round_ties_even(a: $f) -> $f {
                    let r = Self::round_ties_away(a);
                    iif![r % 2.0 == 0.0; r ;
                        iif![Self::abs(a - r) == 0.5; r - Self::signum(a); r]]
                }

                /// Returns the nearest integer to `a`, rounding ties to the nearest odd integer.
                #[must_use]
                #[inline]
                pub fn round_ties_odd(a: $f) -> $f {
                    let r = Self::round_ties_away(a);
                    iif![r % 2.0 != 0.0; r ;
                        iif![Self::abs(a - r) == 0.5; r + Self::signum(a); r]]
                }

                /// Returns `true` if `a` is positive.
                #[must_use]
                #[inline]
                pub fn is_sign_positive(a: $f) -> bool { <$f>::is_sign_positive(a) }

                /// Returns `true` if `a` is negative.
                #[must_use]
                #[inline]
                pub fn is_sign_negative(a: $f) -> bool { <$f>::is_sign_negative(a) }

                /// The square root calculated using the fast inverse square root algorithm.
                ///
                /// - <https://en.wikipedia.org/wiki/Fast_inverse_square_root>.
                #[must_use]
                #[inline]
                pub fn sqrt_fisr(a: $f) -> $f {
                    let mut i = a.to_bits();
                    let three_halfs: $f = 1.5;
                    let x2 = a * 0.5;
                    let mut y: $f;

                    i = [< FISR_MAGIC_ $f:upper >] - (i >> 1);
                    y = <$f>::from_bits(i);
                    y = y * (three_halfs - (x2 * y * y));

                    1.0 / y
                }

                /// The square root calculated using the Newton-Raphson method.
                ///
                /// - <https://en.wikipedia.org/wiki/Newton%27s_method>.
                #[must_use]
                #[inline]
                pub fn sqrt_nr(a: $f) -> $f {
                    const TOLERANCE: $f = [<NR_TOLERANCE_ $f:upper>];
                    let mut x = a;
                    let mut x_next = 0.5 * (x + a / x);
                    while Self::abs(x - x_next) > TOLERANCE {
                        x = x_next;
                        x_next = 0.5 * (x + a / x);
                    }
                    x_next
                }

                /// The factorial of the integer value `a`.
                ///
                /// The maximum values with a representable result are:
                /// 34 for `f32` and 170 for `f64`.
                ///
                /// Note that precision is poor for large values.
                #[must_use]
                #[inline]
                pub fn factorial(a: $ue) -> $f {
                    let mut result = 1.0;
                    for i in 1..=a {
                        result *= i as $f;
                    }
                    result
                }

                /// The sine calculated using Taylor series expansion.
                ///
                /// $$
                /// \sin(a) = a - \frac{a^3}{3!} + \frac{a^5}{5!} - \frac{a^7}{7!} + \cdots
                /// $$
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
                /// ± 0.990 →      6     10
                /// ± 0.999 →      6     10
                /// ```
                #[must_use]
                #[inline]
                pub fn sin_taylor(a: $f, terms: $ue) -> $f {
                    let a = Self::clamp(a, -Self::PI, Self::PI);
                    let (mut sin_approx, mut num, mut den) = (0.0, a, 1.0);
                    for i in 0..terms {
                        if i > 0 {
                            num *= -a * a;
                            den *= ((2 * i + 1) * (2 * i)) as $f;
                        }
                        sin_approx += num / den;
                    }
                    sin_approx
                }

                /// Computes the cosine using taylor series expansion.
                ///
                /// $$
                /// \cos(a) = 1 - \frac{a^2}{2!} + \frac{a^4}{4!} - \frac{a^6}{6!} + \cdots
                /// $$
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
                /// ± 0.990 →      7     11
                /// ± 0.999 →      7     11
                /// ```
                #[must_use]
                #[inline]
                pub fn cos_taylor(a: $f, terms: $ue) -> $f {
                    let a = Self::clamp(a, -Self::PI, Self::PI);
                    let (mut cos_approx, mut num, mut den) = (0.0, 1.0, 1.0);
                    for i in 0..terms {
                        if i > 0 {
                            num *= -a * a;
                            den *= ((2 * i) * (2 * i - 1)) as $f;
                        }
                        cos_approx += num / den;
                    }
                    cos_approx
                }

                /// Computes the tangent using Taylor series expansion of sine and cosine.
                ///
                /// The tangent function has singularities and is not defined for
                /// `cos(a) = 0`. This function clamps `a` within an appropriate range
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
                /// ± 0.990 →      7     11
                /// ± 0.999 →      7     11
                /// ```
                #[must_use]
                #[inline]
                pub fn tan_taylor(a: $f, terms: $ue) -> $f {
                    let a = Self::clamp(a, -Self::PI / 2.0 + 0.0001, Self::PI / 2.0 - 0.0001);
                    let sin_approx = Self::sin_taylor(a, terms);
                    let cos_approx = Self::cos_taylor(a, terms);
                    iif![cos_approx.abs() < 0.0001; return $f::MAX];
                    sin_approx / cos_approx
                }

                /// Computes the arcsine using Taylor series expansion.
                ///
                /// $$
                /// \arcsin(a) = a + \left( \frac{1}{2} \right) \frac{a^3}{3} +
                /// \left( \frac{1}{2} \cdot \frac{3}{4} \right) \frac{a^5}{5} +
                /// \left( \frac{1}{2} \cdot \frac{3}{4} \cdot \frac{5}{6} \right) \frac{a^7}{7} +
                /// \cdots
                /// $$
                ///
                /// asin is undefined for $ |a| > 1 $ and in that case returns `NaN`.
                ///
                /// The series converges more slowly near the edges of the domain
                /// (i.e., as `a` approaches -1 or 1). For more accurate results,
                /// especially near these boundary values, a higher number of terms
                /// may be necessary.
                ///
                /// See also [`asin_taylor_terms`][Self::asin_taylor_terms].
                #[must_use]
                #[inline]
                pub fn asin_taylor(a: $f, terms: $ue) -> $f {
                    iif![Self::abs(a) > 1.0; return $f::NAN];
                    let (mut asin_approx, mut multiplier, mut power_x) = (0.0, 1.0, a);
                    for i in 0..terms {
                        if i != 0 {
                            multiplier *= (2 * i - 1) as $f / (2 * i) as $f;
                            power_x *= a * a;
                        }
                        asin_approx += multiplier * power_x / (2 * i + 1) as $f;
                    }
                    asin_approx
                }

                /// Determines the number of terms needed for [`asin_taylor`][Self::asin_taylor]
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
                #[inline(always)]
                pub fn asin_taylor_terms(a: $f) -> $ue { Self::[<asin_acos_taylor_terms_ $f>](a) }

                /// Computes the arccosine using the Taylor expansion of arcsine.
                ///
                /// $$
                /// arccos(a)=2π-arcsin(a)
                /// $$
                ///
                /// See the [`asin_taylor`][Self#method.asin_taylor] table for
                /// information about the number of `terms` needed.
                #[must_use]
                #[inline]
                pub fn acos_taylor(a: $f, terms: $ue) -> $f {
                    iif![a.abs() > 1.0; return $f::NAN];
                    Self::FRAC_PI_2 - Self::asin_taylor(a, terms)
                }
                /// Determines the number of terms needed for [`acos_taylor`][Self::acos_taylor]
                /// to reach a stable result based on the input value.
                ///
                /// The table is the same as [`asin_taylor_terms`][Self::asin_taylor_terms].
                #[must_use]
                #[inline(always)]
                pub fn acos_taylor_terms(a: $f) -> $ue { Self::[<asin_acos_taylor_terms_ $f>](a) }

                /// Computes the arctangent using Taylor series expansion.
                ///
                /// $$
                /// \arctan(a) = a - \frac{a^3}{3} + \frac{a^5}{5} - \frac{a^7}{7} + \cdots
                /// $$
                ///
                /// For values $ |a| > 1 $ it uses the identity:
                /// $$
                /// \arctan(a) = \frac{\pi}{2} - \arctan(\frac{1}{x})
                /// $$
                ///
                /// The series converges more slowly near the edges of the domain
                /// (i.e., as `a` approaches -1 or 1). For more accurate results,
                /// especially near these boundary values, a higher number of terms
                /// may be necessary.
                ///
                /// See also [`atan_taylor_terms`][Self::atan_taylor_terms].
                #[must_use]
                #[inline]
                pub fn atan_taylor(a: $f, terms: $ue) -> $f {
                    if Self::abs(a) > 1.0 {
                        if a > 0.0 {
                            Self::FRAC_PI_2 - Self::atan_taylor(1.0 / a, terms)
                        } else {
                            -Self::FRAC_PI_2 - Self::atan_taylor(1.0 / a, terms)
                        }
                    } else {
                        let (mut atan_approx, mut num, mut sign) = (0.0, a, 1.0);
                        for i in 0..terms {
                            if i > 0 {
                                num *= a * a;
                                sign = -sign;
                            }
                            atan_approx += sign * num / (2 * i + 1) as $f;
                        }
                        atan_approx
                    }
                }
                /// Determines the number of terms needed for [`atan_taylor`][Self::atan_taylor]
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
                #[inline(always)]
                pub fn atan_taylor_terms(a: $f) -> $ue { Self::[<atan_taylor_terms_ $f>](a) }

                /// Computes the four quadrant arctangent of `a` and `b` using Taylor series expansion.
                ///
                /// See also [`atan_taylor_terms`][Self::atan_taylor_terms].
                #[must_use]
                #[inline]
                pub fn atan2_taylor(a: $f, b: $f, terms: $ue) -> $f {
                    if b > 0.0 {
                        Self::atan_taylor(a / b, terms)
                    } else if a >= 0.0 && b < 0.0 {
                        Self::atan_taylor(a / b, terms) + Self::PI
                    } else if a < 0.0 && b < 0.0 {
                        Self::atan_taylor(a / b, terms) - Self::PI
                    } else if a > 0.0 && b == 0.0 {
                        Self::PI / 2.0
                    } else if a < 0.0 && b == 0.0 {
                        -Self::PI / 2.0
                    } else {
                        // a and b are both zero, undefined behavior
                        $f::NAN
                    }
                }

                /// Returns the clamped value, ignoring `NaN`.
                #[must_use]
                #[inline(always)]
                pub fn clamp(value: $f, min: $f, max: $f) -> $f {
                    Self::min(Self::max(value, min), max)
                }

                /// Returns the clamped value, using total order.
                #[must_use]
                #[inline(always)]
                #[cfg(feature = "unsafe_math")]
                pub const fn clamp_total(value: $f, min: $f, max: $f) -> $f {
                    $crate::data::cmp::[<clamp_ $f>](value, min, max)
                }
                #[must_use]
                #[inline(always)]
                #[cfg(not(feature = "unsafe_math"))]
                pub fn clamp_total(value: $f, min: $f, max: $f) -> $f {
                    $crate::data::cmp::[<clamp_ $f>](value, min, max)
                }

                /// Returns the maximum of two numbers using total order.
                #[must_use]
                #[inline(always)]
                #[cfg(feature = "unsafe_math")]
                pub const fn max_total(a: $f, b: $f) -> $f { $crate::data::cmp::[<max_ $f>](a, b) }
                #[must_use]
                #[inline(always)]
                #[cfg(not(feature = "unsafe_math"))]
                pub fn max_total(a: $f, b: $f) -> $f { $crate::data::cmp::[<max_ $f>](a, b) }

                /// Returns the minimum of two numbers using total order.
                #[must_use]
                #[inline(always)]
                #[cfg(feature = "unsafe_math")]
                pub const fn min_total(a: $f, b: $f) -> $f { $crate::data::cmp::[<min_ $f>](a, b) }
                #[must_use]
                #[inline(always)]
                #[cfg(not(feature = "unsafe_math"))]
                pub fn min_total(a: $f, b: $f) -> $f { $crate::data::cmp::[<min_ $f>](a, b) }

                /// Returns the clamped a value, propagating `NaN`.
                #[must_use]
                #[inline(always)]
                pub fn clamp_nan(value: $f, min: $f, max: $f) -> $f {
                    Self::min_nan(Self::max_nan(value, min), max)
                }
                /// Returns the maximum of two numbers, propagating `NaN`.
                // WAIT: https://github.com/rust-lang/rust/issues/91079
                #[must_use]
                #[inline(always)]
                pub fn max_nan(a: $f, b: $f) -> $f {
                    if a > b {
                        a
                    } else if b > a {
                        b
                    } else if a == b {
                        if a.is_sign_positive() && b.is_sign_negative() { a } else { b }
                    } else {
                        a + b
                    }
                }
                /// Returns the minimum of two numbers, propagating `NaN`.
                #[must_use]
                #[inline(always)]
                // WAIT: https://github.com/rust-lang/rust/issues/91079
                pub fn min_nan(a: $f, b: $f) -> $f {
                    if a < b {
                        a
                    } else if b < a {
                        b
                    } else if a == b {
                        if a.is_sign_negative() && b.is_sign_positive() { a } else { b }
                    } else {
                        // At least one input is NaN. Use `+` to perform NaN propagation and quieting.
                        a + b
                    }
                }
            }
        }};
    }
    custom_impls![(f32, u32, i32), (f64, u32, i32)];

    /* private helpers */

    #[rustfmt::skip]
    impl Fp<f32> {
        // Determines the number of terms needed for a Taylor series approximation
        // of asin & acos to reach a stable result based on the f32 input value.
        #[must_use]
        #[inline]
        pub(super) fn asin_acos_taylor_terms_f32(a: f32) -> u32 {
            let abs_a = Self::abs(a);
            if abs_a <= 0.1 { 5
            } else if abs_a <= 0.3 { 7
            } else if abs_a <= 0.5 { 10
            } else if abs_a <= 0.7 { 18
            } else if abs_a <= 0.9 { 47
            } else if abs_a <= 0.99 { 333
            } else { 1989 // computed for 0.999
            }
        }
        // Determines the number of terms needed for a Taylor series approximation
        // of atan to reach a stable result based on the f32 input value.
        #[must_use]
        #[inline]
        pub(super) fn atan_taylor_terms_f32(a: f32) -> u32 {
            let abs_a = Self::abs(a);
            if abs_a <= 0.1 { 5
            } else if abs_a <= 0.3 { 7
            } else if abs_a <= 0.5 { 12
            } else if abs_a <= 0.7 { 20
            } else if abs_a <= 0.9 { 61
            } else if abs_a <= 0.99 { 518
            } else { 4151 // computed for 0.999
            }
        }
    }
    #[rustfmt::skip]
    impl Fp<f64> {
        #[must_use]
        #[inline]
        pub(super) fn asin_acos_taylor_terms_f64(a: f64) -> u32 {
            let abs_a = Self::abs(a);
            if abs_a <= 0.1 { 9
            } else if abs_a <= 0.3 { 15
            } else if abs_a <= 0.5 { 24
            } else if abs_a <= 0.7 { 44
            } else if abs_a <= 0.9 { 134
            } else if abs_a <= 0.99 { 1235
            } else { 10768 // computed for 0.999
            }
        }
        // Determines the number of terms needed for a Taylor series approximation
        // of atan to reach a stable result based on the f64 input value.
        #[must_use]
        #[inline]
        pub(super) fn atan_taylor_terms_f64(a: f64) -> u32 {
            let abs_a = Self::abs(a);
            if abs_a <= 0.1 { 9
            } else if abs_a <= 0.3 { 15
            } else if abs_a <= 0.5 { 26
            } else if abs_a <= 0.7 { 47
            } else if abs_a <= 0.9 { 152
            } else if abs_a <= 0.99 { 1466
            } else { 13604 // computed for 0.999
            }
        }
    }
}

#[cfg(all(not(feature = "libm"), not(feature = "std")))]
mod _no_std_no_libm {
    use super::*;
    use crate::meta::iif;

    // $f: the floating-point type.
    // $ub: unsigned int type with the same bit-size.
    // $ie: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $ub:ty, $ie:ty) ),+) => { $( custom_impls![@$f, $ub, $ie]; )+ };
        (@$f:ty, $ub:ty, $ie:ty) => { $crate::meta::paste! {
            /// # *Implementations without `std` or `libm`*.
            impl Fp<$f> {
                /// The largest integer less than or equal to `a`.
                #[must_use]
                #[inline]
                pub fn floor(a: $f) -> $f {
                    let mut result = Self::trunc(a);
                    if a.is_sign_negative() && Self::abs(a - result) > <$f>::EPSILON {
                        result -= 1.0;
                    }
                    result
                }

                /// The smallest integer greater than or equal to `a`.
                #[must_use]
                #[inline]
                pub fn ceil(a: $f) -> $f {
                    let mut result = Self::trunc(a);
                    if a.is_sign_positive() && Self::abs(a - result) > <$f>::EPSILON {
                        result += 1.0;
                    }
                    result
                }

                /// Returns the nearest integer to `self`, default rounding
                ///
                /// This is the default [`round_ties_away`] implementation.
                #[must_use]
                #[inline]
                pub fn round(a: $f) -> $f {
                    Self::trunc(a + Self::copysign(0.5 - 0.25 * <$f>::EPSILON, a))
                }

                /// Returns the nearest integer to `self`, rounding ties away from `0.0`.
                ///
                /// This is the default [`round`] implementation.
                #[must_use]
                #[inline]
                pub fn round_ties_away(a: $f) -> $f {
                    Self::trunc(a + Self::copysign(0.5 - 0.25 * <$f>::EPSILON, a))
                }

                /// The integral part.
                /// This means that non-integer numbers are always truncated towards zero.
                ///
                /// This implementation uses bitwise manipulation to remove the fractional part
                /// of the floating-point number. The exponent is extracted, and a mask is
                /// created to remove the fractional part. The new bits are then used to create
                /// the truncated floating-point number.
                #[must_use]
                #[inline]
                pub fn trunc(a: $f) -> $f {
                    let bits = a.to_bits();
                    const BIAS: $ie = [<BIAS_ $f:upper>] as $ie;
                    const SIG_BITS: $ie = [<SIGNIFICAND_BITS_ $f:upper>] as $ie;
                    const EXP_MASK: $ub = 1 << [<EXPONENT_BITS_ $f:upper>] as $ub - 1;

                    #[allow(clippy::cast_possible_wrap)]
                    let exponent = ((bits >> SIG_BITS) & EXP_MASK) as $ie - BIAS;
                    if exponent < 0 {
                        iif![a.is_sign_positive(); 0.0; -0.0]
                    } else if exponent < SIG_BITS {
                        let mask = !(([<1_ $ub>] << (SIG_BITS - exponent)) - 1);
                        let new_bits = bits & mask;
                        <$f>::from_bits(new_bits)
                    } else {
                        a
                    }
                }

                /// The fractional part.
                #[must_use]
                #[inline(always)]
                pub fn fract(a: $f) -> $f { a - Self::trunc(a) }

                /// Returns the integral and fractional parts.
                #[must_use]
                #[inline(always)]
                pub fn split(a: $f) -> ($f, $f) { (Self::trunc(a), Self::fract(a)) }

                /// The absolute value.
                #[must_use]
                #[inline]
                pub fn abs(a: $f) -> $f {
                    let mask = <$ub>::MAX / 2;
                    let bits: $ub = a.to_bits() & mask;
                    <$f>::from_bits(bits)
                }

                /// A number that represents the sign of `a`, propagating `NaN`.
                #[must_use]
                #[inline(always)]
                pub fn signum(a: $f) -> $f {
                    iif![a.is_nan(); <$f>::NAN; Self::copysign(1.0, a)]
                }

                /// A number composed of a magnitude of `a` and the sign of `sign`.
                #[must_use]
                #[inline(always)]
                pub fn copysign(a: $f, sign: $f) -> $f {
                    const SIGN_MASK: $ub = <$ub>::MAX / 2 + 1;
                    const VALUE_MASK: $ub = <$ub>::MAX / 2;
                    let sign_bit = sign.to_bits() & SIGN_MASK;
                    let value_bits = a.to_bits() & VALUE_MASK;
                    <$f>::from_bits(value_bits | sign_bit)
                }

                /// Returns the maximum of two numbers, ignoring `NaN`.
                #[must_use]
                #[inline]
                pub fn max(a: $f, b: $f) -> $f {
                    (if a.is_nan() || a < b { b } else { a }) * 1.0
                }

                /// Returns the minimum of two numbers, ignoring `NaN`.
                #[must_use]
                #[inline]
                pub fn min(a: $f, b: $f) -> $f {
                    (iif![b.is_nan() || a < b; a; b]) * 1.0
                }

                /// Raises `a` to the `p` integer power.
                #[must_use]
                #[inline]
                pub fn powi(a: $f, p: $ie) -> $f {
                    match p {
                        0 => 1.0,
                        1.. => {
                            let mut result = a;
                            for _i in 1..p {
                                result *= a;
                            }
                            result
                        }
                        _ => {
                            let mut result = a;
                            for _i in 1..p.abs() {
                                result /= a;
                            }
                            result
                        }
                    }
                }

                /// The square root.
                ///
                /// It uses `Fp::`[`sqrt_nr`][Self::sqrt_nr].
                #[must_use]
                #[inline(always)]
                pub fn sqrt(a: $f) -> $f {Self::sqrt_nr(a) }

            }
        }};
    }
    custom_impls![(f32, u32, i32), (f64, u64, i32)];
}

// https://en.wikipedia.org/wiki/List_of_mathematical_constants
mod _consts {
    use super::Fp;

    // $f: the floating-point type.
    macro_rules! custom_impls {
        ($( $f:ty),+) => { $( custom_impls![@$f]; )+ };
        (@$f:ty) => { $crate::meta::paste! {
            /// # *Implementations of mathematical constants.
            #[allow(clippy::excessive_precision)] // 36 decimal points
            impl Fp<$f> {
                /* π related */

                /// $ π $ the ratio of the circumference to the diameter
                /// ([A000796](https://oeis.org/A000796))
                pub const PI: $f = 3.14159265358979323846264338327950288;

                /// $ π/2 $
                /// ([A019669](https://oeis.org/A019669))
                pub const FRAC_PI_2: $f = 1.57079632679489661923132169163975144;

                /// $ π/3 $
                /// ([A019670](https://oeis.org/A019670))
                pub const FRAC_PI_3: $f = 1.04719755119659774615421446109316763;

                /// $ π/4 $
                /// ([A003881](https://oeis.org/A003881))
                pub const FRAC_PI_4: $f = 0.785398163397448309615660845819875721;

                /// $ π/6 $
                /// ([A019673](https://oeis.org/A019673))
                pub const FRAC_PI_6: $f = 0.52359877559829887307710723054658381;

                /// $ π/8 $
                /// ([A019675](https://oeis.org/A019675))
                pub const FRAC_PI_8: $f = 0.39269908169872415480783042290993786;

                /// $ \sqrt{π} $
                /// ([A002161](https://oeis.org/A002161))
                pub const SQRT_PI: $f = 1.77245385090551602729816748334114518;

                /// $ 1/π $
                /// ([A049541](https://oeis.org/A049541))
                pub const FRAC_1_PI: $f = 0.318309886183790671537767526745028724;

                /// $ 1/\sqrt{π} $
                /// ([A087197](https://oeis.org/A087197))
                // WAIT: https://github.com/rust-lang/rust/issues/103883
                pub const FRAC_1_SQRT_PI: $f = 0.564189583547756286948079451560772586;

                /// $ 2/π $
                /// ([A060294](https://oeis.org/A060294))
                pub const FRAC_2_PI: $f = 0.636619772367581343075535053490057448;

                /// $ 2/\sqrt{π} $
                /// ([A190732](https://oeis.org/A190732))
                pub const FRAC_2_SQRT_PI: $f = 1.12837916709551257389615890312154517;

                /* τ related */

                /// $ τ = 2π = 360º $ the ratio of the circumference to the radius
                /// ([A019692](https://oeis.org/A019692))
                pub const TAU: $f = 6.28318530717958647692528676655900577;

                /// $ τ/2 = π = 180º $
                /// ([A000796](https://oeis.org/A000796))
                pub const FRAC_TAU_2: $f = Self::PI;

                /// $ τ/3  = 2π/3 = 120º $
                /// ([A019693](https://oeis.org/A019693))
                pub const FRAC_TAU_3: $f = 2.09439510239319549230842892218633526;

                /// $ τ/4 = π/2 = 90º $
                /// ([A019693](https://oeis.org/A019693))
                pub const FRAC_TAU_4: $f = Self::FRAC_PI_2;

                /// $ τ/5 = 2π/5 = 72º $
                /// ([A019694](https://oeis.org/A019694))
                pub const FRAC_TAU_5: $f = 1.25663706143591729538505735331180115;

                /// $ τ/6 = π/3 = 60º $
                /// ([A019670](https://oeis.org/A019670))
                pub const FRAC_TAU_6: $f = Self::FRAC_PI_3;

                /// $ τ/8 = π/4 = 45º $
                /// ([A003881](https://oeis.org/A003881))
                pub const FRAC_TAU_8: $f = Self::FRAC_PI_4;

                /// $ τ/9 = 2π/9 = 40º $
                /// ([A019696](https://oeis.org/A019696))
                pub const FRAC_TAU_9: $f = 0.69813170079773183076947630739544508;

                /// $ τ/12 = π/6 = 30º $
                /// ([A019673](https://oeis.org/A019673))
                pub const FRAC_TAU_12: $f = Self::FRAC_PI_6;

                /// $ τ/16 = π/8 = 22.5º $
                /// ([A019675](https://oeis.org/A019675))
                pub const FRAC_TAU_16: $f = Self::FRAC_PI_8;

                /// $ τ/24 = π/12 = 15º $
                /// ([A019679](https://oeis.org/A019679))
                pub const FRAC_TAU_24: $f = 0.26179938779914943653855361527329191;

                /// $ τ/72 = π/36 = 5º $
                pub const FRAC_TAU_72: $f = 0.08726646259971647884618453842443063;

                /// $ τ/360 = π/180 = 1º $ *arcdegree*
                /// ([A019685](https://oeis.org/A019685),
                /// [wikipedia](https://en.wikipedia.org/wiki/Degree_(angle)))
                pub const FRAC_TAU_360: $f = 0.01745329251994329576923690768488613;

                /// $ 360/τ = 180/π $
                /// ([A072097](https://oeis.org/A072097))
                pub const FRAC_360_TAU: $f = 57.29577951308232087679815481410517033;

                /// $ τ/(360*60) = 1' $ *arcminute*
                /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
                pub const FRAC_TAU_ARCMINUTES: $f = 0.00029088820866572159615394846141477;

                /// $ τ/(360 * 60 * 60) = 1'' $ *arcsecond*
                /// ([wikipedia](https://en.wikipedia.org/wiki/Minute_and_second_of_arc))
                pub const FRAC_TAU_ARCSECONDS: $f = 0.00000484813681109535993589914102358;

                /// $ \sqrt{τ} $
                /// ([A019727](https://oeis.org/A019727))
                pub const SQRT_TAU: $f = 2.50662827463100050241576528481104525;

                /// $ 1/τ = 1/2π $
                /// ([A086201](https://oeis.org/A086201))
                pub const FRAC_1_TAU: $f = 0.159154943091895335768883763372514362;

                /// $ 1/\sqrt{τ} = 1/\sqrt{2π} $
                /// ([A231863](https://oeis.org/A231863))
                pub const FRAC_1_SQRT_TAU: $f = 0.398942280401432677939946059934381868;

                /// $ 2/τ = 1/π $
                /// ([A049541](https://oeis.org/A049541))
                pub const FRAC_2_TAU: $f = Self::FRAC_1_PI;

                /// $ 2/\sqrt{τ} = \sqrt{2/π} $
                /// ([A231863](https://oeis.org/A231863))
                pub const FRAC_2_SQRT_TAU: $f = 0.797884560802865355879892119868763737;

                /* φ related */

                /// $ φ $ the golden ratio $\frac{1+\sqrt{5}}{2}$
                /// ([A001622](https://oeis.org/A001622))
                // WAIT: https://github.com/rust-lang/rust/issues/103883
                pub const PHI: $f = 1.618033988749894848204586834365638118;

                /// sqrt{φ} $
                /// ([A139339](https://oeis.org/A139339))
                pub const SQRT_PHI: $f = 1.27201964951406896425242246173749149;

                /// The tribonacci constant.
                /// ([A058265](https://oeis.org/A058265))
                pub const TRIBONACCI: $f = 1.83928675521416113255185256465328660;

                /* integer sqrts */

                /// $ \sqrt{2} $
                /// ([A002193](https://oeis.org/A002193),
                /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2))
                pub const SQRT_2: $f = 1.41421356237309504880168872420969808;

                /// $ 1/\sqrt{2} = \sqrt{1/2} $
                /// ([A010503](https://oeis.org/A010503),
                /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_2#Multiplicative_inverse))
                pub const FRAC_1_SQRT_2: $f = 0.707106781186547524400844362104849039;

                /// $ \sqrt{3} $
                /// ([A002194](https://oeis.org/A002194),
                /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_3))
                // WAIT: https://github.com/rust-lang/rust/issues/103883
                pub const SQRT_3: $f = 1.732050807568877293527446341505872367;

                /// $ 1/\sqrt{3} = \sqrt{1/3} $
                // WAIT: https://github.com/rust-lang/rust/issues/103883
                pub const FRAC_1_SQRT_3: $f = 0.577350269189625764509148780501957456;

                /// $ \sqrt{5} $
                /// ([A002163](https://oeis.org/A002163),
                /// [wikipedia](https://en.wikipedia.org/wiki/Square_root_of_5))
                pub const SQRT_5: $f = 2.236067977499789696409173668731276235;

                /// $ \sqrt{6} $
                /// ([A010464](https://oeis.org/A010464))
                pub const SQRT_6: $f = 2.449489742783178098197284074705891392;

                /// $ \sqrt{7} $
                /// ([A010465](https://oeis.org/A010465))
                pub const SQRT_7: $f = 2.645751311064590590501615753639260426;

                /// $ \sqrt{8} $
                /// ([A010466](https://oeis.org/A010466))
                pub const SQRT_8: $f = 2.828427124746190097603377448419396157;

                /// $ \sqrt{10} $
                /// ([A010467](https://oeis.org/A010467))
                pub const SQRT_10: $f = 3.162277660168379331998893544432718534;

                /// $ \sqrt{11} $
                /// ([A010468](https://oeis.org/A010468))
                pub const SQRT_11: $f = 3.316624790355399849114932736670686684;

                /// $ \sqrt{12} $
                /// ([A010469](https://oeis.org/A010469))
                pub const SQRT_12: $f = 3.464101615137754587054892683011744734;

                /* integer cbrts */

                /// $ \sqrt[\small 3]{2} $
                /// ([A002580](https://oeis.org/A002580),
                /// [wikipedia](https://en.wikipedia.org/wiki/Doubling_the_cube))
                pub const CBRT_2: $f = 1.259921049894873164767210607278228350;

                /// $ \sqrt[\small 3]{3} $
                /// ([A002581](https://oeis.org/A002581))
                pub const CBRT_3: $f = 1.442249570307408382321638310780109588;

                /// $ 1/\sqrt[\small 3]{3} = (\normalsize\frac{1}{3})^{\small\frac{1}{3}} $
                /// ([A072365](https://oeis.org/A072365))
                pub const FRAC_1_CBRT_3: $f = 0.693361274350634704843352274785961795;

                /* other */

                /// $ e $ Euler's number
                pub const E: $f = 2.71828182845904523536028747135266250;

                /// $ γ $ The Euler-Mascheroni constant
                /// ([A001620](https://oeis.org/A001620))
                // WAIT: https://github.com/rust-lang/rust/issues/103883
                pub const EGAMMA: $f = 0.577215664901532860606512090082402431;

                /// log<sub>2</sub>(e)
                pub const LOG2_E: $f = 1.44269504088896340735992468100189214;

                /// log<sub>2</sub>(10)
                pub const LOG2_10: $f = 3.32192809488736234787031942948939018;

                /// log<sub>10</sub>(e)
                pub const LOG10_E: $f = 0.434294481903251827651128918916605082;

                /// log<sub>10</sub>(2)
                pub const LOG10_2: $f = 0.301029995663981195213738894724493027;

                /// ln(2)
                pub const LN_2: $f = 0.693147180559945309417232121458176568;

                /// ln(10)
                pub const LN_10: $f = 2.30258509299404568401799145468436421;
            }
        }};
    }
    custom_impls![f32, f64];
}
