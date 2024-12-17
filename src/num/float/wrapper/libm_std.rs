// devela::num::float::wrapper::libm_std
//
//! Methods depending on libm, std, or their absence
//
// TOC
// - impls for libm
// - impls for std && not(libm)
// - impls for not(std) && not(libm)
// - macro helper: impl_fp!

use crate::Float;

#[cfg(feature = "dep_libm")]
mod _libm {
    use super::{super::super::shared_docs::*, impl_fp, Float};
    use crate::{_dep::libm::Libm, iif};

    // custom implementations are commented out
    impl_fp![libm:f*:
        r"The largest integer less than or equal to `x`.
        $$ \lfloor x \rfloor = \max \{ n \in \mathbb{Z} \,|\, n \leq x \} $$ "
        floor = floor: ;
        r"The smallest integer greater than or equal to `x`.
        $$ \lceil x \rceil = \min \{ n \in \mathbb{Z} \,|\, n \geq x \} $$"
        ceil = ceil: ;
        "The nearest integer to itself, rounding ties away from `0.0`."
        round = round_ties_away: ;
        "The integral part."
        trunc = trunc: ;
        // fract
        // split == modf
        // abs
        // signum
        "A number composed of its magnitude and the `sign` of other."
        copysign = copysign: sign;
        "Fused multiply-add. Computes (self * mul) + add with only one rounding error."
        fma = mul_add: mul, add;
        // div_euclid
        // rem_euclid
        "Raises itself to the `p` floating point power."
        pow = powf: p;
        // powi
        "Square root."
        sqrt = sqrt: ;
        "$e^x$ (the exponential function)."
        exp = exp: ;
        "$2^x$."
        exp2 = exp2: ;
        "$e^x -1$, more accurately for small values of `x`."
        expm1 = exp_m1: ;
        // ln = ln: x;
        "The natural logarithm."
        log = ln: ;
        "The natural logarithm plus 1, more accurately."
        log1p = ln_1p: ;
        // log
        "The base 2 logarithm."
        log2 = log2: ;
        "The base 10 logarithm."
        log10 = log10: ;
        "The cubic root."
        cbrt = cbrt: ;
        "The hypothenuse (the euclidean distance)."
        hypot = hypot: other;
        "The sine."
        sin = sin: ;
        "The cosine."
        cos = cos: ;
        "The tangent."
        tan = tan: ;
        "The arc sine."
        asin = asin: ;
        "The arc cosine."
        acos = acos: ;
        "The arc tangent."
        atan = atan: ;
        "The arc tangent of two variables."
        atan2 = atan2: other;
        // sin_cos
        "The hyperbolic sine."
        sinh = sinh: ;
        "The hyperbolic cosine."
        cosh = cosh: ;
        "The hyperbolic tangent."
        tanh = tanh: ;
        "The inverse hyperbolic sine."
        asinh = asinh: ;
        "The inverse hyperbolic cosine."
        acosh = acosh: ;
        "The inverse hyperbolic tangent."
        atanh = atanh: ;

        "The minimum of two numbers, ignoring `NaN`."
        fmax = max: other;
        "The minimum of two numbers, ignoring `NaN`."
        fmin = min: other;

        /* only in libm */

        "`10^x`."
        exp10 = exp10: ;
        "The gamma function. Generalizes the factorial function to complex numbers."
        tgamma = gamma: ;
        "The natural logarithm of the absolute value of the gamma function."
        lgamma = lgamma: ;
        "The error function."
        erf = erf: ;
        "The complementary error function (1 - erf)."
        erfc = erfc: ;
        "The bessel function of the first kind, of order 0."
        j0 = j0: ;
        "The bessel function of the first kind, of order 1."
        j1 = j1: ;
        // jn
        "The bessel function of the second kind, of order 0."
        y0 = y0: ;
        "The bessel function of the second kind, of order 1."
        y1 = y1:
        // yn
    ];

