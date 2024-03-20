// devela::num::int::wrapper::impl_modulo
//
//! implements modulo related functions
//
// TOC
// - signed|unsigned:
//   - modulo (uc)
//   - modulo_fits (uc)
//   - modulo_add (uc)
//   - modulo_add_fits (uc)
//   - modulo_sub (uc)

use crate::{
    code::{cif, paste},
    num::{isize_up, usize_up, Int, NumError, NumResult as Result},
};
use NumError::{NonZeroRequired, Overflow};

// helper function to be called from the cold path branch when modulus == 0.
#[cold] #[inline(never)] #[rustfmt::skip]
const fn cold_err_zero<T>() -> Result<T> { Err(NonZeroRequired) }
// helper function to be called from the cold path branch for rare i128 overflow.
#[cold] #[inline(never)] #[rustfmt::skip]
const fn cold_err_overflow<T>() -> Result<T> { Err(Overflow(None)) }

// helper macro to deal with the case when we can't upcast (i.e. for 128-bits).
//
// $op:  an overloadable operator (+, -, *, /)
// $fn:  the corresponding function (add, sub, mul, div)
// $lhs: the left hand side operator
// $rhs: the right hand side operator
// $is_up: whether we've upcasted (Y) or not (N), known at compile-time
//
// WAIT: [unchecked_add|mul](https://github.com/rust-lang/rust/issues/85122)
//   [Stabilize unchecked_{add,sub,mul}](https://github.com/rust-lang/rust/pull/122520)
//   let Some(x) = x.checked_add(y) else { unsafe { hint::unreachable_unchecked() }};
#[rustfmt::skip]
macro_rules! upcastop {
    // this is used for checked versions
    (err $op:tt $fn:ident($lhs:expr, $rhs:expr) $is_up:ident) => { paste! {
        if cif!(same($is_up, Y)) { // can't overflow if upcasted
            $lhs $op $rhs
        } else { // otherwise do the checked operation:
            if let Some(result) = $lhs.[<checked_ $fn>]($rhs) {
                result } else { return Err(Overflow(None));
            }
        }
    }};
    // this is used for checked versions that don't need to calculate times
    (reduce_err $op:tt $fn:ident($lhs:expr, $rhs:expr) % $modulus:expr, $is_up:ident) => { paste! {
        if cif!(same($is_up, Y)) { // can't overflow if upcasted
            $lhs $op $rhs
        } else { // otherwise reduce each sumand before the checked operation:
            if let Some(result) = ($lhs % $modulus).[<checked_ $fn>]($rhs % $modulus) {
                result } else { return Err(Overflow(None));
            }
        }
    }};
    // this is used for unchecked versions that don't need to calculate times
    (reduce $op:tt $fn:ident($lhs:expr, $rhs:expr) % $modulus:expr, $is_up:ident) => { paste! {
        if cif!(same($is_up, Y)) { // can't overflow if upcasted
            $lhs $op $rhs
        } else { // otherwise reduce each sumand before the unchecked operation:
            ($lhs % $modulus) $op ($rhs % $modulus)
        }
    }};
}

