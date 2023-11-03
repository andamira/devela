// devela::ops::ext::float
//
//! Floating point wrapper.
//
// TOC
// - FloatExt trait
// - Fp wrapper

#![cfg_attr(not(feature = "ops"), allow(unused))]

/// Extension trait for floating-point types.
///
/// This trait is normally more convenient to use than the [`Fp`] struct.
///
/// `Fp` has a few more methods implemented if the `libm` feature is enabled,
/// and some of the methods are const if the `unsafe_ops` feature is enabled.
#[rustfmt::skip]
pub trait FloatExt: Sized {
    /// The largest integer less than or equal to `self`.
    fn floor(self) -> Self;
    /// The smallest integer greater than or equal to `self`.
    fn ceil(self) -> Self;
    /// The nearest integer to `self`, half away from `0.0`.
    fn round(self) -> Self;
    /// The integral part.
    fn trunc(self) -> Self;
    /// The fractional part.
    fn fract(self) -> Self;
    /// Returns the integral and fractional parts.
    fn split(self) -> (Self, Self);
    /// The absolute value.
    fn abs(self) -> Self;
    /// A number that represents the sign of `self`.
    fn signum(self) -> Self;
    /// A number composed of a magnitude of `self` and the sign of `sign`.
    fn copysign(self, sign: Self) -> Self;
    /// Fused multiply-add. Computes `(self * mul) + add` with only one rounding error.
    fn mul_add(self, mul: Self, add: Self) -> Self;
    /// The euclidean division.
    fn div_euclid(self, rhs: Self) -> Self;
    /// The least nonnegative remainder of `self % rhs`.
    fn rem_euclid(self, rhs: Self) -> Self;
    /// Raises `self` to the `n` floating point power.
    fn powf(self, n: Self) -> Self;
    /// Raises `self` to the `n` integer power.
    fn powi(self, n: i32) -> Self;
    /// The square root.
    fn sqrt(self) -> Self;
    /// Returns `e^a` (the exponential function).
    fn exp(self) -> Self;
    /// Returns `2^a`.
    fn exp2(self) -> Self;
    /// The exponential minus 1, more accurately.
    fn exp_m1(self) -> Self;
    /// The natural logarithm.
    fn ln(self) -> Self;
    /// The natural logarithm plus 1, more accurately.
    fn ln_1p(self) -> Self;
    /// The logarithm of the number with respect to an arbitrary `base`.
    fn log(self, base: Self) -> Self;
    /// The base 2 logarithm.
    fn log2(self) -> Self;
    /// The base 10 logarithm.
    fn log10(self) -> Self;
    /// The cubic root.
    fn cbrt(self) -> Self;
    /// The hypothenuse (the euclidean distance).
    fn hypot(self, rhs: Self) -> Self;
    /// The sine.
    fn sin(self) -> Self;
    /// The cosine.
    fn cos(self) -> Self;
    /// The tangent.
    fn tan(self) -> Self;
    /// The arc sine.
    fn asin(self) -> Self;
    /// The arc cosine.
    fn acos(self) -> Self;
    /// The arc tangent.
    fn atan(self) -> Self;
    /// The arc tangent of two variables.
    fn atan2(self, other: Self) -> Self;
    /// Returns both the sine and cosine.
    fn sin_cos(self) -> (Self, Self);
    /// The hyperbolic sine.
    fn sinh(self) -> Self;
    /// The hyperbolic cosine.
    fn cosh(self) -> Self;
    /// The hyperbolic tangent.
    fn tanh(self) -> Self;
    /// The inverse hyperbolic sine.
    fn asinh(self) -> Self;
    /// The inverse hyperbolic cosine.
    fn acosh(self) -> Self;
    /// The inverse hyperbolic tangent.
    fn atanh(self) -> Self;

    /// Returns the clamped value, ignoring `NaN`.
    fn clamp(self, min: Self, max: Self) -> Self;
    /// Returns the maximum of two numbers, ignoring `NaN`.
    fn max(self, other: Self) -> Self;
    /// Returns the minimum of two numbers, ignoring `NaN`.
    fn min(self, other: Self) -> Self;
    
