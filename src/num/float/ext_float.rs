// devela::num::float::ext_float
//
//! Extention trait for floatin-point methods.
//
// IMPROVE: remove redundant methods implemented in `core`

use super::shared_docs::*;
#[cfg(_float··)]
use crate::Float;
use crate::{ExtFloatConst, Sign};

#[doc = crate::TAG_NUM!()]
#[doc = crate::TAG_NAMESPACE!()]
/// Extension trait for floating-point types. Associated methods.
///
/// This trait can be more convenient to use than the [`Float`] struct,
/// for non-const operations over primitive floating-point types.
///
/// # Features
/// It depends on having any `_float_f[32|64]` features enabled.
///
/// `Float` has a few more methods implemented if the `dep_libm` feature is enabled.
#[rustfmt::skip]
pub trait ExtFloat: ExtFloatConst + Sized {

    /// The largest integer less than or equal to `self`.
    ///
    /// # Formula
    #[doc = FORMULA_FLOOR!()]
    #[must_use]
    fn floor(self) -> Self;

    /// The smallest integer greater than or equal to `self`.
    ///
    /// # Formula
    #[doc = FORMULA_CEIL!()]
    #[must_use]
    fn ceil(self) -> Self;

    /// The nearest integer to `self`, default rounding, same as
    /// [`round_ties_away`][ExtFloat::round_ties_away]
    #[must_use]
    fn round(self) -> Self;

    /// The nearest integer to `self`, rounding ties away from `0.0`.
    ///
    /// # Formula
    #[doc = FORMULA_ROUND_TIES_AWAY!()]
    #[must_use]
    fn round_ties_away(self) -> Self;

    /// The nearest integer to `self`, rounding ties to the nearest even integer.
    ///
    /// # Formula
    #[doc = FORMULA_ROUND_TIES_EVEN!()]
    #[must_use]
    fn round_ties_even(self) -> Self;

    /// The nearest integer to `self`, rounding ties to the nearest odd integer.
    ///
    /// # Formula
    #[doc = FORMULA_ROUND_TIES_ODD!()]
    #[must_use]
    fn round_ties_odd(self) -> Self;

    /// The integral part of `self`.
    ///
    /// # Formula
    #[doc = FORMULA_TRUNC!()]
    #[must_use]
    fn trunc(self) -> Self;

    /// The fractional part of `self`.
    ///
    /// # Formula
    #[doc = FORMULA_FRACT!()]
    #[must_use]
    fn fract(self) -> Self;

    /// The integral and fractional parts ox `self`.
    ///
    /// # Formula
    #[doc = FORMULA_SPLIT!()]
    #[must_use]
    fn split(self) -> (Self, Self);

    /// The absolute value of `self`.
    #[must_use]
    fn abs(self) -> Self;

    /// The negative absolute value of `self`.
    #[must_use]
    fn neg_abs(self) -> Self;

    /// Returns the `Sign` of `self`.
    fn sign(self) -> Sign;

    /// Returns the `Sign` of `self`, except for zero.
    fn sign_nonzero(self) -> Sign;

    /// A number that represents the sign of `self`.
    #[must_use]
    fn signum(self) -> Self;

    /// Flips the sign of `self`.
    #[must_use]
    fn flip_sign(self) -> Self;

    /// Returns `true` if `self` has a positive sign.
    #[must_use]
    fn is_sign_positive(self) -> bool;

    /// Returns `true` if `self` has a negative sign.
    #[must_use]
    fn is_sign_negative(self) -> bool;

    /// Returns `true` if `self` is either 0.0 or -0.0.
    #[must_use]
    fn is_zero(self) -> bool;

    /// Returns `true` if `self` has a positive sign and is not zero.
    #[must_use]
    fn is_sign_positive_nonzero(self) -> bool;

    /// Returns `true` if `self` has a negative sign and is not zero.
    #[must_use]
    fn is_sign_negative_nonzero(self) -> bool;

    /// A number composed of a magnitude of `self` and the sign of `sign`.
    #[must_use]
    fn copysign(self, sign: Self) -> Self;

    /// Fused multiply-add. Computes `(self * mul) + add` with only one rounding error.
    ///
    /// With either `std` or `dep_libm` enabled it leverages compiler intrinsics,
    /// otherwise it uses [`mul_add_fallback`][Float::mul_add_fallback].
    #[must_use]
    fn mul_add(self, mul: Self, add: Self) -> Self;

    /// The euclidean division.
    #[must_use]
    fn div_euclid(self, rhs: Self) -> Self;