    /// $f:   the floating-point type.
    /// $e:   the integer type for integer exponentiation.
    /// $cap: the capability feature enables the given implementation. E.g "_float_f32".
    macro_rules! custom_impls {
        () => {
            custom_impls![(f32, i32):"_float_f32", (f64, i32):"_float_f64"];
        };

        ($( ($f:ty, $e:ty): $cap:literal ),+) => {
            $( custom_impls![@$f, $e, $cap]; )+
        };
        (@$f:ty, $e:ty, $cap:literal) => {
            #[doc = crate::doc_availability!(feature = $cap)]
            ///
            /// # *Implementations using the `libm` feature*.
            #[cfg(feature = $cap )]
            // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
            impl Float<$f> {
                /// The fractional part.
                /// # Formulation
                #[doc = FORMULA_FRACT!()]
                #[must_use]
                pub fn fract(self) -> Float<$f> { Float(self.0 - Libm::<$f>::trunc(self.0)) }

                /// The integral and fractional parts.
                /// # Formulation
                #[doc = FORMULA_SPLIT!()]
                #[must_use]
                pub fn split(self) -> (Float<$f>, Float<$f>) {
                    let (i, f) = Libm::<$f>::modf(self.0);
                    (Self(i), Self(f))
                }

                /// A number that represents the sign of `x`, propagating `NaN`.
                #[must_use]
                pub fn signum(self) -> Float<$f> {
                    if self.0.is_nan() {
                        Self::NAN
                    } else {
                        Float(Libm::<$f>::copysign(1.0, self.0))
                    }
                }

                /// Returns the nearest integer to `x`, rounding ties to the nearest even integer.
                #[must_use]
                pub fn round_ties_even(self) -> Float<$f> {
                    let r = self.round_ties_away();
                    iif![r.0 % 2.0 == 0.0; r;
                        iif![(self - r).abs() == 0.5; r - self.signum(); r]]
                }

                /// Returns itself clamped between `min` and `max`, ignoring `NaN`.
                #[must_use]
                pub fn clamp(self, min: $f, max: $f) -> Float<$f> {
                    self.max(min).min(max)
                }

                /// Raises `x` to the `p` integer power.
                #[must_use]
                pub fn powi(self, p: $e) -> Float<$f> { self.powf(p as $f) }

                /// The logarithm of the number with respect to an arbitrary base.
                #[must_use]
                pub fn log(self, base: $f) -> Float<$f> {
                    // Float(Self::ln(base).0 / Self::ln(self).0)
                    Float(Float(base).ln().0 / self.ln().0)
                }

                /// The sine and cosine.
                #[must_use]
                pub fn sin_cos(self) -> (Float<$f>, Float<$f>) {
                    let (sin, cos) = Libm::<$f>::sincos(self.0);
                    (Float(sin), Float(cos))
                }

                // NOTE: implemented manually in `shared.rs`
                //
                // /// The clamped `x` value, propagating `NaN`.
                // #[must_use]
                // pub fn clamp_nan(x: $f, min: $f, max: $f) -> $f {
                //     Self::min_nan(Self::max_nan(x, min), max)
                // }
                // /// The maximum of two numbers, propagating `NaN`.
                // #[must_use]
                // pub fn max_nan(x: $f, y: $f) -> $f {
                //     iif![x.is_nan() || y.is_nan(); <$f>::NAN; Libm::<$f>::fmax(x, y)]
                // }
                // /// The minimum of two numbers, propagating `NaN`.
                // #[must_use]
                // pub fn min_nan(x: $f, y: $f) -> $f {
                //     iif![x.is_nan() || y.is_nan(); <$f>::NAN; Libm::<$f>::fmin(x, y)]
                // }

                /* only in libm */

                /// The natural logarithm of the absolute value of the gamma function,
                /// plus its sign.
                #[must_use]
                pub fn lgamma_r(self) -> (Float<$f>, $e) {
                    let (f, sign) = Libm::<$f>::lgamma_r(self.0);
                    (Float(f), sign)
                }
                /// Bessel function of the first kind, of order `n`.
                #[must_use]
                pub fn jn(self, n: $e) -> Float<$f> { Float(Libm::<$f>::jn(n, self.0)) }
                /// Bessel function of the second kind, of order `n`.
                #[must_use]
                pub fn yn(self, n: $e) -> Float<$f> { Float(Libm::<$f>::yn(n, self.0)) }
            }
        };
    }
    custom_impls!();
}

