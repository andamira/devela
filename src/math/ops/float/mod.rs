// devela::math::ops::float
//
//! Floating point wrapper.
//
// TOC
// - define Fp struct
// - implement Fp methods
//   - when std is enabled
//   - when libm is enabled
//   - when neither std or libm are enabled

#![cfg_attr(not(feature = "math"), allow(unused))]

mod consts;
mod fp_common;
mod r#trait;

pub use r#trait::FloatExt;

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

// macro helper for implementing methods for `Fp`, from either `libm` or `std`.
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
       r"The largest integer less than or equal to `x`.
        $$ \lfloor x \rfloor = \max \{ n \in \mathbb{Z} \,|\, n \leq x \} $$ "
        floor = floor: x;
        r"The smallest integer greater than or equal to `x`.
        $$ \lceil x \rceil = \min \{ n \in \mathbb{Z} \,|\, n \geq x \} $$"
        ceil = ceil: x;
        "The nearest integer to `x`, rounding ties away from `0.0`."
        round = round_ties_away: x;
        r"The integral part.
        $$ \text{trunc}(x) = \begin{cases}
        \lfloor x \rfloor, & \text{if } x \geq 0 \\
        \lceil x \rceil, & \text{if } x < 0
        \end{cases} $$"
        trunc = trunc: x;
        r"The fractional part.
        $$ \text{fract}(x) = x - \text{trunc}(x) $$"
        fract = fract: x;
        // split == modf
        "The absolute value of `x`."
        abs = abs: x;
        "A number that represents the sign of `x`."
        signum = signum: x;
        "A number composed of a `magnitude` and a `sign`."
        copysign = copysign: magnitude, sign;
        "Fused multiply-add. Computes (x * y) + z with only one rounding error."
        mul_add = mul_add: x, y, z;
        "The euclidean division."
        div_euclid = div_euclid: x, y;
        "The least nonnegative remainder of `x` % `y`."
        rem_euclid = rem_euclid: x, y;
        "Raises `x` to the `p` floating point power."
        powf = powf: x, p;
        // powi
        "The square root."
        sqrt = sqrt: x;
        "$e^x$ (the exponential function)."
        exp = exp: x;
        "$2^x$."
        exp2 = exp2: x;
        "$e^x -1$, more accurately for small values of `x`."
        exp_m1 = exp_m1: x;
        "The natural logarithm."
        ln = ln: x;
        "The natural logarithm plus 1, more accurately."
        ln_1p = ln_1p: x;
        "The logarithm of the number with respect to an arbitrary base."
        log = log: x, y;
        "The base 2 logarithm."
        log2 = log2: x;
        "The base 10 logarithm."
        log10 = log10: x;
        "The cubic root."
        cbrt = cbrt: x;
        "The hypothenuse (the euclidean distance)."
        hypot = hypot: x, y;
        "The sine."
        sin = sin: x;
        "The cosine."
        cos = cos: x;
        "The tangent."
        tan = tan: x;
        "The arc sine."
        asin = asin: x;
        "The arc cosine."
        acos = acos: x;
        "The arc tangent."
        atan = atan: x;
        "The arc tangent of two variables."
        atan2 = atan2: x, y;
        // sin_cos
        "The hyperbolic sine."
        sinh = sinh: x;
        "The hyperbolic cosine."
        cosh = cosh: x;
        "The hyperbolic tangent."
        tanh = tanh: x;
        "The inverse hyperbolic sine."
        asinh = asinh: x;
        "The inverse hyperbolic cosine."
        acosh = acosh: x;
        "The inverse hyperbolic tangent."
        atanh = atanh: x;

        "The maximum of two numbers, ignoring `NaN`."
        max = max: x, y;
        "The minimum of two numbers, ignoring `NaN`."
        min = min: x, y

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
                /// Raises `x` to the `p` integer power.
                #[inline(always)]
                pub fn powi(x: $f, p: $e) -> $f { <$f>::powi(x, p) }
                /// Both the sine and cosine.
                #[inline(always)]
                pub fn sin_cos(x: $f) -> ($f, $f) { <$f>::sin_cos(x) }
                /// The integral and fractional parts of `x`.
                ///
                /// $$ \text{split}(x) = (\text{trunc}(x), \text{fract}(x)) $$
                #[inline(always)]
                pub fn split(x: $f) -> ($f, $f) {
                    let trunc = Self::trunc(x);
                    (trunc, x - trunc)
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
       r"The largest integer less than or equal to `x`.
        $$ \lfloor x \rfloor = \max \{ n \in \mathbb{Z} \,|\, n \leq x \} $$ "
        floor = floor: x;
        r"The smallest integer greater than or equal to `x`.
        $$ \lceil x \rceil = \min \{ n \in \mathbb{Z} \,|\, n \geq x \} $$"
        ceil = ceil: x;
        "The nearest integer to `x`, rounding ties away from `0.0`."
        round = round_ties_away: x;
        "The integral part."
        trunc = trunc: x;
        // fract
        // split == modf
        "The absolute value of `x`."
        fabs = abs: x;
        // signum
        "A number composed of a `magnitude` and a `sign`."
        copysign = copysign: magnitude, sign;
        "Fused multiply-add. Computes (x * y) + z with only one rounding error."
        fma = mul_add: x, y, z;
        // div_euclid
        // rem_euclid
        "Raises `x` to the `p` floating point power."
        pow = powf: x, p;
        // powi
        "Square root."
        sqrt = sqrt: x;
        "$e^x$ (the exponential function)."
        exp = exp: x;
        "$2^x$."
        exp2 = exp2: x;
        "$e^x -1$, more accurately for small values of `x`."
        expm1 = exp_m1: x;
        // ln = ln: x;
        "The natural logarithm."
        log = ln: x;
        "The natural logarithm plus 1, more accurately."
        log1p = ln_1p: x;
        // log
        "The base 2 logarithm."
        log2 = log2: x;
        "The base 10 logarithm."
        log10 = log10: x;
        "The cubic root."
        cbrt = cbrt: x;
        "The hypothenuse (the euclidean distance)."
        hypot = hypot: x, y;
        "The sine."
        sin = sin: x;
        "The cosine."
        cos = cos: x;
        "The tangent."
        tan = tan: x;
        "The arc sine."
        asin = asin: x;
        "The arc cosine."
        acos = acos: x;
        "The arc tangent."
        atan = atan: x;
        "The arc tangent of two variables."
        atan2 = atan2: x, y;
        // sin_cos
        "The hyperbolic sine."
        sinh = sinh: x;
        "The hyperbolic cosine."
        cosh = cosh: x;
        "The hyperbolic tangent."
        tanh = tanh: x;
        "The inverse hyperbolic sine."
        asinh = asinh: x;
        "The inverse hyperbolic cosine."
        acosh = acosh: x;
        "The inverse hyperbolic tangent."
        atanh = atanh: x;

        "The minimum of two numbers, ignoring `NaN`."
        fmax = max: x, y;
        "The minimum of two numbers, ignoring `NaN`."
        fmin = min: x, y;

        /* only in libm */

        "`10^x`."
        exp10 = exp10: x;
        "The gamma function. Generalizes the factorial function to complex numbers."
        tgamma = gamma : x;
        "The natural logarithm of the absolute value of the gamma function."
        lgamma = lgamma : x;
        "The error function."
        erf = erf: x;
        "The complementary error function (1 - erf)."
        erfc = erfc: x;
        "The bessel function of the first kind, of order 0."
        j0 = j0: x;
        "The bessel function of the first kind, of order 1."
        j1 = j1: x;
        // jn
        "The bessel function of the second kind, of order 0."
        y0 = y0: x;
        "The bessel function of the second kind, of order 1."
        y1 = y1: x
        // yn
    ];
    // $f: the floating-point type.
    // $e: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $e:ty) ),+) => { $( custom_impls![@$f, $e]; )+ };
        (@$f:ty, $e:ty) => {
            /// # *Implementations using the `libm` feature*.
            impl Fp<$f> {
                /// The fractional part of `x`.
                ///
                /// $$ \text{fract}(x) = x - \lfloor x \rfloor $$
                #[must_use] #[inline(always)]
                pub fn fract(x: $f) -> $f { x - Libm::<$f>::trunc(x) }

                /// The integral and fractional parts of `x`.
                ///
                /// $$ \text{split}(x) = (\text{trunc}(x), \text{fract}(x)) $$
                #[must_use] #[inline(always)]
                pub fn split(x: $f) -> ($f, $f) { Libm::<$f>::modf(x) }

                /// A number that represents the sign of `x`, propagating `NaN`.
                #[must_use] #[inline]
                pub fn signum(x: $f) -> $f {
                    iif![x.is_nan(); <$f>::NAN; Libm::<$f>::copysign(1.0, x)]
                }

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

                /// Raises `x` to the `p` integer power.
                #[must_use] #[inline(always)]
                pub fn powi(x: $f, p: $e) -> $f { Self::powf(x, p as $f) }

                /// The logarithm of the number with respect to an arbitrary base.
                #[must_use] #[inline(always)]
                pub fn log(x: $f, base: $f) -> $f { Self::ln(base) / Self::ln(x) }

                /// The sine and cosine.
                #[must_use] #[inline(always)]
                pub fn sin_cos(x: $f) -> ($f, $f) { Libm::<$f>::sincos(x) }

                // NOTE: implemented manually in _either
                //
                // /// The clamped `x` value, propagating `NaN`.
                // #[must_use] #[inline(always)]
                // pub fn clamp_nan(x: $f, min: $f, max: $f) -> $f {
                //     Self::min_nan(Self::max_nan(x, min), max)
                // }
                // /// The maximum of two numbers, propagating `NaN`.
                // #[must_use] #[inline(always)]
                // pub fn max_nan(x: $f, y: $f) -> $f {
                //     iif![x.is_nan() || y.is_nan(); <$f>::NAN; Libm::<$f>::fmax(x, y)]
                // }
                // /// The minimum of two numbers, propagating `NaN`.
                // #[must_use] #[inline(always)]
                // pub fn min_nan(x: $f, y: $f) -> $f {
                //     iif![x.is_nan() || y.is_nan(); <$f>::NAN; Libm::<$f>::fmin(x, y)]
                // }

                /* only in libm */

                /// The natural logarithm of the absolute value of the gamma function,
                /// plus its sign.
                #[must_use] #[inline(always)]
                pub fn lgamma_r(x: $f) -> ($f, $e) { Libm::<$f>::lgamma_r(x) }
                /// Bessel function of the first kind, of order `n`.
                #[must_use] #[inline(always)]
                pub fn jn(n: $e, x: $f) -> $f { Libm::<$f>::jn(n, x) }
                /// Bessel function of the second kind, of order `n`.
                #[must_use] #[inline(always)]
                pub fn yn(n: $e, x: $f) -> $f { Libm::<$f>::yn(n, x) }
            }
        };
    }
    custom_impls![(f32, i32), (f64, i32)];
}