    /// The least nonnegative remainder of `self % rhs`.
    #[must_use]
    fn rem_euclid(self, rhs: Self) -> Self;

    /// Returns `self` between `[min..=max]` scaled to a new range `[u..=v]`.
    ///
    /// Values of `self` outside `[min..=max]` are not clamped and will result in extrapolation.
    /// # Formula
    #[doc = FORMULA_SCALE!()]
    #[must_use]
    fn scale(self, min: Self, max: Self, u: Self, v: Self) -> Self;

    /// Calculates a linearly interpolated value between `u..=v`
    /// based on the percentage `self` between `[0..=1]`.
    ///
    /// Values of `self` outside `[0..=1]` are not clamped and will result in extrapolation.
    /// # Formula
    #[doc = FORMULA_LERP!()]
    #[must_use]
    fn lerp(self, u: Self, v: Self) -> Self;

    /// Raises `self` to the `y` floating point power.
    ///
    /// With either `std` or `dep_libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`powf_series`][Float::powf_series].
    #[must_use]
    fn powf(self, y: Self) -> Self;

    /// Raises `self` to the `p` integer power.
    #[must_use]
    fn powi(self, p: i32) -> Self;

    /// The square root.
    ///
    /// With either `std` or `dep_libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`sqrt_nr`][Float::sqrt_nr].
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
    /// With either `std` or `dep_libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`cbrt_nr`][Float::cbrt_nr].
    #[must_use]
    fn cbrt(self) -> Self;

    /// The hypothenuse (the euclidean distance).
    ///
    /// With either `std` or `dep_libm` enabled it leverages compiler intrinsics,
    /// otherwise it's equal to [`hypot_nr`][Float::hypot_nr].
    #[must_use]
    fn hypot(self, rhs: Self) -> Self;

    /// $e^x$ (the exponential function).
    ///
    /// The maximum values with a representable result are:
    /// 88.722… for `f32` and 709.782… for `f64`.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`exp_series`][Float::exp_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn exp(self) -> Self;

    /// $2^x$.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`exp2_series`][Float::exp2_series]
    /// with [`exp2_series_terms`][Float::exp2_series_terms].
    #[must_use]
    fn exp2(self) -> Self;

    /// The exponential minus 1, more accurately.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`exp_m1_series`][Float::exp_m1_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn exp_m1(self) -> Self;

    /// The natural logarithm of `self`.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`ln_series`][Float::ln_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn ln(self) -> Self;

    /// The natural logarithm of `self` plus 1, more accurately.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`ln_1p_series`][Float::ln_1p_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn ln_1p(self) -> Self;

    /// The logarithm of `self` with respect to an arbitrary `base`.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`log_series`][Float::log_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn log(self, base: Self) -> Self;

    /// The base 2 logarithm of `self`.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`log2_series`][Float::log2_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn log2(self) -> Self;

    /// The base 10 logarithm of `self`.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`log10_series`][Float::log10_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
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
    /// With both `std` and `dep_libm` disabled it leverages
    /// [`sin_series`][Float::sin_series] with 8 terms.
    #[must_use]
    fn sin(self) -> Self;

    /// The cosine.
    ///
    /// With both `std` and `dep_libm` disabled it leverages
    /// [`cos_series`][Float::cos_series] with 8 terms.
    #[must_use]
    fn cos(self) -> Self;

    /// Both the sine and cosine.
    ///
    /// With both `std` and `dep_libm` disabled it leverages
    /// [`sin_cos_series`][Float::sin_cos_series] with 8 terms.
    #[must_use]
    fn sin_cos(self) -> (Self, Self);

    /// The tangent.
    ///
    /// With both `std` and `dep_libm` disabled it leverages
    /// [`tan_series`][Float::tan_series] with 8 terms.
    #[must_use]
    fn tan(self) -> Self;

    /// The arc sine.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`asin_series`][Float::asin_series]
    /// with [`asin_series_terms`][Float::asin_series_terms].
    #[must_use]
    fn asin(self) -> Self;

    /// The arc cosine.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`acos_series`][Float::acos_series]
    /// with [`acos_series_terms`][Float::acos_series_terms].
    #[must_use]
    fn acos(self) -> Self;

    /// The arc tangent.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`atan_series`][Float::atan_series]
    /// with [`atan_series_terms`][Float::atan_series_terms].
    #[must_use]
    fn atan(self) -> Self;

