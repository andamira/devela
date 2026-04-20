// devela::num::dom::real::float::wrapper_std::std
//
//! Methods depending on std, or its absence
//
// TOC
// - macro _num_dom_real_float_impl_std!
// - impls for std

use crate::Float;

/// Macro helper for implementing methods for `Float`, from `std`.
///
/// $lib: the library to use.
/// $f: the floating-point type to support.
/// $doc: an optional documentation string.
/// $opfn: the original operation function name.
/// $op: the new operation function name in Float.
macro_rules! _num_dom_real_float_impl_std {
    (
        // Matches a wildcard floating-point type (f*).
        // Expands to specific floating-point types (f32, f64).
        $lib:ident : f* : $($rest:tt)*
    ) => {
        _num_dom_real_float_impl_std![$lib : f32 : $($rest)*];
        _num_dom_real_float_impl_std![$lib : f64 : $($rest)*];
    };
    (
        // Matches a specific floating-point type and any number of operations.
        // Generates the impl block for Float<$f> and calls the matching implementation.
        $lib:ident : $f:ty : $($rest:tt)*
    ) => { $crate::paste! {
        impl Float<$f> {
            _num_dom_real_float_impl_std![@$lib : $f : $($rest)*];
        }
    }};
    (
        // Matches multiple operations and uses recursion to process each one.
        @$lib:ident : $f:ty : $($doc:literal)? $(+const$($_c:lifetime)?)?
        $opfn:ident = $op:ident : $($arg:ident),*
        ; $($rest:tt)*
    ) => {
        _num_dom_real_float_impl_std![@$lib : $f :
            $($doc)? $(+const$($_c)?)? $opfn = $op : $($arg),*];
        _num_dom_real_float_impl_std![@$lib : $f : $($rest)*];
    };
    (
        // Matches a single operation and implements it using the `std` library.
        @std : $f:ty : $($doc:literal)? $(+const$($_c:lifetime)?)?
        $opfn:ident = $op:ident : $($arg:ident),* $(;)?
    ) => {
        $(#[doc = $doc])?
        pub $(const$($_c)?)? fn $op(self, $($arg: $f),*) -> Float<$f> {
            Float(<$f>::$opfn(self.0, $($arg),*))
        }
    };
}

mod _std {
    use super::Float;

    // custom implementations are commented out:
    _num_dom_real_float_impl_std![std:f*:
        r"The largest integer less than or equal to `x`.
        $$ \lfloor x \rfloor = \max \{ n \in ℤ \,|\, n \leq x \} $$ "
        +const floor = floor: ;
        r"The smallest integer greater than or equal to `x`.
        $$ \lceil x \rceil = \min \{ n \in ℤ \,|\, n \geq x \} $$"
        +const ceil = ceil: ;
        "The nearest integer to `x`, rounding ties away from `0.0`."
        +const round = round_ties_away: ;
        "The nearest integer to `x`, rounding ties to the nearest even integer."
        +const round_ties_even = round_ties_even: ;
        // trunc = trunc: ;
        // fract = fract: ;
        // split == modf
        // abs
        // signum = signum: ;
        // copysign = copysign: sign;
        "Fused multiply-add. Computes (self * mul) + add with only one rounding error."
        +const mul_add = mul_add: mul, add;
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
    macro_rules! _impl_float_std_custom {
        () => {
            _impl_float_std_custom![(f32, i32), (f64, i32)];
        };
        ($( ($f:ty, $e:ty)),+) => {
            $( _impl_float_std_custom![@$f, $e]; )+
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
                /// The nearest integer to itself, default rounding
                ///
                /// This is equivalent to [`round_ties_away`][Self::round_ties_away].
                pub const fn round(self) -> Float<$f> { Float(self.0.round()) }

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
    _impl_float_std_custom!();
}
