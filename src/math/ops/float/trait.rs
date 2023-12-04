// devela::math::ops::float::trait

use super::Fp;

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
    /// otherwise it's equal to [`sqrt_nr`][Fp::sqrt_nr].
    #[must_use]
    fn sqrt(self) -> Self;

    /// The square root calculated using the
    /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
    #[must_use]
    fn sqrt_fisr(self) -> Self;

    /// $ 1 / \sqrt{x} $ the
    /// [fast inverse square root algorithm](https://en.wikipedia.org/wiki/Fast_inverse_square_root).
    #[must_use]
    fn fisr(self) -> Self;

    /// The cubic root.
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`cbrt_nr`][Fp::cbrt_nr].
    #[must_use]
    fn cbrt(self) -> Self;

    /// The hypothenuse (the euclidean distance).
    ///
    /// With either `std` or `libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`hypot_nr`][Fp::hypot_nr].
    #[must_use]
    fn hypot(self, rhs: Self) -> Self;

    /// Returns $e^x$ (the exponential function).
    ///
    /// The maximum values with a representable result are:
    /// 88.722… for `f32` and 709.782… for `f64`.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp_taylor`][Fp::exp_taylor]
    /// with [`exp_taylor_terms`][Fp::exp_taylor_terms].
    #[must_use]
    fn exp(self) -> Self;

    /// Returns $2^x$.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp2_taylor`][Fp::exp2_taylor]
    /// with [`exp2_taylor_terms`][Fp::exp2_taylor_terms].
    #[must_use]
    fn exp2(self) -> Self;

    /// The exponential minus 1, more accurately.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp_m1_taylor`][Fp::exp_m1_taylor]
    /// with [`exp_taylor_terms`][Fp::exp_taylor_terms].
    #[must_use]
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
    ///
    /// The maximum values with a representable result are:
    /// 34 for `f32` and 170 for `f64`.
    #[must_use]
    fn factorial(n: u32) -> Self;

    /// The sine.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`sin_taylor`][Fp::sin_taylor] with 8 terms.
    #[must_use]
    fn sin(self) -> Self;

    /// The cosine.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`cos_taylor`][Fp::cos_taylor] with 8 terms.
    #[must_use]
    fn cos(self) -> Self;

    /// Returns both the sine and cosine.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`sin_cos_taylor`][Fp::sin_cos_taylor] with 8 terms.
    #[must_use]
    fn sin_cos(self) -> (Self, Self);

    /// The tangent.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`tan_taylor`][Fp::tan_taylor] with 8 terms.
    #[must_use]
    fn tan(self) -> Self;

    /// The arc sine.
    ///
    /// With both `std` and `libm` disabled it leverages [`asin_taylor`][Fp::asin_taylor]
    /// with [`asin_taylor_terms`][Fp::asin_taylor_terms].
    #[must_use]
    fn asin(self) -> Self;

    /// The arc cosine.
    ///
    /// With both `std` and `libm` disabled it leverages [`acos_taylor`][Fp::acos_taylor]
    /// with [`acos_taylor_terms`][Fp::acos_taylor_terms].
    #[must_use]
    fn acos(self) -> Self;

    /// The arc tangent.
    ///
    /// With both `std` and `libm` disabled it leverages [`atan_taylor`][Fp::atan_taylor]
    /// with [`atan_taylor_terms`][Fp::atan_taylor_terms].
    #[must_use]
    fn atan(self) -> Self;

    /// The arc tangent of two variables.
    ///
    /// With both `std` and `libm` disabled it leverages [`atan2_taylor`][Fp::atan2_taylor]
    /// with [`atan2_taylor_terms`][Fp::atan2_taylor_terms].
    #[must_use]
    fn atan2(self, other: Self) -> Self;

    /// The hyperbolic sine.
    ///
    /// With both `std` and `libm` disabled it leverages [`sinh_taylor`][Fp::sinh_taylor]
    /// with [`exp_taylor_terms`][Fp::exp_taylor_terms].
    #[must_use]
    fn sinh(self) -> Self;

    /// The hyperbolic cosine.
    ///
    /// With both `std` and `libm` disabled it leverages [`cosh_taylor`][Fp::cosh_taylor]
    /// with [`exp_taylor_terms`][Fp::exp_taylor_terms].
    #[must_use]
    fn cosh(self) -> Self;

    /// The hyperbolic tangent.
    ///
    /// With both `std` and `libm` disabled it leverages [`cosh_taylor`][Fp::cosh_taylor]
    /// with [`exp_taylor_terms`][Fp::exp_taylor_terms].
    #[must_use]
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
            // const BIAS: u32 = [<BIAS_ $f:upper>];
            // const EXPONENT_BITS: u32 = [<EXPONENT_BITS_ $f:upper>];
            // const SIGNIFICAND_BITS: u32 = [<SIGNIFICAND_BITS_ $f:upper>];

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

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn sqrt(self) -> Self { Fp::<$f>::sqrt(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sqrt(self) -> Self { Fp::<$f>::sqrt_nr(self) }

            #[inline(always)]
            fn sqrt_fisr(self) -> Self { Fp::<$f>::sqrt_fisr(self) }

            #[inline(always)]
            fn fisr(self) -> Self { Fp::<$f>::fisr(self) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn exp(self) -> Self { Fp::<$f>::exp(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn exp(self) -> Self { Fp::<$f>::exp_taylor(self, Fp::<$f>::exp_taylor_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn exp2(self) -> Self { Fp::<$f>::exp2(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn exp2(self) -> Self { Fp::<$f>::exp2_taylor(self, Fp::<$f>::exp2_taylor_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn exp_m1(self) -> Self { Fp::<$f>::exp_m1(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn exp_m1(self) -> Self {
                Fp::<$f>::exp_m1_taylor(self, Fp::<$f>::exp_taylor_terms(self))
            }

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

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn cbrt(self) -> Self { Fp::<$f>::cbrt(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn cbrt(self) -> Self { Fp::<$f>::cbrt_nr(self) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn hypot(self, rhs: Self) -> Self { Fp::<$f>::hypot(self, rhs) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn hypot(self, rhs: Self) -> Self { Fp::<$f>::hypot_nr(self, rhs) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn sin(self) -> Self { Fp::<$f>::sin(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sin(self) -> Self { Fp::<$f>::sin_taylor(self, 8) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn cos(self) -> Self { Fp::<$f>::cos(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn cos(self) -> Self { Fp::<$f>::cos_taylor(self, 8) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn sin_cos(self) -> (Self, Self) { Fp::<$f>::sin_cos(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sin_cos(self) -> (Self, Self) { Fp::<$f>::sin_cos_taylor(self, 8) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn tan(self) -> Self { Fp::<$f>::tan(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn tan(self) -> Self { Fp::<$f>::tan_taylor(self, 8) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn asin(self) -> Self { Fp::<$f>::asin(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn asin(self) -> Self { Fp::<$f>::asin_taylor(self, Fp::<$f>::asin_taylor_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn acos(self) -> Self { Fp::<$f>::acos(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn acos(self) -> Self { Fp::<$f>::acos_taylor(self, Fp::<$f>::acos_taylor_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn atan(self) -> Self { Fp::<$f>::atan(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn atan(self) -> Self { Fp::<$f>::atan_taylor(self, Fp::<$f>::atan_taylor_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn atan2(self, other: Self) -> Self { Fp::<$f>::atan2(self, other) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn atan2(self, other: Self) -> Self {
                Fp::<$f>::atan2_taylor(self, other, Fp::<$f>::atan_taylor_terms(self))
            }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn sinh(self) -> Self { Fp::<$f>::sinh(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sinh(self) -> Self { Fp::<$f>::sinh_taylor(self, Fp::<$f>::exp_taylor_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn cosh(self) -> Self { Fp::<$f>::cosh(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn cosh(self) -> Self { Fp::<$f>::cosh_taylor(self, Fp::<$f>::exp_taylor_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn tanh(self) -> Self { Fp::<$f>::tanh(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn tanh(self) -> Self { Fp::<$f>::tanh_taylor(self, Fp::<$f>::exp_taylor_terms(self)) }

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