    /// Returns the clamped value, propagating `NaN`.
    fn clamp_nan(self, min: Self, max: Self) -> Self;
    /// Returns the maximum of two numbers, propagating `NaN`.
    fn max_nan(self, other: Self) -> Self;
    /// Returns the minimum of two numbers, propagating `NaN`.
    fn min_nan(self, other: Self) -> Self;

    /// Returns the clamped value, using total order.
    fn clamp_total(self, min: Self, max: Self) -> Self;
    /// Returns the maximum of two numbers using total order.
    fn max_total(self, other: Self) -> Self;
    /// Returns the minimum of two numbers using total order.
    fn min_total(self, other: Self) -> Self;
}

macro_rules! impl_float_ext {
    ($($t:ty),+) => { $( impl_float_ext![@$t]; )+ };
    (@$t:ty) => {
        impl FloatExt for $t {
            fn floor(self) -> Self { Fp::<$t>::floor(self) }
            fn ceil(self) -> Self { Fp::<$t>::ceil(self) }
            fn round(self) -> Self { Fp::<$t>::round(self) }
            fn trunc(self) -> Self { Fp::<$t>::trunc(self) }
            fn fract(self) -> Self { Fp::<$t>::fract(self) }
            fn split(self) -> (Self, Self) { Fp::<$t>::split(self) }
            fn abs(self) -> Self { Fp::<$t>::abs(self) }
            fn signum(self) -> Self { Fp::<$t>::signum(self) }
            fn copysign(self, sign: Self) -> Self { Fp::<$t>::copysign(self, sign) }
            fn mul_add(self, mul: Self, add: Self) -> Self { Fp::<$t>::mul_add(self, mul, add) }
            fn div_euclid(self, rhs: Self) -> Self { Fp::<$t>::div_euclid(self, rhs) }
            fn rem_euclid(self, rhs: Self) -> Self { Fp::<$t>::rem_euclid(self, rhs) }
            fn powf(self, n: Self) -> Self { Fp::<$t>::powf(self, n) }
            fn powi(self, n: i32) -> Self { Fp::<$t>::powi(self, n) }
            fn sqrt(self) -> Self { Fp::<$t>::sqrt(self) }
            fn exp(self) -> Self { Fp::<$t>::exp(self) }
            fn exp2(self) -> Self { Fp::<$t>::exp2(self) }
            fn exp_m1(self) -> Self { Fp::<$t>::exp_m1(self) }
            fn ln(self) -> Self { Fp::<$t>::ln(self) }
            fn ln_1p(self) -> Self { Fp::<$t>::ln_1p(self) }
            fn log(self, base: Self) -> Self { Fp::<$t>::log(self, base) }
            fn log2(self) -> Self { Fp::<$t>::log2(self) }
            fn log10(self) -> Self { Fp::<$t>::log10(self) }
            fn cbrt(self) -> Self { Fp::<$t>::cbrt(self) }
            fn hypot(self, rhs: Self) -> Self { Fp::<$t>::hypot(self, rhs) }
            fn sin(self) -> Self { Fp::<$t>::sin(self) }
            fn cos(self) -> Self { Fp::<$t>::cos(self) }
            fn tan(self) -> Self { Fp::<$t>::tan(self) }
            fn asin(self) -> Self { Fp::<$t>::asin(self) }
            fn acos(self) -> Self { Fp::<$t>::acos(self) }
            fn atan(self) -> Self { Fp::<$t>::atan(self) }
            fn atan2(self, other: Self) -> Self { Fp::<$t>::atan2(self, other) }
            fn sin_cos(self) -> (Self, Self) { Fp::<$t>::sin_cos(self) }
            fn sinh(self) -> Self { Fp::<$t>::sinh(self) }
            fn cosh(self) -> Self { Fp::<$t>::cosh(self) }
            fn tanh(self) -> Self { Fp::<$t>::tanh(self) }
            fn asinh(self) -> Self { Fp::<$t>::asinh(self) }
            fn acosh(self) -> Self { Fp::<$t>::acosh(self) }
            fn atanh(self) -> Self { Fp::<$t>::atanh(self) }
            fn clamp(self, min: Self, max: Self) -> Self { Fp::<$t>::clamp(self, min, max) }
            fn max(self, other: Self) -> Self { Fp::<$t>::max(self, other) }
            fn min(self, other: Self) -> Self { Fp::<$t>::min(self, other) }
            fn clamp_nan(self, min: Self, max: Self) -> Self { Fp::<$t>::clamp_nan(self, min, max) }
            fn max_nan(self, other: Self) -> Self { Fp::<$t>::max_nan(self, other) }
            fn min_nan(self, other: Self) -> Self { Fp::<$t>::min_nan(self, other) }
            fn max_total(self, other: Self) -> Self { Fp::<$t>::max_total(self, other) }
            fn min_total(self, other: Self) -> Self { Fp::<$t>::min_total(self, other) }
            fn clamp_total(self, min: Self, max: Self) -> Self {
                Fp::<$t>::clamp_total(self, min, max)
            }
        }
    }
}
impl_float_ext![f32, f64];