    /// The arc tangent of two variables.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`atan2_series`][Float::atan2_series]
    /// with [`atan_series_terms`][Float::atan_series_terms].
    #[must_use]
    fn atan2(self, other: Self) -> Self;

    /// The hyperbolic sine.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`sinh_series`][Float::sinh_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn sinh(self) -> Self;

    /// The hyperbolic cosine.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`cosh_series`][Float::cosh_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn cosh(self) -> Self;

    /// The hyperbolic tangent.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`cosh_series`][Float::cosh_series]
    /// with [`exp_series_terms`][Float::exp_series_terms].
    #[must_use]
    fn tanh(self) -> Self;

    /// The inverse hyperbolic sine of `self`.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`asinh_series`][Float::asinh_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn asinh(self) -> Self;

    /// The inverse hyperbolic cosine of `self`.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`acosh_series`][Float::acosh_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn acosh(self) -> Self;

    /// The inverse hyperbolic tangent of `self`.
    ///
    /// With both `std` and `dep_libm` disabled it leverages [`atanh_series`][Float::atanh_series]
    /// with [`ln_series_terms`][Float::ln_series_terms].
    #[must_use]
    fn atanh(self) -> Self;

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
    ///
    /// # Features
    /// This will only work if the corresponding `_cmp_[f32|f64]` feature is enabled,
    /// otherwise it will return `NaN`.
    #[must_use]
    fn clamp_total(self, min: Self, max: Self) -> Self;

    /// The maximum of two numbers using total order.
    ///
    /// # Features
    /// This will only work if the corresponding `_cmp_[f32|f64]` feature is enabled,
    /// otherwise it will return `NaN`.
    #[must_use]
    fn max_total(self, other: Self) -> Self;

    /// The minimum of two numbers using total order.
    ///
    /// # Features
    /// This will only work if the corresponding `_cmp_[f32|f64]` feature is enabled,
    /// otherwise it will return `NaN`.
    #[must_use]
    fn min_total(self, other: Self) -> Self;

    /// Evaluates a polynomial at the `self` point value, using [Horner's method].
    ///
    /// [Horner's method]: https://en.wikipedia.org/wiki/Horner%27s_method#Polynomial_evaluation_and_long_division
    #[must_use]
    fn eval_poly(self, coefficients: &[Self]) -> Self;

    /// Approximates the derivative of the 1D function `f` at `x` point using step size `h`.
    ///
    /// Uses the [finite difference method].
    ///
    /// See also the [`autodiff`] attr macro, enabled with the `nightly_autodiff` cfg flag.
    ///
    /// [finite difference method]: https://en.wikipedia.org/wiki/Finite_difference_method
    /// [`autodiff`]: crate::autodiff
    fn derivative<F>(f: F, x: Self, h: Self) -> Self
    where
        F: Fn(Self) -> Self;

    /// Approximates the integral of the 1D function `f` over the range `[x, y]`
    /// using `n` subdivisions.
    ///
    /// Uses the [Riemann Sum](https://en.wikipedia.org/wiki/Riemann_sum).
    fn integrate<F>(f: F, x: Self, y: Self, n: usize) -> Self
    where
        F: Fn(Self) -> Self;

    /// Approximates the partial derivative of the 2D function `f` at point (`x`, `y`)
    /// with step size `h`, differentiating over `x`.
    fn partial_derivative_x<F>(f: F, x: Self, y: Self, h: Self) -> Self
    where
        F: Fn(Self, Self) -> Self;

    /// Approximates the partial derivative of the 2D function `f` at point (`x`, `y`)
    /// with step size `h`, differentiating over `x`.
    fn partial_derivative_y<F>(f: F, x: Self, y: Self, h: Self) -> Self
    where
        F: Fn(Self, Self) -> Self;
}

