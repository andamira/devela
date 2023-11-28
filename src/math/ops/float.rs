// devela::math::ops::float
//
//! Floating point wrapper.
//
// TOC
// - FloatExt trait
// - Fp wrapper

#![cfg_attr(not(feature = "math"), allow(unused))]

/* floating-point constants */

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
    /// Returns the nearest integer to `self`, rounding ties away from `0.0`.
    #[must_use]
    fn round_ties_away(self) -> Self;
    /// Returns the nearest integer to `self`, rounding ties to the nearest even integer.
    #[must_use]
    fn round_ties_even(self) -> Self;
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
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
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
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn sin(self) -> Self;
    /// The cosine.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn cos(self) -> Self;
    /// The tangent.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn tan(self) -> Self;
    /// The arc sine.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn asin(self) -> Self;
    /// The arc cosine.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn acos(self) -> Self;
    /// The arc tangent.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
    fn atan(self) -> Self;
    /// The arc tangent of two variables.
    #[must_use]
    #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
    #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
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
    ($($t:ty),+) => { $( impl_float_ext![@$t]; )+ };
    (@$t:ty) => {
        impl FloatExt for $t { $crate::meta::paste! {
            const BIAS: u32 = [<BIAS_ $t:upper>];
            const EXPONENT_BITS: u32 = [<EXPONENT_BITS_ $t:upper>];
            const SIGNIFICAND_BITS: u32 = [<SIGNIFICAND_BITS_ $t:upper>];

            #[inline(always)]
            fn floor(self) -> Self { Fp::<$t>::floor(self) }
            #[inline(always)]
            fn ceil(self) -> Self { Fp::<$t>::ceil(self) }
            #[inline(always)]
            fn round_ties_away(self) -> Self { Fp::<$t>::round_ties_away(self) }
            #[inline(always)]
            fn round_ties_even(self) -> Self { Fp::<$t>::round_ties_even(self) }
            #[inline(always)]
            fn trunc(self) -> Self { Fp::<$t>::trunc(self) }
            #[inline(always)]
            fn fract(self) -> Self { Fp::<$t>::fract(self) }
            #[inline(always)]
            fn split(self) -> (Self, Self) { Fp::<$t>::split(self) }
            #[inline(always)]
            fn abs(self) -> Self { Fp::<$t>::abs(self) }
            #[inline(always)]
            fn is_sign_positive(self) -> bool { <$t>::is_sign_positive(self) }
            #[inline(always)]
            fn is_sign_negative(self) -> bool { <$t>::is_sign_negative(self) }
            #[inline(always)]
            fn signum(self) -> Self { Fp::<$t>::signum(self) }
            #[inline(always)]
            fn copysign(self, sign: Self) -> Self { Fp::<$t>::copysign(self, sign) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn mul_add(self, mul: Self, add: Self) -> Self { Fp::<$t>::mul_add(self, mul, add) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn div_euclid(self, rhs: Self) -> Self { Fp::<$t>::div_euclid(self, rhs) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn rem_euclid(self, rhs: Self) -> Self { Fp::<$t>::rem_euclid(self, rhs) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn powf(self, p: Self) -> Self { Fp::<$t>::powf(self, p) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn powi(self, p: i32) -> Self { Fp::<$t>::powi(self, p) }
            #[inline(always)]
            fn sqrt(self) -> Self { Fp::<$t>::sqrt(self) }
            #[inline(always)]
            fn sqrt_fisr(self) -> Self { Fp::<$t>::sqrt_fisr(self) }
            #[inline(always)]
            fn sqrt_nr(self) -> Self { Fp::<$t>::sqrt_nr(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn exp(self) -> Self { Fp::<$t>::exp(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn exp2(self) -> Self { Fp::<$t>::exp2(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn exp_m1(self) -> Self { Fp::<$t>::exp_m1(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn ln(self) -> Self { Fp::<$t>::ln(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn ln_1p(self) -> Self { Fp::<$t>::ln_1p(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn log(self, base: Self) -> Self { Fp::<$t>::log(self, base) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn log2(self) -> Self { Fp::<$t>::log2(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn log10(self) -> Self { Fp::<$t>::log10(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn cbrt(self) -> Self { Fp::<$t>::cbrt(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn hypot(self, rhs: Self) -> Self { Fp::<$t>::hypot(self, rhs) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn sin(self) -> Self { Fp::<$t>::sin(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn cos(self) -> Self { Fp::<$t>::cos(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn tan(self) -> Self { Fp::<$t>::tan(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn asin(self) -> Self { Fp::<$t>::asin(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn acos(self) -> Self { Fp::<$t>::acos(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn atan(self) -> Self { Fp::<$t>::atan(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn atan2(self, other: Self) -> Self { Fp::<$t>::atan2(self, other) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn sin_cos(self) -> (Self, Self) { Fp::<$t>::sin_cos(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn sinh(self) -> Self { Fp::<$t>::sinh(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn cosh(self) -> Self { Fp::<$t>::cosh(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn tanh(self) -> Self { Fp::<$t>::tanh(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn asinh(self) -> Self { Fp::<$t>::asinh(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn acosh(self) -> Self { Fp::<$t>::acosh(self) }
            #[inline(always)]
            #[cfg(any(feature = "std", feature = "libm"))] // IMPROVE
            #[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
            fn atanh(self) -> Self { Fp::<$t>::atanh(self) }
            #[inline(always)]
            fn clamp(self, min: Self, max: Self) -> Self { Fp::<$t>::clamp(self, min, max) }
            #[inline(always)]
            fn max(self, other: Self) -> Self { Fp::<$t>::max(self, other) }
            #[inline(always)]
            fn min(self, other: Self) -> Self { Fp::<$t>::min(self, other) }
            #[inline(always)]
            fn clamp_nan(self, min: Self, max: Self) -> Self { Fp::<$t>::clamp_nan(self, min, max) }
            #[inline(always)]
            fn max_nan(self, other: Self) -> Self { Fp::<$t>::max_nan(self, other) }
            #[inline(always)]
            fn min_nan(self, other: Self) -> Self { Fp::<$t>::min_nan(self, other) }
            #[inline(always)]
            fn max_total(self, other: Self) -> Self { Fp::<$t>::max_total(self, other) }
            #[inline(always)]
            fn min_total(self, other: Self) -> Self { Fp::<$t>::min_total(self, other) }
            #[inline(always)]
            fn clamp_total(self, min: Self, max: Self) -> Self {
                Fp::<$t>::clamp_total(self, min, max)
            }
        }}
    }
}
impl_float_ext![f32, f64];

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
                /// A number that represents the sign of `a`.
                #[must_use]
                #[inline(always)]
                pub fn signum(a: $f) -> $f { a - Libm::<$f>::copysign(1.0 as $f, a) }
                /// The euclidean division.
                #[must_use]
                #[inline(always)]
                pub fn div_euclid(a: $f, b: $f) -> $f {
                    let q = Fp::<$f>::trunc(a / b);
                    iif![a % b < 0.0; return iif![b > 0.0; q - 1.0; q + 1.0]]; q
                }
                /// The least nonnegative remainder of `a` % `b`.
                #[must_use]
                #[inline(always)]
                pub fn rem_euclid(a: $f, b: $f) -> $f {
                    let r = a % b; iif![r < 0.0; r + Fp::<$f>::abs(b); r]
                }
                /// Raises `a` to the `p` integer power.
                #[must_use]
                #[inline(always)]
                pub fn powi(a: $f, p: $e) -> $f { Fp::<$f>::powf(a, p as $f) }
                /// The logarithm of the number with respect to an arbitrary base.
                #[must_use]
                #[inline(always)]
                pub fn log(value: $f, base: $f) -> $f { Fp::<$f>::ln(base) / Fp::<$f>::ln(value) }
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
                //     Fp::<$f>::min_nan(Fp::<$f>::max_nan(value, min), max)
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

mod _either {
    #![allow(missing_docs)]

    use super::*;

    // $f: the floating-point type.
    // $e: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $e:ty) ),+) => { $( custom_impls![@$f, $e]; )+ };
        (@$f:ty, $e:ty) => { $crate::meta::paste! {
            /// # *Common implementations with or without `std` or `libm`*.
            ///
            /// Total order const fns will only be `const` if the `unsafe_math` feature is enabled.
            impl Fp<$f> {
                /// Returns the nearest integer to `a`, rounding ties to the nearest even integer.
                // WAIT: https://github.com/rust-lang/rust/issues/96710
                #[must_use]
                #[inline]
                pub fn round_ties_even(a: $f) -> $f {
                    let rounded = Fp::<$f>::round_ties_away(a);
                    if rounded % 2.0 == 0.0 || Fp::<$f>::abs(rounded - a) > 0.5 {
                        rounded
                    } else {
                        rounded - Fp::<$f>::signum(a)
                    }
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

                    while Fp::<$f>::abs(x - x_next) > TOLERANCE {
                        x = x_next;
                        x_next = 0.5 * (x + a / x);
                    }

                    x_next
                }

                /// Returns the clamped value, ignoring `NaN`.
                #[must_use]
                #[inline(always)]
                pub fn clamp(value: $f, min: $f, max: $f) -> $f {
                    Fp::<$f>::min(Fp::<$f>::max(value, min), max)
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
                    Fp::<$f>::min_nan(Fp::<$f>::max_nan(value, min), max)
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
    custom_impls![(f32, i32), (f64, i32)];
}

#[cfg(all(not(feature = "libm"), not(feature = "std")))]
mod _no_std_no_libm {
    use super::*;
    use crate::meta::iif;

    // $f: the floating-point type.
    // $u: unsigned int type with the same bit-size.
    // $e: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $u:ty, $e:ty) ),+) => { $( custom_impls![@$f, $u, $e]; )+ };
        (@$f:ty, $u:ty, $e:ty) => { $crate::meta::paste! {
            /// # *Implementations without `std` or `libm`*.
            impl Fp<$f> {
                /// The largest integer less than or equal to `a`.
                #[must_use]
                #[inline]
                pub fn floor(a: $f) -> $f {
                    let mut result = Fp::<$f>::trunc(a);
                    if a.is_sign_negative() && Fp::<$f>::abs(a - result) > <$f>::EPSILON {
                        result -= 1.0;
                    }
                    result
                }

                /// The smallest integer greater than or equal to `a`.
                #[must_use]
                #[inline]
                pub fn ceil(a: $f) -> $f {
                    let mut result = Fp::<$f>::trunc(a);
                    if a.is_sign_positive() && Fp::<$f>::abs(a - result) > <$f>::EPSILON {
                        result += 1.0;
                    }
                    result
                }

                /// Returns the nearest integer to `self`, rounding ties away from `0.0`.
                #[must_use]
                #[inline]
                pub fn round_ties_away(a: $f) -> $f {
                    Fp::<$f>::trunc(a + Fp::<$f>::copysign(0.5 - 0.25 * <$f>::EPSILON, a))
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
                    const BIAS: $e = [<BIAS_ $f:upper>] as $e;
                    const SIG_BITS: $e = [<SIGNIFICAND_BITS_ $f:upper>] as $e;
                    const EXP_MASK: $u = 1 << [<EXPONENT_BITS_ $f:upper>] as $u - 1;

                    #[allow(clippy::cast_possible_wrap)]
                    let exponent = ((bits >> SIG_BITS) & EXP_MASK) as $e - BIAS;
                    if exponent < 0 {
                        iif![a.is_sign_positive(); 0.0; -0.0]
                    } else if exponent < SIG_BITS {
                        let mask = !(([<1_ $u>] << (SIG_BITS - exponent)) - 1);
                        let new_bits = bits & mask;
                        <$f>::from_bits(new_bits)
                    } else {
                        a
                    }
                }

                /// The fractional part.
                #[must_use]
                #[inline(always)]
                pub fn fract(a: $f) -> $f { a - Fp::<$f>::trunc(a) }

                /// Returns the integral and fractional parts.
                #[must_use]
                #[inline(always)]
                pub fn split(a: $f) -> ($f, $f) { (Fp::<$f>::trunc(a), Fp::<$f>::fract(a)) }

                /// The absolute value.
                #[must_use]
                #[inline]
                pub fn abs(a: $f) -> $f {
                    let mask = <$u>::MAX / 2;
                    let bits: $u = a.to_bits() & mask;
                    <$f>::from_bits(bits)
                }

                /// A number that represents the sign of `a`.
                #[must_use]
                #[inline(always)]
                pub fn signum(a: $f) -> $f { a - Fp::<$f>::copysign(1.0, a) }

                /// A number composed of a magnitude of `a` and the sign of `sign`.
                #[must_use]
                #[inline(always)]
                pub fn copysign(a: $f, sign: $f) -> $f {
                    const SIGN_MASK: $u = <$u>::MAX / 2 + 1;
                    const VALUE_MASK: $u = <$u>::MAX / 2;
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
                pub fn powi(a: $f, p: $e) -> $f {
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
                pub fn sqrt(a: $f) -> $f {Fp::<$f>::sqrt_nr(a) }

            }
        }};
    }
    custom_impls![(f32, u32, i32), (f64, u64, i32)];
}