/// Floating-point operations wrapper over `std` or `libm`.
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

// $lib: the library to use.
// $f: the floating-point type to support.
// $doc: an optional documentation string.
// $opfn: the original operation function name.
// $op: the new operation function name in Fp.
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
        #[doc = "*These implementations are using the `" $lib "` feature.*"]
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
        "The nearest integer to `a`, half away from `0.0`."
        round = round: a;
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
        "Raises `a` to the `b` floating point power."
        powf = powf: a, b;
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
            /// *These implementations are using the `std` feature*.
            impl Fp<$f> {
                /// Raises `a` to the `b` integer power.
                #[inline(always)]
                pub fn powi(a: $f, b: $e) -> $f { <$f>::powi(a, b) }
                /// Both the sine and cosine.
                #[inline(always)]
                pub fn sin_cos(a: $f) -> ($f, $f) { <$f>::sin_cos(a) }
                /// Returns the integral and fractional parts.
                #[inline(always)]
                pub fn split(value: $f) -> ($f, $f) { (value.trunc(), value.fract()) }

                /// Returns the clamped a value, propagating `NaN`.
                #[inline(always)]
                pub fn clamp_nan(value: $f, min: $f, max: $f) -> $f {
                    Fp::<$f>::min_nan(Fp::<$f>::max_nan(value, min), max)
                }
                /// Returns the maximum of two numbers, propagating `NaN`.
                // WAIT: https://github.com/rust-lang/rust/issues/91079
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
        "The nearest integer to `a`, half away from `0.0`."
        round = round: a;
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
        "Raises `a` to the `b` floating point power."
        pow = powf: a, b;
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
            /// *These implementations are using the `libm` feature*.
            impl Fp<$f> {
                /// Returns the fractional part.
                #[inline(always)]
                pub fn fract(a: $f) -> $f { a - Libm::<$f>::trunc(a) }
                /// The integral and fractional parts.
                #[inline(always)]
                pub fn split(value: $f) -> ($f, $f) { Libm::<$f>::modf(value) }
                /// A number that represents the sign of `a`.
                #[inline(always)]
                pub fn signum(a: $f) -> $f { a - Libm::<$f>::copysign(1.0 as $f, a) }
                /// The euclidean division.
                #[inline(always)]
                pub fn div_euclid(a: $f, b: $f) -> $f {
                    let q = Fp::<$f>::trunc(a / b);
                    iif![a % b < 0.0; return iif![b > 0.0; q - 1.0; q + 1.0]]; q
                }
                /// The least nonnegative remainder of `a` % `b`.
                #[inline(always)]
                pub fn rem_euclid(a: $f, b: $f) -> $f {
                    let r = a % b; iif![r < 0.0; r + Fp::<$f>::abs(b); r]
                }
                /// Raises `a` to the `b` integer power.
                #[inline(always)]
                pub fn powi(a: $f, b: $e) -> $f { Fp::<$f>::powf(a, b as $f) }
                /// The logarithm of the number with respect to an arbitrary base.
                #[inline(always)]
                pub fn log(value: $f, base: $f) -> $f { Fp::<$f>::ln(base) / Fp::<$f>::ln(value) }
                /// The sine and cosine.
                #[inline(always)]
                pub fn sin_cos(a: $f) -> ($f, $f) { Libm::<$f>::sincos(a) }

                /// Returns the clamped a value, propagating `NaN`.
                #[inline(always)]
                pub fn clamp_nan(value: $f, min: $f, max: $f) -> $f {
                    Fp::<$f>::min_nan(Fp::<$f>::max_nan(value, min), max)
                }
                /// Returns the maximum of two numbers, propagating `NaN`.
                #[inline(always)]
                pub fn max_nan(a: $f, b: $f) -> $f {
                    iif![a.is_nan() || b.is_nan(); <$f>::NAN; Libm::<$f>::fmax(a, b)]
                }
                /// Returns the minimum of two numbers, propagating `NaN`.
                #[inline(always)]
                pub fn min_nan(a: $f, b: $f) -> $f {
                    iif![a.is_nan() || b.is_nan(); <$f>::NAN; Libm::<$f>::fmin(a, b)]
                }

                /* only in libm */

                /// The natural logarithm of the absolute value of the gamma function,
                /// plus its sign.
                #[inline(always)]
                pub fn lgamma_r(a: $f) -> ($f, $e) { Libm::<$f>::lgamma_r(a) }
                /// Bessel function of the first kind, of order `n`.
                #[inline(always)]
                pub fn jn(n: $e, a: $f) -> $f { Libm::<$f>::jn(n, a) }
                /// Bessel function of the second kind, of order `n`.
                #[inline(always)]
                pub fn yn(n: $e, a: $f) -> $f { Libm::<$f>::yn(n, a) }
            }
        };
    }
    custom_impls![(f32, i32), (f64, i32)];
}