#[cfg(all(not(feature = "libm"), not(feature = "std")))]
mod _no_std_no_libm {
    use super::Fp;
    use crate::meta::iif;

    // $f: the floating-point type.
    // $ub: unsigned int type with the same bit-size.
    // $ie: the integer type for integer exponentiation.
    macro_rules! custom_impls {
        ($( ($f:ty, $ub:ty, $ie:ty) ),+) => { $( custom_impls![@$f, $ub, $ie]; )+ };
        (@$f:ty, $ub:ty, $ie:ty) => { $crate::meta::paste! {
            /// # *Implementations without `std` or `libm`*.
            impl Fp<$f> {
                /// The largest integer less than or equal to `x`.
                ///
                /// $$ \lfloor x \rfloor = \max \{ n \in \mathbb{Z} \,|\, n \leq x \} $$
                #[must_use] #[inline]
                pub fn floor(x: $f) -> $f {
                    let mut result = Self::trunc(x);
                    if x.is_sign_negative() && Self::abs(x - result) > <$f>::EPSILON {
                        result -= 1.0;
                    }
                    result
                }

                /// The smallest integer greater than or equal to `x`.
                ///
                /// $$ \lceil x \rceil = \min \{ n \in \mathbb{Z} \,|\, n \geq x \} $$
                #[must_use] #[inline]
                pub fn ceil(x: $f) -> $f {
                    let mut result = Self::trunc(x);
                    if x.is_sign_positive() && Self::abs(x - result) > <$f>::EPSILON {
                        result += 1.0;
                    }
                    result
                }

                /// The nearest integer to `self`, default rounding
                ///
                /// This is the default [`round_ties_away`] implementation.
                #[must_use] #[inline]
                pub fn round(x: $f) -> $f {
                    Self::trunc(x + Self::copysign(0.5 - 0.25 * <$f>::EPSILON, x))
                }

                /// The nearest integer to `self`, rounding ties away from `0.0`.
                ///
                /// This is the default [`round`] implementation.
                ///
                /// $$
                /// \text{round\\_ties\\_away}(x) = \begin{cases}
                /// \lceil x \rceil, & \text{if } x - \lfloor x \rfloor > 0.5 \text{ or }
                ///     (x - \lfloor x \rfloor = 0.5 \text{ and } x > 0) \cr
                /// \lfloor x \rfloor, & \text{if } x - \lfloor x \rfloor < 0.5 \text{ or }
                ///     (x - \lfloor x \rfloor = 0.5 \text{ and } x < 0)
                /// \end{cases}
                /// $$
                #[must_use] #[inline]
                pub fn round_ties_away(x: $f) -> $f {
                    Self::trunc(x + Self::copysign(0.5 - 0.25 * <$f>::EPSILON, x))
                }

                /// The integral part.
                /// This means that non-integer numbers are always truncated towards zero.
                ///
                /// $$
                /// \text{trunc}(x) = \begin{cases}
                /// \lfloor x \rfloor, & \text{if } x \geq 0 \cr
                /// \lceil x \rceil, & \text{if } x < 0
                /// \end{cases}
                /// $$
                ///
                /// This implementation uses bitwise manipulation to remove the fractional part
                /// of the floating-point number. The exponent is extracted, and a mask is
                /// created to remove the fractional part. The new bits are then used to create
                /// the truncated floating-point number.
                #[must_use] #[inline]
                pub fn trunc(x: $f) -> $f {
                    let bits = x.to_bits();
                    const BIAS: $ie = Fp::<$f>::BIAS as $ie;
                    const SIG_BITS: $ie = Fp::<$f>::SIGNIFICAND_BITS as $ie;
                    const EXP_MASK: $ub = 1 << Fp::<$f>::EXPONENT_BITS as $ub - 1;

                    #[allow(clippy::cast_possible_wrap)]
                    let exponent = ((bits >> SIG_BITS) & EXP_MASK) as $ie - BIAS;
                    if exponent < 0 {
                        iif![x.is_sign_positive(); 0.0; -0.0]
                    } else if exponent < SIG_BITS {
                        let mask = !(([<1_ $ub>] << (SIG_BITS - exponent)) - 1);
                        let new_bits = bits & mask;
                        <$f>::from_bits(new_bits)
                    } else {
                        x
                    }
                }

                /// The fractional part.
                ///
                /// $$ \text{fract}(x) = x - \text{trunc}(x) $$
                #[must_use] #[inline(always)]
                pub fn fract(x: $f) -> $f { x - Self::trunc(x) }

                /// The integral and fractional parts.
                ///
                /// $$ \text{split}(x) = (\text{trunc}(x), \text{fract}(x)) $$
                #[must_use] #[inline(always)]
                pub fn split(x: $f) -> ($f, $f) {
                    let trunc = Self::trunc(x);
                    (trunc, x - trunc)
                }

                /// The absolute value.
                #[must_use] #[inline]
                pub fn abs(x: $f) -> $f {
                    let mask = <$ub>::MAX / 2;
                    let bits: $ub = x.to_bits() & mask;
                    <$f>::from_bits(bits)
                }

                /// A number that represents the sign of `x`, propagating `NaN`.
                #[must_use] #[inline]
                pub fn signum(x: $f) -> $f {
                    iif![x.is_nan(); <$f>::NAN; Self::copysign(1.0, x)]
                }

                /// A number composed of a `magnitude` and a `sign`.
                #[must_use] #[inline(always)]
                pub fn copysign(magnitude: $f, sign: $f) -> $f {
                    const SIGN_MASK: $ub = <$ub>::MAX / 2 + 1;
                    const VALUE_MASK: $ub = <$ub>::MAX / 2;
                    let sign_bit = sign.to_bits() & SIGN_MASK;
                    let value_bits = magnitude.to_bits() & VALUE_MASK;
                    <$f>::from_bits(value_bits | sign_bit)
                }

                /// The maximum of two numbers, ignoring `NaN`.
                #[must_use] #[inline]
                pub fn max(x: $f, y: $f) -> $f {
                    (if x.is_nan() || x < y { y } else { x }) * 1.0
                }

                /// The minimum of two numbers, ignoring `NaN`.
                #[must_use] #[inline]
                pub fn min(x: $f, y: $f) -> $f {
                    (iif![y.is_nan() || x < y; x; y]) * 1.0
                }

                /// Raises `x` to the `p` integer power.
                #[must_use] #[inline]
                pub fn powi(x: $f, p: $ie) -> $f {
                    match p {
                        0 => 1.0,
                        1.. => {
                            let mut result = x;
                            for _i in 1..p {
                                result *= x;
                            }
                            result
                        }
                        _ => {
                            let mut result = x;
                            for _i in 1..p.abs() {
                                result /= x;
                            }
                            result
                        }
                    }
                }
            }
        }};
    }
    custom_impls![(f32, u32, i32), (f64, u64, i32)];
}