// $t:     the input/output type
// $up:    the upcasted type to do the operations on (the ones that can overflow) (not used)
// $is_up: information about the upcasted type [Y|N] in size
macro_rules! impl_int {
    (signed $( ($t:ty, $up:ty:$is_up:ident, $d:literal) ),+) => {
        $( impl_int![@signed ($t, $up:$is_up, $d)]; )+ };
    (unsigned $( ($t:ty, $up:ty:$is_up:ident, $d:literal) ),+) => {
        $( impl_int![@unsigned ($t, $up:$is_up, $d)]; )+ };

    // implements signed ops
    (@signed ($t:ty, $up:ty:$is_up:ident, $d:literal) ) => { paste! {

        #[doc = "# Integer modulo related methods for `" $t "`\n\n"]
        #[doc = "- [modulo](#method.modulo" $d
            ") *([uc](#method.modulo_unchecked" $d ")*)"]
        #[doc = "- [modulo_fits](#method.modulo_fits" $d
            ") *([uc](#method.modulo_fits_unchecked" $d "))*"]
        #[doc = "- [modulo_add](#method.modulo_add" $d
            ") *([uc](#method.modulo_add_unchecked" $d "))*"]
        #[doc = "- [modulo_add_fits](#method.modulo_add" $d
            ") *([uc](#method.modulo_add_fits_unchecked" $d "))*"]
        #[doc = "- [modulo_sub](#method.modulo_sub" $d
            ") *([uc](#method.modulo_sub_unchecked" $d "))*"]
        impl Int<$t> {
            /* modulo (signed) */

            /// Computes the non-negative modulo of `self` over |`modulus`|.
            ///
            /// The result is non-negative and less than the absolute value of `modulus`,
            /// i.e., in the range $ [0, |\text{modulus}|) $.
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `i128`
            /// it could also return [`Overflow`].
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(-4_" $t ").modulo(m)?, 2];"]
            #[doc = "assert_eq![Int(-3_" $t ").modulo(m)?, 0];"]
            #[doc = "assert_eq![Int(-2_" $t ").modulo(m)?, 1];"]
            #[doc = "assert_eq![Int(-1_" $t ").modulo(m)?, 2];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo(m)?, 0];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo(m)?, 1];"]
            #[doc = "assert_eq![Int(2_" $t ").modulo(m)?, 2];"]
            #[doc = "assert_eq![Int(3_" $t ").modulo(m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo(m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo(-m)?, 1];"]
            ///
            #[doc = "assert_eq![Int(" $t "::MAX).modulo(" $t "::MIN)?, " $t "::MAX];"]
            #[doc = "assert_eq![Int(" $t "::MIN).modulo(" $t "::MAX)?, " $t "::MAX - 1];"]
            #[doc = "assert_eq![Int(" $t "::MIN).modulo(" $t "::MIN)?, 0];"]
            #[doc = "assert![Int(i64::MIN).modulo(-1).is_ok()];"]
            ///
            #[doc = "assert_eq![Int(1_" $t ").modulo(0), Err(NumError::NonZeroRequired)];"]
            #[doc = "assert_eq![Int(i128::MIN).modulo(-1), Err(NumError::Overflow(None))];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo(self, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (v, m) = (self.0 as $up, modulus as $up);
                    if let Some(v) = v.checked_rem_euclid(m) {
                        Ok(Int(v as $t))
                    } else {
                        cold_err_overflow()
                    }
                }
            }

            /// Computes the non-negative modulo of `self` over |`modulus`|, unchecked version.
            ///
            /// The result is non-negative and less than the absolute value of `modulus`,
            /// i.e., in the range $ [0, |\text{modulus}|) $.
            #[doc = "It upcasts internally to [`" $up "`] for the inner operation."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it could also panic on overflow.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let m = 3;
            #[doc = "assert_eq![Int(-4_" $t ").modulo_unchecked(m), 2];"]
            #[doc = "assert_eq![Int(-3_" $t ").modulo_unchecked(m), 0];"]
            #[doc = "assert_eq![Int(-2_" $t ").modulo_unchecked(m), 1];"]
            #[doc = "assert_eq![Int(-1_" $t ").modulo_unchecked(m), 2];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_unchecked(m), 0];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo_unchecked(m), 1];"]
            #[doc = "assert_eq![Int(2_" $t ").modulo_unchecked(m), 2];"]
            #[doc = "assert_eq![Int(3_" $t ").modulo_unchecked(m), 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_unchecked(m), 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_unchecked(-m), 1];"]
            ///
            #[doc = "assert_eq![Int(" $t "::MAX).modulo_unchecked(" $t "::MAX - 1), 1];"]
            #[doc = "assert_eq![Int(" $t "::MAX).modulo_unchecked(" $t "::MAX), 0];"]
            #[doc = "assert_eq![Int(" $t "::MAX).modulo_unchecked(" $t "::MIN), " $t "::MAX];"]
            #[doc = "assert_eq![Int(" $t "::MIN).modulo_unchecked(" $t "::MAX), " $t "::MAX - 1];"]
            #[doc = "assert_eq![Int(" $t "::MIN).modulo_unchecked(" $t "::MIN), 0];"]
            ///
            #[doc = "assert_eq![Int(i64::MIN).modulo_unchecked(-1), 0];"]
            /// ```
            /// ```should_panic
            /// # use devela::num::Int;
            #[doc = "let _ = Int(i128::MIN).modulo_unchecked(-1); // i128 could overflow"]
            /// ```
            /// ```should_panic
            /// # use devela::num::Int;
            #[doc = "let _ = Int(1_" $t ").modulo_unchecked(0); // panics if modulus == 0"]
            /// ```
            #[inline]
            pub const fn modulo_unchecked(self, modulus: $t) -> Int<$t> {
                let (v, m) = (self.0 as $up, modulus as $up);
                Int(v.rem_euclid(m) as $t)
            }

            /* modulo fits (signed) */

            /// Computes the non-negative modulo of `self` over |`modulus`|,
            /// and the |times| it fits.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `i128`
            /// if `self == MIN && modulus == ±1` it can return [`Overflow`].
            #[inline]
            pub const fn modulo_fits(self, modulus: $t) -> Result<(Int<$t>, Int<$t>)> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (orig, m) = (self.0 as $up, modulus as $up);
                    if let Some(v) = orig.checked_rem_euclid(m) {
                        let modulo = Int(v as $t);
                        let times = Int(((orig / m) as $t).abs());
                        Ok((modulo, times))
                    } else {
                        cold_err_overflow()
                    }
                }
            }

            /// Computes the non-negative modulo of `self` over |`modulus`|,
            /// and the |times| it fits, unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it can also panic
            /// if `self == MIN && modulus == ±1`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let m = 3;
            #[doc = "let (modulo, times) = Int(-3_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 0]; assert_eq![times, 1];
            #[doc = "let (modulo, times) = Int(-2_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 1]; assert_eq![times, 0];
            #[doc = "let (modulo, times) = Int(-1_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 2]; assert_eq![times, 0];
            #[doc = "let (modulo, times) = Int(0_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 0]; assert_eq![times, 0];
            #[doc = "let (modulo, times) = Int(1_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 1]; assert_eq![times, 0];
            #[doc = "let (modulo, times) = Int(2_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 2]; assert_eq![times, 0];
            #[doc = "let (modulo, times) = Int(3_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 0]; assert_eq![times, 1];
            ///
            #[doc = "let (modulo, times) = Int(10_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 1]; assert_eq![times, 3];
            /// ```
            #[must_use] #[inline]
            pub const fn modulo_fits_unchecked(self, modulus: $t) -> (Int<$t>, Int<$t>) {
                let (v, m) = (self.0 as $up, modulus as $up);
                let modulo = Int(v.rem_euclid(m) as $t);
                let times = Int(((v / m) as $t).abs());
                (modulo, times)
            }

            /* modulo add (signed) */

            /// Computes the non-negative modulo of `self + other` over |`modulus`|.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `i128`
            /// it could also return [`Overflow`].
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(-2_" $t ").modulo_add(-1, m)?, 0];"]
            #[doc = "assert_eq![Int(-1_" $t ").modulo_add(-1, m)?, 1];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(-1, m)?, 2];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(0, m)?, 0];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(1, m)?, 1];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(2, m)?, 2];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(3, m)?, 0];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo_add(1, m)?, 2];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_add(self, other: $t, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    let sum = upcastop![reduce_err +add(a, b) % m, $is_up];
                    if let Some(v) = sum.checked_rem_euclid(m) { // TODO:TEST
                        Ok(Int(v as $t))
                    } else {
                        cold_err_overflow()
                    }
                }
            }

            /// Computes the non-negative modulo of `self + other` over |`modulus`|,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it could also panic on overflow.
            #[inline]
            pub const fn modulo_add_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                let sum = upcastop![reduce +add(a, b) % m, $is_up];
                Int(sum.rem_euclid(m) as $t)
            }

            /* modulo add fits (signed) */

            /// Computes the non-negative modulo of `self + other` over |`modulus`|,
            /// and the *times* it fits in the sum.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `i128`
            /// it can also return [`Overflow`], more probably than in
            /// [`modulo_add`][Self::modulo_add] since we can't reduce
            /// the operands beforehand in order to calculate *times*.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 4;
            #[doc = "let (modulo, times) = Int(-10_" $t ").modulo_add_fits(15, m)?;"]
            /// assert_eq!(modulo, 1); assert_eq!(times, 1);
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_add_fits(self, other: $t, modulus: $t)
                -> Result<(Int<$t>, Int<$t>)> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    // not reducing for i128 makes overflow more likely,
                    // but we can't if we want to calculate `times`.
                    let sum = upcastop![err +add(a, b) $is_up];
                    if let Some(v) = sum.checked_rem_euclid(m) {
                        let modulo = Int(v as $t);
                        let times = Int(((sum / m) as $t).abs());
                        Ok((modulo, times))
                    } else {
                        cold_err_overflow()
                    }
                }
            }

            /// Computes the non-negative modulo of `self + other` over |`modulus`|,
            /// and the times it fits in the sum.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it can also panic on overflow,
            /// more probably than in [`modulo_add_unchecked`][Self::modulo_add_unchecked]
            /// since we can't reduce the operands beforehand in order to calculate *times*.
            #[must_use] #[inline]
            pub const fn modulo_add_fits_unchecked(self, other: $t, modulus: $t)
                -> (Int<$t>, Int<$t>) {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                // not reducing for i128 makes overflow more likely,
                // but we can't if we want to calculate `times`.
                let sum = a + b;
                let modulo = sum.rem_euclid(m) as $t;
                let times = ((sum / m) as $t).abs();
                (Int(modulo), Int(times))
            }

            /* modulo sub (signed) */

            /// Computes the modulo of `self - other` over `modulus`.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(-4, m)?, 2];"]
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(-3, m)?, 1];"]
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(-2, m)?, 0];"]
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(-1, m)?, 2];"]
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(0, m)?, 1];"]
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(1, m)?, 0];"]
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(2, m)?, 2];"]
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(3, m)?, 1];"]
            #[doc = "assert_eq![Int(-5_" $t ").modulo_sub(4, m)?, 0];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_sub(self, other: $t, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    let res = upcastop![reduce_err -sub(a, b) % m, $is_up];
                    Ok(Int(res.rem_euclid(m) as $t))
                }
            }

            /// Computes the modulo of `self - other` over `modulus`,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`] for the inner operations."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`.
            #[inline]
            pub const fn modulo_sub_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                let res = upcastop![reduce -sub(a, b) % m, $is_up];
                Int(res.rem_euclid(m) as $t)
            }
        }
    }};

    // implements unsigned ops
    // NOTE: upcasting is not used for unsigned operations
    (@unsigned ($t:ty, $up:ty:$is_up:ident, $d:literal) ) => { paste! {
        #[doc = "# Integer modulo related methods for `" $t "`\n\n"]
        #[doc = "- [modulo](#method.modulo" $d
            ") *([uc](#method.modulo_unchecked" $d ")*)"]
        #[doc = "- [modulo_fits](#method.modulo_fits" $d
            ") *([uc](#method.modulo_fits_unchecked" $d "))*"]
        #[doc = "- [modulo_add](#method.modulo_add" $d
            ") *([uc](#method.modulo_add_unchecked" $d "))*"]
        #[doc = "- [modulo_add_fits](#method.modulo_add" $d
            ") *([uc](#method.modulo_add_fits_unchecked" $d "))*"]
        #[doc = "- [modulo_sub](#method.modulo_sub" $d
            ") *([uc](#method.modulo_sub_unchecked" $d "))*"]
        impl Int<$t> {
            /* modulo (unsigned) */

            /// Computes the non-negative modulo of `self` over `modulus`.
            ///
            /// The result is less than the value of `modulus`,
            /// i.e., in the range $ [0, \text{modulus}) $.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(0_" $t ").modulo(m)?, 0];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo(m)?, 1];"]
            #[doc = "assert_eq![Int(2_" $t ").modulo(m)?, 2];"]
            #[doc = "assert_eq![Int(3_" $t ").modulo(m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo(m)?, 1];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo(self, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    Ok(Int(self.0 % modulus))
                }
            }

            /// Computes the non-negative modulo of `self` over `modulus`, unchecked version.
            ///
            /// The result is less than the value of `modulus`,
            /// i.e., in the range $ [0, \text{modulus}) $.
            ///
            /// # Panics
            /// Panics if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let m = 3;
            #[doc = "assert_eq![Int(0_" $t ").modulo_unchecked(m), 0];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo_unchecked(m), 1];"]
            #[doc = "assert_eq![Int(2_" $t ").modulo_unchecked(m), 2];"]
            #[doc = "assert_eq![Int(3_" $t ").modulo_unchecked(m), 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_unchecked(m), 1];"]
            /// ```
            #[inline]
            pub const fn modulo_unchecked(self, modulus: $t) -> Int<$t> {
                Int(self.0 % modulus)
            }

            /* modulo fits (unsigned) */

            /// Computes the non-negative modulo of `self` over `modulus`,
            /// and the times it fits.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`.
            #[inline]
            pub const fn modulo_fits(self, modulus: $t) -> Result<(Int<$t>, Int<$t>)> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    Ok((Int(self.0 % modulus), Int(self.0 / modulus)))
                }
            }

            /// Computes the non-negative modulo of `self` over `modulus`,
            /// and the times it fits, unchecked version.
            ///
            /// # Panics
            /// Panics if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let m = 3;
            #[doc = "let (modulo, times) = Int(0_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 0]; assert_eq![times, 0];
            #[doc = "let (modulo, times) = Int(1_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 1]; assert_eq![times, 0];
            #[doc = "let (modulo, times) = Int(2_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 2]; assert_eq![times, 0];
            #[doc = "let (modulo, times) = Int(3_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 0]; assert_eq![times, 1];
            ///
            #[doc = "let (modulo, times) = Int(10_" $t ").modulo_fits_unchecked(m);"]
            /// assert_eq![modulo, 1]; assert_eq![times, 3];
            /// ```
            #[must_use] #[inline]
            pub const fn modulo_fits_unchecked(self, modulus: $t) -> (Int<$t>, Int<$t>) {
                (Int(self.0 % modulus), Int(self.0 / modulus))
            }

            /* modulo add (unsigned) */

            /// Computes the modulo of `self + other` over `modulus`.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `u128`
            /// it could also return [`Overflow`].
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(0, m)?, 0];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(1, m)?, 1];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(2, m)?, 2];"]
            #[doc = "assert_eq![Int(0_" $t ").modulo_add(3, m)?, 0];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo_add(1, m)?, 2];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_add(self, other: $t, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    let sum = upcastop![reduce_err +add(a, b) % m, $is_up];
                    Ok((Int((sum % m) as $t)))
                }
            }

            /// Computes the modulo of `self + other` over `modulus`,
            /// unchecked version.
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `u128` it could also panic on overflow.
            #[inline]
            pub const fn modulo_add_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                let sum = upcastop![reduce +add(a, b) % m, $is_up];
                Int((sum % m) as $t)
            }

            /* modulo add fits (unsigned) */

            /// Computes the modulo of `self + other` over `modulus`,
            /// and the *times* it fits in the sum.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `u128`
            /// it can also return [`Overflow`], more probably than in
            /// [`modulo_add`][Self::modulo_add] since we can't reduce
            /// the operands beforehand in order to calculate *times*.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 4;
            #[doc = "let (modulo, times) = Int(10_" $t ").modulo_add_fits(15, m)?;"]
            /// assert_eq!(modulo, 1); assert_eq!(times, 6);
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_add_fits(self, other: $t, modulus: $t)
                -> Result<(Int<$t>, Int<$t>)> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    // not reducing for u128 makes overflow more likely,
                    // but we can't if we want to calculate `times`.
                    let sum = upcastop![err +add(a, b) $is_up];
                    let modulo = Int((sum % m) as $t);
                    let times = Int((sum / m) as $t);
                    Ok((modulo, times))
                }
            }

            /// Computes the modulo of `self + other` over `modulus`,
            /// and the times it fits in the sum.
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `u128` it can also panic on overflow,
            /// more probably than in [`modulo_add_unchecked`][Self::modulo_add_unchecked]
            /// since we can't reduce the operands beforehand in order to calculate *times*.
            #[must_use] #[inline]
            pub const fn modulo_add_fits_unchecked(self, other: $t, modulus: $t)
                -> (Int<$t>, Int<$t>) {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                // not reducing for u128 makes overflow more likely,
                // but we can't if we want to calculate `times`.
                let sum = a + b;
                let modulo = Int((sum % m) as $t);
                let times = Int((sum / m) as $t);
                ((modulo, times))
            }

            /* modulo sub (unsigned) */

            /// Computes the modulo of `self - other` over `modulus`.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`,
            /// and it could also return [`Overflow`].
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(5_" $t ").modulo_sub(0, m)?, 2];"]
            #[doc = "assert_eq![Int(5_" $t ").modulo_sub(1, m)?, 1];"]
            #[doc = "assert_eq![Int(5_" $t ").modulo_sub(2, m)?, 0];"]
            #[doc = "assert_eq![Int(5_" $t ").modulo_sub(3, m)?, 2];"]
            #[doc = "assert_eq![Int(5_" $t ").modulo_sub(4, m)?, 1];"]
            ///
            #[doc = "assert_eq![Int(0_" $t ").modulo_sub(1, m), Err(NumError::Overflow(None))];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_sub(self, other: $t, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    if let Some(res) = self.0.checked_sub(other) {
                        Ok(Int(res % modulus))
                    } else {
                        Err(Overflow(None))
                    }
                }
            }

            /// Computes the modulo of `self - other` over `modulus`,
            /// unchecked version.
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and on overflow.
            #[inline]
            pub const fn modulo_sub_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                Int(((self.0 - other) % modulus))
            }
        }
    }};
}
impl_int![signed
    (i8, i16:Y, ""), (i16, i32:Y, "-1"), (i32, i64:Y, "-2"),
    (i64, i128:Y, "-3"), (i128, i128:N, "-4"), (isize, isize_up:Y, "-5")
];
impl_int![unsigned
    (u8, u16:Y, "-6"), (u16, u32:Y, "-7"), (u32, u64:Y, "-8"),
    (u64, u128:Y, "-9"), (u128, u128:N, "-10"), (usize, usize_up:Y, "-11")
];