mod _either {
    #![allow(missing_docs)]

    use super::Fp;

    // $f: the floating-point type.
    // $e: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $e:ty) ),+) => { $( custom_impls![@$f, $e]; )+ };
        (@$f:ty, $e:ty) => { $crate::meta::paste! {
            /// *Common implementations for either `libm` or `std`*.
            ///
            /// Const fns will only be `const` if the `unsafe_ops` feature is enabled.
            impl Fp<$f> {
                /// Returns the clamped value, ignoring `NaN`.
                #[inline(always)]
                pub fn clamp(value: $f, min: $f, max: $f) -> $f {
                    Fp::<$f>::min(Fp::<$f>::max(value, min), max)
                }

                /// Returns the clamped value, using total order.
                #[inline(always)]
                #[cfg(feature = "unsafe_ops")]
                pub const fn clamp_total(value: $f, min: $f, max: $f) -> $f {
                    $crate::ops::[<clamp_ $f>](value, min, max)
                }
                #[inline(always)]
                #[cfg(not(feature = "unsafe_ops"))]
                pub fn clamp_total(value: $f, min: $f, max: $f) -> $f {
                    $crate::ops::[<clamp_ $f>](value, min, max)
                }

                /// Returns the maximum of two numbers using total order.
                #[inline(always)]
                #[cfg(feature = "unsafe_ops")]
                pub const fn max_total(a: $f, b: $f) -> $f { $crate::ops::[<max_ $f>](a, b) }
                #[inline(always)]
                #[cfg(not(feature = "unsafe_ops"))]
                pub fn max_total(a: $f, b: $f) -> $f { $crate::ops::[<max_ $f>](a, b) }

                /// Returns the minimum of two numbers using total order.
                #[inline(always)]
                #[cfg(feature = "unsafe_ops")]
                pub const fn min_total(a: $f, b: $f) -> $f { $crate::ops::[<min_ $f>](a, b) }
                #[inline(always)]
                #[cfg(not(feature = "unsafe_ops"))]
                pub fn min_total(a: $f, b: $f) -> $f { $crate::ops::[<min_ $f>](a, b) }
            }
        }};
    }
    custom_impls![(f32, i32), (f64, i32)];
}