#[cfg(all(not(feature = "dep_libm"), feature = "std"))]
mod _std {
    use super::{super::super::shared_docs::*, impl_fp, Float};

    // custom implementations are commented out:
    impl_fp![std:f*:
        r"The largest integer less than or equal to `x`.
        $$ \lfloor x \rfloor = \max \{ n \in \mathbb{Z} \,|\, n \leq x \} $$ "
        floor = floor: ;
        r"The smallest integer greater than or equal to `x`.
        $$ \lceil x \rceil = \min \{ n \in \mathbb{Z} \,|\, n \geq x \} $$"
        ceil = ceil: ;
        "The nearest integer to `x`, rounding ties away from `0.0`."
        round = round_ties_away: ;
        "The nearest integer to `x`, rounding ties to the nearest even integer."
        round_ties_even = round_ties_even: ;
        r"The integral part.
        $$ \text{trunc}(x) = \begin{cases}
        \lfloor x \rfloor, & \text{if } x \geq 0 \\
        \lceil x \rceil, & \text{if } x < 0
        \end{cases} $$"
        trunc = trunc: ;
        r"The fractional part.
        $$ \text{fract}(x) = x - \text{trunc}(x) $$"
        fract = fract: ;
        // split == modf
        // abs
        "A number that represents the sign of `x`."
        signum = signum: ;
        "A number composed of a `magnitude` and a `sign`."
        copysign = copysign: sign;
        "Fused multiply-add. Computes (self * mul) + add with only one rounding error."
        mul_add = mul_add: mul, add;
        // implemented manually for all:
        // div_euclid = div_euclid: other;
        // rem_euclid = rem_euclid: other;
        "Raises itself to the `p` floating point power."
        powf = powf: p;
        // powi
        "The square root."
        sqrt = sqrt: ;
        "$e^x$ (the exponential function)."
        exp = exp: ;
        "$2^x$."
        exp2 = exp2: ;
        "$e^x -1$, more accurately for small values of `x`."
        exp_m1 = exp_m1: ;
        "The natural logarithm."
        ln = ln: ;
        "The natural logarithm plus 1, more accurately."
        ln_1p = ln_1p: ;
        "The logarithm of the number with respect to an arbitrary base."
        log = log: base;
        "The base 2 logarithm."
        log2 = log2: ;
        "The base 10 logarithm."
        log10 = log10: ;
        "The cubic root."
        cbrt = cbrt: ;
        "The hypothenuse (the euclidean distance)."
        hypot = hypot: other;
        "The sine."
        sin = sin: ;
        "The cosine."
        cos = cos: ;
        "The tangent."
        tan = tan: ;
        "The arc sine."
        asin = asin: ;
        "The arc cosine."
        acos = acos: ;
        "The arc tangent."
        atan = atan: ;
        "The arc tangent of two variables."
        atan2 = atan2: other;
        // sin_cos
        "The hyperbolic sine."
        sinh = sinh: ;
        "The hyperbolic cosine."
        cosh = cosh: ;
        "The hyperbolic tangent."
        tanh = tanh: ;
        "The inverse hyperbolic sine."
        asinh = asinh: ;
        "The inverse hyperbolic cosine."
        acosh = acosh: ;
        "The inverse hyperbolic tangent."
        atanh = atanh: ;

        "The clamped value between `min` and `max`, ignoring `NaN`."
        clamp = clamp: min, max;
        "The maximum of two numbers, ignoring `NaN`."
        max = max: other;
        "The minimum of two numbers, ignoring `NaN`."
        min = min: other

        /* not implemented */
        // exp10: https://internals.rust-lang.org/t/enh-add-exp10-and-expf-base-x-f64-f32-methods-to-stdlib-to-symmetrize-api
        // WAIT: (next_up, next_down) [float_next_up_down](https://github.com/rust-lang/rust/issues/91399)
        // WAIT: (gamma, ln_gamma) [float_gamma](https://github.com/rust-lang/rust/issues/99842)
    ];

