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
    ///
    /// $$ \lfloor x \rfloor = \max \{ n \in \mathbb{Z} \,|\, n \leq x \} $$
    #[must_use]
    fn floor(self) -> Self;

    /// The smallest integer greater than or equal to `self`.
    ///
    /// $$ \lceil x \rceil = \min \{ n \in \mathbb{Z} \,|\, n \geq x \} $$
    #[must_use]
    fn ceil(self) -> Self;

    /// The nearest integer to `self`, default rounding, same as
    /// [`round_ties_away`][FloatExt::round_ties_away]
    #[must_use]
    fn round(self) -> Self;

    /// The nearest integer to `self`, rounding ties away from `0.0`.
    ///
    /// $$
    /// \text{round\\_ties\\_away}(x) = \begin{cases}
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \text{ or }
    ///     (x - \lfloor x \rfloor = 0.5 \text{ and } x > 0) \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \text{ or }
    ///     (x - \lfloor x \rfloor = 0.5 \text{ and } x < 0)
    /// \end{cases}
    /// $$
    #[must_use]
    fn round_ties_away(self) -> Self;

    /// The nearest integer to `self`, rounding ties to the nearest even integer.
    ///
    /// $$
    /// \text{round\\_ties\\_even}(x) = \begin{cases}
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and } \lfloor x \rfloor \text{ is even} \cr
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and } \lfloor x \rfloor \text{ is odd}
    /// \end{cases}
    /// $$
    #[must_use]
    fn round_ties_even(self) -> Self;

    /// The nearest integer to `self`, rounding ties to the nearest odd integer.
    ///
    /// $$
    /// \text{round\\_ties\\_odd}(x) = \begin{cases}
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \cr
    /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and } \lfloor x \rfloor \text{ is odd} \cr
    /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor = 0.5 \text{ and } \lfloor x \rfloor \text{ is even}
    /// \end{cases}
    /// $$
    #[must_use]
    fn round_ties_odd(self) -> Self;

    /// The integral part of `self`.
    ///
    /// $$
    /// \text{trunc}(x) = \begin{cases}
    /// \lfloor x \rfloor, & \text{if } x \geq 0 \\
    /// \lceil x \rceil, & \text{if } x < 0
    /// \end{cases}
    /// $$
    #[must_use]
    fn trunc(self) -> Self;

    /// The fractional part of `self`.
    ///
    /// $$ \text{fract}(x) = x - \text{trunc}(x) $$
    #[must_use]
    fn fract(self) -> Self;

    /// The integral and fractional parts ox `self`.
    ///
    /// $$ \text{split}(x) = (\text{trunc}(x), \text{fract}(x)) $$
    #[must_use]
    fn split(self) -> (Self, Self);

    /// The absolute value.
    #[must_use]
    fn abs(self) -> Self;

    /// Returns `true` if `self` has a positive sign.
    #[must_use]
    fn is_sign_positive(self) -> bool;

    /// Returns `true` if `self` has a negative sign.
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

    /// Raises `self` to the `y` floating point power.
    #[must_use]
    fn powf(self, y: Self) -> Self;

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

    /// $e^x$ (the exponential function).
    ///
    /// The maximum values with a representable result are:
    /// 88.722… for `f32` and 709.782… for `f64`.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp_series`][Fp::exp_series]
    /// with [`exp_series_terms`][Fp::exp_series_terms].
    #[must_use]
    fn exp(self) -> Self;

    /// $2^x$.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp2_series`][Fp::exp2_series]
    /// with [`exp2_series_terms`][Fp::exp2_series_terms].
    #[must_use]
    fn exp2(self) -> Self;

    /// The exponential minus 1, more accurately.
    ///
    /// With both `std` and `libm` disabled it leverages [`exp_m1_series`][Fp::exp_m1_series]
    /// with [`exp_series_terms`][Fp::exp_series_terms].
    #[must_use]
    fn exp_m1(self) -> Self;

    /// The natural logarithm of `self`.
    #[must_use]
    fn ln(self) -> Self;

    /// The natural logarithm of `self` plus 1, more accurately.
    #[must_use]
    fn ln_1p(self) -> Self;

    /// The logarithm of `self` with respect to an arbitrary `base`.
    #[must_use]
    fn log(self, base: Self) -> Self;

    /// The base 2 logarithm of `self`.
    #[must_use]
    fn log2(self) -> Self;

    /// The base 10 logarithm of `self`.
    #[must_use]
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
    /// [`sin_series`][Fp::sin_series] with 8 terms.
    #[must_use]
    fn sin(self) -> Self;

    /// The cosine.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`cos_series`][Fp::cos_series] with 8 terms.
    #[must_use]
    fn cos(self) -> Self;

    /// Both the sine and cosine.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`sin_cos_series`][Fp::sin_cos_series] with 8 terms.
    #[must_use]
    fn sin_cos(self) -> (Self, Self);

    /// The tangent.
    ///
    /// With both `std` and `libm` disabled it leverages
    /// [`tan_series`][Fp::tan_series] with 8 terms.
    #[must_use]
    fn tan(self) -> Self;

    /// The arc sine.
    ///
    /// With both `std` and `libm` disabled it leverages [`asin_series`][Fp::asin_series]
    /// with [`asin_series_terms`][Fp::asin_series_terms].
    #[must_use]
    fn asin(self) -> Self;

    /// The arc cosine.
    ///
    /// With both `std` and `libm` disabled it leverages [`acos_series`][Fp::acos_series]
    /// with [`acos_series_terms`][Fp::acos_series_terms].
    #[must_use]
    fn acos(self) -> Self;

    /// The arc tangent.
    ///
    /// With both `std` and `libm` disabled it leverages [`atan_series`][Fp::atan_series]
    /// with [`atan_series_terms`][Fp::atan_series_terms].
    #[must_use]
    fn atan(self) -> Self;

    /// The arc tangent of two variables.
    ///
    /// With both `std` and `libm` disabled it leverages [`atan2_series`][Fp::atan2_series]
    /// with [`atan_series_terms`][Fp::atan_series_terms].
    #[must_use]
    fn atan2(self, other: Self) -> Self;

    /// The hyperbolic sine.
    ///
    /// With both `std` and `libm` disabled it leverages [`sinh_series`][Fp::sinh_series]
    /// with [`exp_series_terms`][Fp::exp_series_terms].
    #[must_use]
    fn sinh(self) -> Self;

    /// The hyperbolic cosine.
    ///
    /// With both `std` and `libm` disabled it leverages [`cosh_series`][Fp::cosh_series]
    /// with [`exp_series_terms`][Fp::exp_series_terms].
    #[must_use]
    fn cosh(self) -> Self;

    /// The hyperbolic tangent.
    ///
    /// With both `std` and `libm` disabled it leverages [`cosh_series`][Fp::cosh_series]
    /// with [`exp_series_terms`][Fp::exp_series_terms].
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

    /// The clamped value, ignoring `NaN`.
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// The maximum of two numbers, ignoring `NaN`.
    #[must_use]
    fn max(self, other: Self) -> Self;

    /// The minimum of two numbers, ignoring `NaN`.
    #[must_use]
    fn min(self, other: Self) -> Self;

    /// The clamped value, propagating `NaN`.
    #[must_use]
    fn clamp_nan(self, min: Self, max: Self) -> Self;

    /// The maximum of two numbers, propagating `NaN`.
    #[must_use]
    fn max_nan(self, other: Self) -> Self;

    /// The minimum of two numbers, propagating `NaN`.
    #[must_use]
    fn min_nan(self, other: Self) -> Self;

    /// The clamped value, using total order.
    #[must_use]
    fn clamp_total(self, min: Self, max: Self) -> Self;

    /// The maximum of two numbers using total order.
    #[must_use]
    fn max_total(self, other: Self) -> Self;

    /// The minimum of two numbers using total order.
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

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn powf(self, y: Self) -> Self { Fp::<$f>::powf(self, y) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn powf(self, y: Self) -> Self {
                Fp::<$f>::powf_series(self, y, Fp::<$f>::ln_series_terms(self))
            }

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
            fn exp(self) -> Self { Fp::<$f>::exp_series(self, Fp::<$f>::exp_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn exp2(self) -> Self { Fp::<$f>::exp2(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn exp2(self) -> Self { Fp::<$f>::exp2_series(self, Fp::<$f>::exp2_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn exp_m1(self) -> Self { Fp::<$f>::exp_m1(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn exp_m1(self) -> Self {
                Fp::<$f>::exp_m1_series(self, Fp::<$f>::exp_series_terms(self))
            }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn ln(self) -> Self { Fp::<$f>::ln(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn ln(self) -> Self { Fp::<$f>::ln_series(self, Fp::<$f>::ln_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn ln_1p(self) -> Self { Fp::<$f>::ln_1p(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn ln_1p(self) -> Self { Fp::<$f>::ln_1p_series(self, Fp::<$f>::ln_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn log(self, base: Self) -> Self { Fp::<$f>::log(self, base) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn log(self, base: Self) -> Self {
                Fp::<$f>::log_series(self, base, Fp::<$f>::ln_series_terms(self))
            }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn log2(self) -> Self { Fp::<$f>::log2(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn log2(self) -> Self { Fp::<$f>::log2_series(self, Fp::<$f>::ln_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn log10(self) -> Self { Fp::<$f>::log10(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn log10(self) -> Self { Fp::<$f>::log10_series(self, Fp::<$f>::ln_series_terms(self)) }

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
            fn sin(self) -> Self { Fp::<$f>::sin_series(self, 8) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn cos(self) -> Self { Fp::<$f>::cos(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn cos(self) -> Self { Fp::<$f>::cos_series(self, 8) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn sin_cos(self) -> (Self, Self) { Fp::<$f>::sin_cos(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sin_cos(self) -> (Self, Self) { Fp::<$f>::sin_cos_series(self, 8) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn tan(self) -> Self { Fp::<$f>::tan(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn tan(self) -> Self { Fp::<$f>::tan_series(self, 8) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn asin(self) -> Self { Fp::<$f>::asin(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn asin(self) -> Self { Fp::<$f>::asin_series(self, Fp::<$f>::asin_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn acos(self) -> Self { Fp::<$f>::acos(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn acos(self) -> Self { Fp::<$f>::acos_series(self, Fp::<$f>::acos_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn atan(self) -> Self { Fp::<$f>::atan(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn atan(self) -> Self { Fp::<$f>::atan_series(self, Fp::<$f>::atan_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn atan2(self, other: Self) -> Self { Fp::<$f>::atan2(self, other) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn atan2(self, other: Self) -> Self {
                Fp::<$f>::atan2_series(self, other, Fp::<$f>::atan_series_terms(self))
            }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn sinh(self) -> Self { Fp::<$f>::sinh(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn sinh(self) -> Self { Fp::<$f>::sinh_series(self, Fp::<$f>::exp_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn cosh(self) -> Self { Fp::<$f>::cosh(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn cosh(self) -> Self { Fp::<$f>::cosh_series(self, Fp::<$f>::exp_series_terms(self)) }

            #[inline(always)] #[cfg(any(feature = "std", feature = "libm"))]
            fn tanh(self) -> Self { Fp::<$f>::tanh(self) }
            #[inline(always)] #[cfg(not(any(feature = "std", feature = "libm")))]
            fn tanh(self) -> Self { Fp::<$f>::tanh_series(self, Fp::<$f>::exp_series_terms(self)) }

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
