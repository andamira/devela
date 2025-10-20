// devela::num::float::wrapper::std
//
//! Methods depending on std, or its absence
//
// TOC
// - macro impl_fp!
// - impls for std
// - impls for not(std)

use crate::Float;

/// Macro helper for implementing methods for `Float`, from `std`.
///
/// $lib: the library to use.
/// $f: the floating-point type to support.
/// $doc: an optional documentation string.
/// $opfn: the original operation function name.
/// $op: the new operation function name in Float.
#[cfg(feature = "std")]
macro_rules! impl_fp {
    (
        // Matches a wildcard floating-point type (f*).
        // Expands to specific floating-point types (f32, f64).
        $lib:ident : f* : $($ops:tt)*
    ) => {
        impl_fp![$lib : f32 : $($ops)*];
        impl_fp![$lib : f64 : $($ops)*];
    };
    (
        // Matches a specific floating-point type and any number of operations.
        // Generates the impl block for Float<$f> and calls the matching implementation.
        $lib:ident : $f:ty : $($ops:tt)*
    ) => { $crate::paste! {
        #[doc = "# *This implementation block leverages the `" $lib "` feature.*"]
        impl Float<$f> {
            impl_fp![@$lib : $f : $($ops)*];
        }
    }};
    (
        // Matches multiple operations and uses recursion to process each one.
        @$lib:ident : $f:ty : $($doc:literal)? $opfn:ident = $op:ident : $($arg:ident),*
        ; $($rest:tt)*
    ) => {
        impl_fp![@$lib : $f : $($doc)? $opfn = $op : $($arg),*];
        impl_fp![@$lib : $f : $($rest)*];
    };
    (
        // Matches a single operation and implements it using the `std` library.
        @std : $f:ty : $($doc:literal)? $opfn:ident = $op:ident : $($arg:ident),* $(;)?
    ) => {
        $(#[doc = $doc])?
        pub fn $op(self, $($arg: $f),*) -> Float<$f> {
            Float(<$f>::$opfn(self.0, $($arg),*))
        }
    };
}
#[cfg(feature = "std")]
use impl_fp;

#[cfg(feature = "std")]
mod _std {
    use super::{Float, impl_fp};

    // custom implementations are commented out:
    impl_fp![std:f*:
        r"The largest integer less than or equal to `x`.
        $$ \lfloor x \rfloor = \max \{ n \in ℤ \,|\, n \leq x \} $$ "
        floor = floor: ;
        r"The smallest integer greater than or equal to `x`.
        $$ \lceil x \rceil = \min \{ n \in ℤ \,|\, n \geq x \} $$"
        ceil = ceil: ;
        "The nearest integer to `x`, rounding ties away from `0.0`."
        round = round_ties_away: ;
        "The nearest integer to `x`, rounding ties to the nearest even integer."
        round_ties_even = round_ties_even: ;
        // trunc = trunc: ;
        // fract = fract: ;
        // split == modf
        // abs
        // signum = signum: ;
        // copysign = copysign: sign;
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
        atanh = atanh:
        // clamp = clamp: min, max;
        // max = max: other;
        // min = min: other

        /* not implemented */
        // exp10: https://internals.rust-lang.org/t/enh-add-exp10-and-expf-base-x-f64-f32-methods-to-stdlib-to-symmetrize-api
        // WAIT: (gamma, ln_gamma) [float_gamma](https://github.com/rust-lang/rust/issues/99842)
    ]; // IMPORTANT: do not end with `;`

    /// $f:   the floating-point type.
    /// $e:   the integer type for integer exponentiation.
    macro_rules! custom_impls {
        () => {
            custom_impls![(f32, i32), (f64, i32)];
        };
        ($( ($f:ty, $e:ty)),+) => {
            $( custom_impls![@$f, $e]; )+
        };
        (@$f:ty, $e:ty) => {
            /// # *Implementations using the `std` feature*.
            impl Float<$f> {
                /// Raises itself to the `p` integer power.
                pub fn powi(self, p: $e) -> Float<$f> { Float(<$f>::powi(self.0, p)) }
                /// Both the sine and cosine.
                pub fn sin_cos(self) -> (Float<$f>, Float<$f>) {
                    let (sin, cos) = <$f>::sin_cos(self.0);
                    (Float(sin), Float(cos))
                }
                /// The integral part of `x`.
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_TRUNC!()]
                pub const fn trunc(self) -> Float<$f> {
                    Float(self.0.trunc())
                }
                /// The fractional part of `x`.
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_FRACT!()]
                pub const fn fract(self) -> Float<$f> {
                    Float(self.0.fract())
                }
                /// The integral and fractional parts of `x`.
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_SPLIT!()]
                pub const fn split(self) -> (Float<$f>, Float<$f>) {
                    let trunc = self.trunc();
                    (trunc, Float(self.0 - trunc.0))
                }
            }
        };
    }
    custom_impls!();
}

#[cfg(not(feature = "std"))]
mod _no_std {
    use super::Float;

    /// $f:   the floating-point type.
    /// $uf:  unsigned int type with the same bit-size.
    /// $ie:  the integer type for integer exponentiation.
    macro_rules! custom_impls {
        () => {
            custom_impls![(f32, u32, i32), (f64, u64, i32)];
        };
        ($( ($f:ty, $uf:ty, $ie:ty)),+) => {
            $( custom_impls![@$f, $uf, $ie]; )+
        };
        (@$f:ty, $uf:ty, $ie:ty) => {
            /// # *Implementations without `std`
            impl Float<$f> {
                /// The largest integer less than or equal to itself.
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_FLOOR!()]
                pub const fn floor(self) -> Float<$f> { self.const_floor() }

                /// The smallest integer greater than or equal to itself.
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_CEIL!()]
                pub const fn ceil(self) -> Float<$f> { self.const_ceil() }

                /// The nearest integer to itself, default rounding
                ///
                /// This is the default [`round_ties_away`] implementation.
                pub const fn round(self) -> Float<$f> { self.const_round() }

                /// The nearest integer to itself, rounding ties away from `0.0`.
                ///
                /// This is the default [`round`] implementation.
                ///
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_ROUND_TIES_AWAY!()]
                pub const fn round_ties_away(self) -> Float<$f> {self.const_round_ties_away() }

                /// Returns the nearest integer to `x`, rounding ties to the nearest even integer.
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_ROUND_TIES_EVEN!()]
                pub const fn round_ties_even(self) -> Float<$f> { self.const_round_ties_even() }

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
                pub const fn trunc(self) -> Float<$f> { self.const_trunc() }

                /// The fractional part.
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_FRACT!()]
                pub const fn fract(self) -> Float<$f> { self.const_fract() }

                /// The integral and fractional parts.
                /// # Formulation
                #[doc = crate::_FLOAT_FORMULA_SPLIT!()]
                pub const fn split(self) -> (Float<$f>, Float<$f>) { self.const_split() }

                /// Raises itself to the `p` integer power.
                pub const fn powi(self, p: $ie) -> Float<$f> { self.const_powi(p) }
            }
        };
    }
    custom_impls!();
}
