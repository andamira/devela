// devela::num::fop
//
//! Floating operations wrapper.
//

#![cfg_attr(not(feature = "ops"), allow(unused))]
#![allow(missing_docs)]

/// Floating-point operations wrapper over `std` or `libm`.
///
/// It favors `std` style for method's names.
///
/// If both the `libm` and `std` features are enabled the `libm` functions will
/// be used, since it contains more functions, namely:
/// - Gamma functions: [`gamma`][Fp#method.gamma], [`lgamma`][Fp#method.lgamma],
///   [`lgamma_r`][Fp#method.lgamma_r].
/// - Bessel functions:
///   [`j0`][Fp#method.j0], [`j1`][Fp#method.j1], [`jn`][Fp#method.jn],
///   [`y0`][Fp#method.y0], [`y1`][Fp#method.y1], [`yn`][Fp#method.yn].
/// - Error functions: [`erf`][Fp#method.erf], [`erfc`][Fp#method.erfc].
/// - [`minimum`][Fp#method.minimum], [`maximum`][Fp#method.maximum].
/// - [`exp10`][Fp#method.exp10].
#[derive(Debug, Clone, Copy)]
#[cfg(any(feature = "std", feature = "libm"))]
#[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
pub struct Fp<T>(core::marker::PhantomData<T>);

// $lib: the library to use.
// $f: the floating-point type to support.
// $doc: an optional documentation string.
// $opfn: the original operation function name.
// $op: the new operation function name in Fp.
#[cfg(any(feature = "std", feature = "libm"))]
macro_rules! impl_fp {
    // Matches a wildcard floating-point type (f*).
    // Expands to specific floating-point types (f32, f64).
    ($lib:ident : f* : $($ops:tt)*) => {
        impl_fp![$lib : f32 : $($ops)*];
        impl_fp![$lib : f64 : $($ops)*];
    };
    // Matches a specific floating-point type and any number of operations.
    // Generates the impl block for Fp<$f> and calls the matching implementation.
    ($lib:ident : $f:ty : $($ops:tt)*) => {
        impl Fp<$f> {
            impl_fp![@$lib : $f : $($ops)*];
        }
    };
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
#[cfg(any(feature = "std", feature = "libm"))]
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

        "Returns the maximum of two numbers. If either of them is `NaN` returns `NaN`."
        max = max: a, b;
        "Returns the maximum of two numbers. If either of them is `NaN` returns `NaN`."
        min = min: a, b

        /* not implemented */
        // exp10: https://internals.rust-lang.org/t/enh-add-exp10-and-expf-base-x-f64-f32-methods-to-stdlib-to-symmetrize-api
        // gamma, ln_gamma: WAIT: https://github.com/rust-lang/rust/issues/99842
        // next_up, next_down: WAIT: https://github.com/rust-lang/rust/issues/91399
        // maximum, minimum: WAIT: https://github.com/rust-lang/rust/issues/91079
    ];

    // $f: the floating-point type.
    // $e: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $e:ty) ),+) => { $( custom_impls![@$f, $e]; )+ };
        (@$f:ty, $e:ty) => {
            /// Custom implementations.
            impl Fp<$f> {
                /// Raises `a` to the `b` integer power.
                #[inline(always)]
                pub fn powi(a: $f, b: $e) -> $f { <$f>::powi(a, b) }
                /// Both the sine and cosine.
                #[inline(always)]
                pub fn sin_cos(a: $f) -> ($f, $f) { <$f>::sin_cos(a) }
                /// Returns the ingraral and fractional parts.
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

        /* only in libm */

        // max
        // min
        "Returns the minimum of two numbers. If one of them is `NaN` returns the other."
        fmax = maximum: a, b;
        "Returns the minimum of two numbers. If one of them is `NaN` returns the other."
        fmin = minimum: a, b;
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
            /// Custom implementations.
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

                /// Returns the maximum of two numbers. If either of them is `NaN` returns `NaN`.
                #[inline(always)]
                pub fn max(a: $f, b: $f) -> $f {
                    iif![a.is_nan() || b.is_nan(); <$f>::NAN; Libm::<$f>::fmax(a, b)]
                }
                /// Returns the minimum of two numbers. If either of them is `NaN` returns `NaN`.
                #[inline(always)]
                pub fn min(a: $f, b: $f) -> $f {
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