    /// $f:   the floating-point type.
    /// $e:   the integer type for integer exponentiation.
    /// $cap: the capability feature that enables the given implementation. E.g "_float_f32".
    macro_rules! custom_impls {
        () => {
            custom_impls![(f32, i32):"_float_f32", (f64, i32):"_float_f64"];
        };
        ($( ($f:ty, $e:ty): $cap:literal ),+) => {
            $( custom_impls![@$f, $e, $cap]; )+
        };
        (@$f:ty, $e:ty, $cap:literal) => {
            #[doc = crate::doc_availability!(feature = $cap)]
            ///
            /// # *Implementations using the `std` feature*.
            #[cfg(feature = $cap )]
            // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
            impl Float<$f> {
                /// Raises itself to the `p` integer power.
                #[must_use]
                pub fn powi(self, p: $e) -> Float<$f> { Float(<$f>::powi(self.0, p)) }
                /// Both the sine and cosine.
                #[must_use]
                pub fn sin_cos(self) -> (Float<$f>, Float<$f>) {
                    let (sin, cos) = <$f>::sin_cos(self.0);
                    (Float(sin), Float(cos))
                }
                /// The integral and fractional parts of `x`.
                /// # Formulation
                #[doc = FORMULA_SPLIT!()]
                #[must_use]
                pub fn split(self) -> (Float<$f>, Float<$f>) {
                    let trunc = self.trunc();
                    (trunc, Float(self.0 - trunc.0))
                }
            }
        };
    }
    custom_impls!();
}

#[cfg(all(not(feature = "dep_libm"), not(feature = "std")))]
mod _no_std_no_libm {
    use super::{super::super::shared_docs::*, Float};

    /// $f:   the floating-point type.
    /// $uf:  unsigned int type with the same bit-size.
    /// $ie:  the integer type for integer exponentiation.
    /// $cap: the capability feature that enables the given implementation. E.g "_float_f32".
    macro_rules! custom_impls {
        () => {
            custom_impls![(f32, u32, i32):"_float_f32", (f64, u64, i32):"_float_f64"];
        };
        ($( ($f:ty, $uf:ty, $ie:ty) : $cap:literal ),+) => {
            $( custom_impls![@$f, $uf, $ie, $cap]; )+
        };
        (@$f:ty, $uf:ty, $ie:ty, $cap:literal) => {
            #[doc = crate::doc_availability!(feature = $cap)]
            ///
            /// # *Implementations without `std` or `libm`*.
            #[cfg(feature = $cap )]
            // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
            impl Float<$f> {
                /// The largest integer less than or equal to itself.
                /// # Formulation
                #[doc = crate::FORMULA_FLOOR!()]
                #[must_use]
                pub const fn floor(self) -> Float<$f> { self.const_floor() }

                /// The smallest integer greater than or equal to itself.
                /// # Formulation
                #[doc = FORMULA_CEIL!()]
                #[must_use]
                pub const fn ceil(self) -> Float<$f> { self.const_ceil() }

                /// The nearest integer to itself, default rounding
                ///
                /// This is the default [`round_ties_away`] implementation.
                #[must_use]
                pub const fn round(self) -> Float<$f> { self.const_round() }

                /// The nearest integer to itself, rounding ties away from `0.0`.
                ///
                /// This is the default [`round`] implementation.
                ///
                /// # Formulation
                #[doc = FORMULA_ROUND_TIES_AWAY!()]
                #[must_use]
                pub const fn round_ties_away(self) -> Float<$f> {self.const_round_ties_away() }

                /// Returns the nearest integer to `x`, rounding ties to the nearest even integer.
                /// # Formulation
                #[doc = FORMULA_ROUND_TIES_EVEN!()]
                #[must_use]
                pub const fn round_ties_even(self) -> Float<$f> { self.const_round_ties_even() }

                /// The integral part.
                /// This means that non-integer numbers are always truncated towards zero.
                ///
                /// # Formulation
                #[doc = FORMULA_TRUNC!()]
                ///
                /// This implementation uses bitwise manipulation to remove the fractional part
                /// of the floating-point number. The exponent is extracted, and a mask is
                /// created to remove the fractional part. The new bits are then used to create
                /// the truncated floating-point number.
                #[must_use]
                pub const fn trunc(self) -> Float<$f> { self.const_trunc() }

                /// The fractional part.
                /// # Formulation
                #[doc = FORMULA_FRACT!()]
                #[must_use]
                pub const fn fract(self) -> Float<$f> { self.const_fract() }

                /// The integral and fractional parts.
                /// # Formulation
                #[doc = FORMULA_SPLIT!()]
                #[must_use]
                pub const fn split(self) -> (Float<$f>, Float<$f>) { self.const_split() }

                /// A number that represents its sign, propagating `NaN`.
                #[must_use]
                pub const fn signum(self) -> Float<$f> { self.const_signum() }

                /// A number composed of the magnitude of itself and the `sign` of other.
                #[must_use]
                pub const fn copysign(self, sign: $f) -> Float<$f> { self.const_copysign(sign) }

                /// Returns itself clamped between `min` and `max`, ignoring `NaN`.
                // WAIT:1.85 [const_float_methods](https://github.com/rust-lang/rust/pull/133389)
                #[must_use]
                pub const fn clamp(self, min: $f, max: $f) -> Float<$f> {
                    self.const_clamp(min, max)
                }

                /// The maximum between itself and `other`, ignoring `NaN`.
                // WAIT:1.85 [const_float_methods](https://github.com/rust-lang/rust/pull/133389)
                #[must_use]
                pub const fn max(self, other: $f) -> Float<$f> { self.const_max(other) }

                /// The minimum between itself and other, ignoring `NaN`.
                // WAIT:1.85 [const_float_methods](https://github.com/rust-lang/rust/pull/133389)
                #[must_use]
                pub const fn min(self, other: $f) -> Float<$f> { self.const_min(other) }

                /// Raises itself to the `p` integer power.
                #[must_use]
                pub const fn powi(self, p: $ie) -> Float<$f> { self.const_powi(p) }
            }
        };
    }
    custom_impls!();
}