macro_rules! impl_float_ext {
    () => {
        impl_float_ext![
            (f32, u32 | i32):"_float_f32":"_cmp_f32",
            (f64, u32 | i32):"_float_f64":"_cmp_f32"];
    };

    // $f:   the floating-point type.
    // $ue:  unsigned int type with the same bit-size.
    // $ie:  the integer type for integer exponentiation.
    // $cap: the capability feature that enables the given implementation. E.g "_float_f32".
    // $cmp: the feature that enables the given implementation. E.g "_cmp_f32".
    ($( ($f:ty, $ue:ty|$ie:ty): $cap:literal : $cmp:literal ),+) => {
        $( impl_float_ext![@$f, $ue|$ie, $cap:$cmp]; )+
    };
    (@$f:ty, $ue:ty|$ie:ty, $cap:literal : $cmp:literal) => {
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl ExtFloat for $f {
            fn floor(self) -> Self { Float(self).floor().0 }

            fn ceil(self) -> Self { Float(self).ceil().0 }

            fn round(self) -> Self { Float(self).round_ties_away().0 }

            fn round_ties_away(self) -> Self { Float(self).round_ties_away().0 }

            fn round_ties_even(self) -> Self { Float(self).round_ties_even().0 }

            fn round_ties_odd(self) -> Self { Float(self).round_ties_odd().0 }

            fn trunc(self) -> Self { Float(self).trunc().0 }

            fn fract(self) -> Self { Float(self).fract().0 }

            fn split(self) -> (Self, Self) { let (i, f) = Float(self).split(); (i.0, f.0) }

            fn abs(self) -> Self { Float(self).abs().0 }

            fn neg_abs(self) -> Self { Float(self).neg_abs().0 }

            fn sign(self) -> Sign { Float(self).sign() }

            fn sign_nonzero(self) -> Sign { Float(self).sign_nonzero() }

            fn signum(self) -> Self { Float(self).signum().0 }

            fn flip_sign(self) -> Self { Float(self).flip_sign().0 }

            fn is_sign_positive(self) -> bool { Float(self).is_sign_positive() }

            fn is_sign_negative(self) -> bool { Float(self).is_sign_negative() }

            fn is_zero(self) -> bool { Float(self).is_zero() }

            fn is_sign_positive_nonzero(self) -> bool {
                Float(self).is_sign_positive_nonzero() }

            fn is_sign_negative_nonzero(self) -> bool {
                Float(self).is_sign_negative_nonzero() }

            fn copysign(self, sign: Self) -> Self { Float(self).copysign(sign).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn mul_add(self, mul: Self, add: Self) -> Self {
                Float(self).mul_add(mul, add).0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn mul_add(self, mul: Self, add: Self) -> Self {
                Float(self).mul_add_fallback(mul, add).0
            }

            fn div_euclid(self, rhs: Self) -> Self { Float(self).div_euclid(rhs).0 }

            fn rem_euclid(self, rhs: Self) -> Self { Float(self).rem_euclid(rhs).0 }

            fn scale(self, min: Self, max: Self, u: Self, v: Self) -> Self {
                Float(self).scale(min, max, u, v).0 }
            fn lerp(self, u: Self, v: Self) -> Self { Float(self).lerp(u, v).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn powf(self, y: Self) -> Self { Float(self).powf(y).0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn powf(self, y: Self) -> Self {
                Float(self).powf_series(y, Float(self).ln_series_terms()).0
            }

            fn powi(self, p: $ie) -> Self { Float(self).powi(p).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn sqrt(self) -> Self { Float(self).sqrt().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn sqrt(self) -> Self { Float(self).sqrt_nr().0 }

            fn sqrt_fisr(self) -> Self { Float(self).sqrt_fisr().0 }

            fn fisr(self) -> Self { Float(self).fisr().0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn cbrt(self) -> Self { Float(self).cbrt().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn cbrt(self) -> Self { Float(self).cbrt_nr().0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn hypot(self, rhs: Self) -> Self { Float(self).hypot(rhs).0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn hypot(self, rhs: Self) -> Self { Float(self).hypot_nr(rhs).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn exp(self) -> Self { Float(self).exp().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn exp(self) -> Self {
                Float(self).exp_series(Float(self).exp_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn exp2(self) -> Self { Float(self).exp2().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn exp2(self) -> Self {
                Float(self).exp2_series(Float(self).exp2_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn exp_m1(self) -> Self { Float(self).exp_m1().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn exp_m1(self) -> Self {
                Float(self).exp_m1_series(Float(self).exp_series_terms()).0
            }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn ln(self) -> Self { Float(self).ln().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn ln(self) -> Self {
                Float(self).ln_series(Float(self).ln_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn ln_1p(self) -> Self { Float(self).ln_1p().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn ln_1p(self) -> Self {
                Float(self).ln_1p_series(Float(self).ln_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn log(self, base: Self) -> Self { Float(self).log(base).0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn log(self, base: Self) -> Self {
                Float(self).log_series(base, Float(self).ln_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn log2(self) -> Self { Float(self).log2().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn log2(self) -> Self {
                Float(self).log2_series(Float(self).ln_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn log10(self) -> Self { Float(self).log10().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn log10(self) -> Self {
                Float(self).log10_series(Float(self).ln_series_terms()).0 }

            fn factorial(a: $ue) -> Self { Float::<Self>::factorial(a).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn sin(self) -> Self { Float(self).sin().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn sin(self) -> Self { Float(self).sin_series(8).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn cos(self) -> Self { Float(self).cos().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn cos(self) -> Self { Float(self).cos_series(8).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn sin_cos(self) -> (Self, Self) { let (s, c) = Float(self).sin_cos(); (s.0, c.0) }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn sin_cos(self) -> (Self, Self) {
                let (s, c) = Float(self).sin_cos_series(8); (s.0, c.0) }
            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn tan(self) -> Self { Float(self).tan().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn tan(self) -> Self { Float(self).tan_series(8).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn asin(self) -> Self { Float(self).asin().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn asin(self) -> Self {
                Float(self).asin_series(Float(self).asin_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn acos(self) -> Self { Float(self).acos().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn acos(self) -> Self {
                Float(self).acos_series(Float(self).acos_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn atan(self) -> Self { Float(self).atan().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn atan(self) -> Self {
                Float(self).atan_series(Float(self).atan_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn atan2(self, other: Self) -> Self { Float(self).atan2(other).0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn atan2(self, other: Self) -> Self {
                Float(self).atan2_series(other, Float(self).atan_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn sinh(self) -> Self { Float(self).sinh().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn sinh(self) -> Self {
                Float(self).sinh_series(Float(self).exp_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn cosh(self) -> Self { Float(self).cosh().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn cosh(self) -> Self {
                Float(self).cosh_series(Float(self).exp_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn tanh(self) -> Self { Float(self).tanh().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn tanh(self) -> Self {
                Float(self).tanh_series(Float(self).exp_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn asinh(self) -> Self { Float(self).asinh().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn asinh(self) -> Self {
                Float(self).asinh_series(Float(self).exp_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn acosh(self) -> Self { Float(self).acosh().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn acosh(self) -> Self {
                Float(self).acosh_series(Float(self).exp_series_terms()).0 }

            #[cfg(any(feature = "std", feature = "dep_libm"))]
            fn atanh(self) -> Self { Float(self).atanh().0 }
            #[cfg(not(any(feature = "std", feature = "dep_libm")))]
            fn atanh(self) -> Self {
                Float(self).atanh_series(Float(self).exp_series_terms()).0 }

            fn clamp_nan(self, min: Self, max: Self) -> Self { Float(self).clamp_nan(min, max).0 }

            fn max_nan(self, other: Self) -> Self { Float(self).max_nan(other).0 }

            fn min_nan(self, other: Self) -> Self { Float(self).min_nan(other).0 }

            #[cfg(feature = $cmp)]
            fn clamp_total(self, min: Self, max: Self) -> Self {
                Float(self).clamp_total(min, max).0
            }
            #[cfg(not(feature = $cmp))]
            fn clamp_total(self, _: Self, _: Self) -> Self { <$f>::NAN }

            #[cfg(feature = $cmp)]
            fn max_total(self, other: Self) -> Self { Float(self).max_total(other).0 }
            #[cfg(not(feature = $cmp))]
            fn max_total(self, _: Self) -> Self { <$f>::NAN }

            #[cfg(feature = $cmp)]
            fn min_total(self, other: Self) -> Self { Float(self).min_total(other).0 }
            #[cfg(not(feature = $cmp))]
            fn min_total(self, _: Self) -> Self { <$f>::NAN }

            fn eval_poly(self, coefficients: &[Self]) -> Self {
                Float(self).eval_poly(coefficients).0
            }

            fn derivative<F>(f: F, x: Self, h: Self) -> Self
            where F: Fn(Self) -> Self {
                Float::<Self>::derivative(f, x, h).0
            }
            fn integrate<F>(f: F, x: Self, y: Self, n: usize) -> Self
            where F: Fn(Self) -> Self {
                Float::<Self>::integrate(f, x, y, n).0
            }

            fn partial_derivative_x<F>(f: F, x: Self, y: Self, h: Self) -> Self
            where
                F: Fn(Self, Self) -> Self,
            {
                Float::<Self>::partial_derivative_x(f, x, y, h).0
            }

            fn partial_derivative_y<F>(f: F, x: Self, y: Self, h: Self) -> Self
            where
                F: Fn(Self, Self) -> Self,
            {
                Float::<Self>::partial_derivative_y(f, x, y, h).0
            }
        }
    }
}
impl_float_ext!();