/// macro helper for implementing methods for `Float`, from either `libm` or `std`.
///
/// $lib: the library to use.
/// $f: the floating-point type to support.
/// $doc: an optional documentation string.
/// $opfn: the original operation function name.
/// $op: the new operation function name in Float.
#[cfg(any(feature = "dep_libm", feature = "std"))]
macro_rules! impl_fp {
    (
    // Matches a wildcard floating-point type (f*).
    // Expands to specific floating-point types (f32, f64).
    $lib:ident : f* : $($ops:tt)*) => {
        impl_fp![$lib : f32 : $($ops)*];
        impl_fp![$lib : f64 : $($ops)*];
    };
    (
    // Matches a specific floating-point type and any number of operations.
    // Generates the impl block for Float<$f> and calls the matching implementation.
    $lib:ident : $f:ty : $($ops:tt)*) => { $crate::paste! {
        #[doc = "# *This implementation block leverages the `" $lib "` feature.*"]
        impl Float<$f> {
            impl_fp![@$lib : $f : $($ops)*];
        }
    }};
    (
    // Matches multiple operations and uses recursion to process each one.
    @$lib:ident : $f:ty : $($doc:literal)? $opfn:ident = $op:ident : $($arg:ident),*
    ; $($rest:tt)*) => {
        impl_fp![@$lib : $f : $($doc)? $opfn = $op : $($arg),*];
        impl_fp![@$lib : $f : $($rest)*];
    };
    (
    // Matches a single operation and implements it using the `libm` library.
    @libm : $f:ty : $($doc:literal)? $opfn:ident = $op:ident : $($arg:ident),*) => {
        $(#[doc = $doc])?
        #[must_use]
        pub fn $op(self, $($arg: $f),*) -> Float<$f> {
            Float($crate::_dep::libm::Libm::<$f>::$opfn(self.0, $($arg),*))
        }
    };
    (
    // Matches a single operation and implements it using the `std` library.
    @std : $f:ty : $($doc:literal)? $opfn:ident = $op:ident : $($arg:ident),*) => {
        $(#[doc = $doc])?
        #[must_use]
        pub fn $op(self, $($arg: $f),*) -> Float<$f> {
            Float(<$f>::$opfn(self.0, $($arg),*))
        }
    };
}
#[cfg(any(feature = "dep_libm", feature = "std"))]
use impl_fp;
