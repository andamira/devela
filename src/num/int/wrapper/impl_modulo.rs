// devela::num::int::wrapper::impl_modulo
//
//! implements modulo related functions
//
// TOC
// - signed|unsigned:
//   - modulo (uc)
//   - modulo_cycles (uc)
//   - modulo_add (uc)
//   - modulo_add_cycles (uc)
//   - modulo_add_inv (uc)
//   - modulo_sub (uc)
//   - modulo_sub_cycles (uc)
//   - modulo_mul (uc)
//   - modulo_mul_cycles (uc)
//   - modulo_mul_inv (uc)

use crate::{
    code::{cif, iif, paste},
    num::{isize_up, usize_up, Int, NumError, NumResult as Result},
    result::{unwrap, ValueQuant},
};
use NumError::{NoInverse, NonZeroRequired, Overflow};

// helper function to be called from the cold path branch when modulus == 0.
#[cold] #[inline(never)] #[rustfmt::skip] #[allow(dead_code)]
const fn cold_err_zero<T>() -> Result<T> { Err(NonZeroRequired) }
// helper function to be called from the cold path branch for rare i128 overflow.
#[cold] #[inline(never)] #[rustfmt::skip] #[allow(dead_code)]
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
#[rustfmt::skip] #[allow(unused_macros)]
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
// $cap:   the capability feature that enables the given implementation. E.g u8.
// $up:    the upcasted type to do the operations on (the ones that can overflow) E.g. u16.
// $iup:   the signed upcasted type for some methods. E.g. i16.
// $icap:  the feature that enables some methods related to `$iup`. E.g "i16".
// $is_up: [Y|N]. `Y` if bitsize of $up|$iup > $t; `N` if bitsize $up|$iup == $t.
macro_rules! impl_int {
    (signed $( ($t:ty : $cap:literal, $up:ty:$is_up:ident, $d:literal) ),+) => {
        $( impl_int![@signed ($t:$cap, $up:$is_up, $d)]; )+
    };
    (unsigned $(
        ($t:ty : $cap:literal, $up:ty | $iup:ty : $icap:literal : $is_up:ident, $d:literal)
    ),+ ) => {
        $( impl_int![@unsigned ($t:$cap, $up|$iup:$icap :$is_up, $d)]; )+
    };

    // implements signed ops
    (@signed ($t:ty : $cap:literal, $up:ty:$is_up:ident, $d:literal) ) => { paste! {

        #[doc = "# Integer modulo related methods for `" $t "`\n\n"]
        #[doc = "- [modulo](#method.modulo" $d
            ") *([uc](#method.modulo_unchecked" $d ")*)"]
        #[doc = "- [modulo_cycles](#method.modulo_cycles" $d
            ") *([uc](#method.modulo_cycles_unchecked" $d "))*"]
        //
        #[doc = "- [modulo_add](#method.modulo_add" $d
            ") *([uc](#method.modulo_add_unchecked" $d "))*"]
        #[doc = "- [modulo_add_cycles](#method.modulo_add_cycles" $d
            ") *([uc](#method.modulo_add_cycles_unchecked" $d "))*"]
        #[doc = "- [modulo_add_inv](#method.modulo_add_inv" $d
            ") *([uc](#method.modulo_add_inv_unchecked" $d "))*"]
        //
        #[doc = "- [modulo_sub](#method.modulo_sub" $d
            ") *([uc](#method.modulo_sub_unchecked" $d "))*"]
        #[doc = "- [modulo_sub_cycles](#method.modulo_add_cycles" $d
            ") *([uc](#method.modulo_sub_cycles_unchecked" $d "))*"]
        //
        #[doc = "- [modulo_mul](#method.modulo_mul" $d
            ") *([uc](#method.modulo_mul_unchecked" $d "))*"]
        #[doc = "- [modulo_mul_cycles](#method.modulo_mul_cycles" $d
            ") *([uc](#method.modulo_mul_cycles_unchecked" $d "))*"]
        #[doc = "- [modulo_mul_inv](#method.modulo_mul_inv" $d
            ") *([uc](#method.modulo_mul_inv_unchecked" $d "))*"]
        ///
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Int<$t> {
            /* modulo (signed) */

            /// Computes the non-negative modulo of `self` over |`modulus`|.
            ///
            /// The result is non-negative and less than the absolute value of `modulus`,
            /// i.e., in the range $ [0, |\text{modulus}|) $.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
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
            #[doc = "assert_eq![Int( 0_" $t ").modulo(m)?, 0];"]
            #[doc = "assert_eq![Int( 1_" $t ").modulo(m)?, 1];"]
            #[doc = "assert_eq![Int( 2_" $t ").modulo(m)?, 2];"]
            #[doc = "assert_eq![Int( 3_" $t ").modulo(m)?, 0];"]
            #[doc = "assert_eq![Int( 4_" $t ").modulo(m)?, 1];"]
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
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
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
            #[doc = "assert_eq![Int( 0_" $t ").modulo_unchecked(m), 0];"]
            #[doc = "assert_eq![Int( 1_" $t ").modulo_unchecked(m), 1];"]
            #[doc = "assert_eq![Int( 2_" $t ").modulo_unchecked(m), 2];"]
            #[doc = "assert_eq![Int( 3_" $t ").modulo_unchecked(m), 0];"]
            #[doc = "assert_eq![Int( 4_" $t ").modulo_unchecked(m), 1];"]
            #[doc = "assert_eq![Int( 4_" $t ").modulo_unchecked(-m), 1];"]
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

            /* modulo cycles (signed) */

            /// Computes the non-negative modulo of `self` over |`modulus`|,
            /// and the number of cycles the result is reduced.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `i128`
            /// if `self == MIN && modulus == ±1` it can return [`Overflow`].
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(-3_" $t ").modulo_cycles(m)?, (0, 1)];"]
            #[doc = "assert_eq![Int(-2_" $t ").modulo_cycles(m)?, (1, 0)];"]
            #[doc = "assert_eq![Int(-1_" $t ").modulo_cycles(m)?, (2, 0)];"]
            #[doc = "assert_eq![Int( 0_" $t ").modulo_cycles(m)?, (0, 0)];"]
            #[doc = "assert_eq![Int( 1_" $t ").modulo_cycles(m)?, (1, 0)];"]
            #[doc = "assert_eq![Int( 2_" $t ").modulo_cycles(m)?, (2, 0)];"]
            #[doc = "assert_eq![Int( 3_" $t ").modulo_cycles(m)?, (0, 1)];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_cycles(self, modulus: $t) -> Result<ValueQuant<Int<$t>, Int<$t>>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (orig, m) = (self.0 as $up, modulus as $up);
                    if let Some(v) = orig.checked_rem_euclid(m) {
                        let modulo = Int(v as $t);
                        let times = Int(((orig / m) as $t).abs());
                        Ok(ValueQuant::new(modulo, times))
                    } else {
                        cold_err_overflow()
                    }
                }
            }

            /// Computes the non-negative modulo of `self` over |`modulus`|,
            /// and the number of cycles the result is reduced,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it can also panic
            /// if `self == MIN && modulus == ±1`.
            #[inline]
            pub const fn modulo_cycles_unchecked(self, modulus: $t) -> ValueQuant<Int<$t>, Int<$t>> {
                let (v, m) = (self.0 as $up, modulus as $up);
                let modulo = Int(v.rem_euclid(m) as $t);
                let times = Int(((v / m) as $t).abs());
                ValueQuant::new(modulo, times)
            }

            /* modulo add (signed) */

            /// Computes the non-negative modulo of `self + other` over |`modulus`|.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
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
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(-4, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(-3, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(-2, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(-1, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add( 0, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add( 1, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add( 2, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add( 3, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add( 4, m)?, 2];"]
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
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it could also panic on overflow.
            #[inline]
            pub const fn modulo_add_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                let sum = upcastop![reduce +add(a, b) % m, $is_up];
                Int(sum.rem_euclid(m) as $t)
            }

            /* modulo add cycles (signed) */

            /// Computes the non-negative modulo of `self + other` over |`modulus`|,
            /// and the number of cycles the result is reduced.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
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
            /// let m = 3;
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(-4, m)?, (0, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(-3, m)?, (1, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(-2, m)?, (2, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(-1, m)?, (0, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles( 0, m)?, (1, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles( 1, m)?, (2, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles( 2, m)?, (0, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles( 3, m)?, (1, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles( 4, m)?, (2, 2)];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_add_cycles(self, other: $t, modulus: $t)
                -> Result<ValueQuant<Int<$t>, Int<$t>>> {
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
                        Ok(ValueQuant::new(modulo, times))
                    } else {
                        cold_err_overflow()
                    }
                }
            }

            /// Computes the non-negative modulo of `self + other` over |`modulus`|,
            /// and the number of cycles the result is reduced,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it can also panic on overflow,
            /// more probably than in [`modulo_add_unchecked`][Self::modulo_add_unchecked]
            /// since we can't reduce the operands beforehand in order to calculate *times*.
            #[inline]
            pub const fn modulo_add_cycles_unchecked(self, other: $t, modulus: $t)
                -> ValueQuant<Int<$t>, Int<$t>> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                // not reducing for i128 makes overflow more likely,
                // but we can't if we want to calculate `times`.
                let sum = a + b;
                let modulo = sum.rem_euclid(m) as $t;
                let times = ((sum / m) as $t).abs();
                ValueQuant::new(Int(modulo), Int(times))
            }

            /* modulo add inverse (signed) */

            /// Calculates the modular additive inverse.
            ///
            /// The modular additive inverse of *self* modulo *modulus*
            /// is an integer *b* such that $ a+b \equiv 0 (\mod m) $.
            ///
            /// The modular multiplicative inverse always exists and is simply
            /// `modulus - self` if `self != 0`, or 0 otherwise.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(-4_" $t ").modulo_add_inv(m)?, 1];"]
            #[doc = "assert_eq![Int(-3_" $t ").modulo_add_inv(m)?, 0];"]
            #[doc = "assert_eq![Int(-2_" $t ").modulo_add_inv(m)?, 2];"]
            #[doc = "assert_eq![Int(-1_" $t ").modulo_add_inv(m)?, 1];"]
            #[doc = "assert_eq![Int( 0_" $t ").modulo_add_inv(m)?, 0];"]
            #[doc = "assert_eq![Int( 1_" $t ").modulo_add_inv(m)?, 2];"]
            #[doc = "assert_eq![Int( 2_" $t ").modulo_add_inv(m)?, 1];"]
            #[doc = "assert_eq![Int( 3_" $t ").modulo_add_inv(m)?, 0];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_add_inv(self, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let rem = (self.0.rem_euclid(modulus));
                    iif![rem == 0; Ok(Int(0)); Ok(Int(modulus - rem))]
                }
            }

            /// Calculates the modular additive inverse,
            /// unchecked version.
            ///
            /// The modular additive inverse of *self* modulo *modulus*
            /// is an integer *b* such that $ a+b \equiv 0 (\mod m) $.
            ///
            /// The modular multiplicative inverse always exists and is simply
            /// `modulus - self` if `self != 0`, or 0 otherwise.
            ///
            /// # Panics
            /// Panics if `modulus == 0`.
            #[inline]
            pub const fn modulo_add_inv_unchecked(self, modulus: $t) -> Int<$t> {
                let rem = (self.0.rem_euclid(modulus));
                iif![rem == 0; Int(0); Int(modulus - rem)]
            }

            /* modulo sub (signed) */

            /// Computes the modulo of `self - other` over |`modulus`|.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(-4, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(-3, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(-2, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(-1, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub( 0, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub( 1, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub( 2, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub( 3, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub( 4, m)?, 0];"]
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

            /// Computes the modulo of `self - other` over |`modulus`|,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`.
            #[inline]
            pub const fn modulo_sub_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                let res = upcastop![reduce -sub(a, b) % m, $is_up];
                Int(res.rem_euclid(m) as $t)
            }

            /* modulo sub cycles (signed) */

            /// Computes the non-negative modulo of `self - other` over |`modulus`|,
            /// and the number of cycles the result is reduced.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `i128`
            /// it can also return [`Overflow`] (unlike [`modulo_sub`][Self::modulo_sub])
            /// since we can't reduce the operands beforehand in order to calculate *times*.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(-4, m)?, (2, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(-3, m)?, (1, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(-2, m)?, (0, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(-1, m)?, (2, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles( 0, m)?, (1, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles( 1, m)?, (0, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles( 2, m)?, (2, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles( 3, m)?, (1, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles( 4, m)?, (0, 0)];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_sub_cycles(self, other: $t, modulus: $t)
                -> Result<ValueQuant<Int<$t>, Int<$t>>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    // not reducing for i128 makes overflow more likely,
                    // but we can't if we want to calculate `times`.
                    let res = upcastop![err -sub(a, b) $is_up];
                    let modulo = Int(res.rem_euclid(m) as $t);
                    let times = Int(((res / m) as $t).abs());
                    Ok(ValueQuant::new(modulo, times))
                }
            }

            /// Computes the non-negative modulo of `self - other` over |`modulus`|,
            /// and the number of cycles the result is reduced,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it can also panic on overflow,
            /// more probably than in [`modulo_sub_unchecked`][Self::modulo_sub_unchecked]
            /// since we can't reduce the operands beforehand in order to calculate *times*.
            #[must_use] #[inline]
            pub const fn modulo_sub_cycles_unchecked(self, other: $t, modulus: $t)
                -> (Int<$t>, Int<$t>) {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                // not reducing for i128 makes overflow more likely,
                // but we can't if we want to calculate `times`.
                let res = a - b;
                let modulo = res.rem_euclid(m) as $t;
                let times = ((res / m) as $t).abs();
                (Int(modulo), Int(times))
            }

            /* modulo mul (signed) */

            /// Computes the non-negative modulo of `self + other` over |`modulus`|.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
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
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(-4, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(-3, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(-2, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(-1, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul( 0, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul( 1, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul( 2, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul( 3, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul( 4, m)?, 1];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_mul(self, other: $t, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    let sum = upcastop![reduce_err *mul(a, b) % m, $is_up];
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
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it could also panic on overflow.
            #[inline]
            pub const fn modulo_mul_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                let sum = upcastop![reduce *mul(a, b) % m, $is_up];
                Int(sum.rem_euclid(m) as $t)
            }

            /* modulo mul cycles (signed) */

            /// Computes the non-negative modulo of `self + other` over |`modulus`|,
            /// and the number of cycles the result is reduced.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `i128`
            /// it can also return [`Overflow`], more probably than in
            /// [`modulo_mul`][Self::modulo_mul] since we can't reduce
            /// the operands beforehand in order to calculate *times*.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(-4, m)?, (2, 5)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(-3, m)?, (0, 4)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(-2, m)?, (1, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(-1, m)?, (2, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles( 0, m)?, (0, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles( 1, m)?, (1, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles( 2, m)?, (2, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles( 3, m)?, (0, 4)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles( 4, m)?, (1, 5)];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_mul_cycles(self, other: $t, modulus: $t)
                -> Result<ValueQuant<Int<$t>, Int<$t>>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    // not reducing for i128 makes overflow more likely,
                    // but we can't if we want to calculate `times`.
                    let sum = upcastop![err *mul(a, b) $is_up];
                    if let Some(v) = sum.checked_rem_euclid(m) {
                        let modulo = Int(v as $t);
                        let times = Int(((sum / m) as $t).abs());
                        Ok(ValueQuant::new(modulo, times))
                    } else {
                        cold_err_overflow()
                    }
                }
            }

            /// Computes the non-negative modulo of `self + other` over |`modulus`|,
            /// and the number of cycles the result is reduced,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `i128` it can also panic on overflow,
            /// more probably than in [`modulo_mul_unchecked`][Self::modulo_mul_unchecked]
            /// since we can't reduce the operands beforehand in order to calculate *times*.
            #[inline]
            pub const fn modulo_mul_cycles_unchecked(self, other: $t, modulus: $t)
                -> ValueQuant<Int<$t>, Int<$t>> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                // not reducing for i128 makes overflow more likely,
                // but we can't if we want to calculate `times`.
                let sum = a + b;
                let modulo = sum.rem_euclid(m) as $t;
                let times = ((sum / m) as $t).abs();
                ValueQuant::new(Int(modulo), Int(times))
            }

            /* modulo mul inv (signed) */

            /// Calculates the modular multiplicative inverse.
            ///
            /// The modular multiplicative inverse of *self* modulo *modulus*
            /// is an integer *b* such that $ ab \equiv 1 (\mod m) $.
            ///
            /// The modular multiplicative inverse exists only if `self` and
            /// `modulus` are coprime, meaning their greatest common divisor is 1.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`,
            /// or [`NoInverse`] if there's no inverse.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 5;
            #[doc = "assert_eq![Int(-4_" $t ").modulo_mul_inv(m)?, 4];"]
            #[doc = "assert_eq![Int(-3_" $t ").modulo_mul_inv(m)?, 2];"]
            #[doc = "assert_eq![Int(-2_" $t ").modulo_mul_inv(m)?, 3];"]
            #[doc = "assert_eq![Int(-1_" $t ").modulo_mul_inv(m)?, 1];"]
            #[doc = "assert_eq![Int( 0_" $t ").modulo_mul_inv(m), Err(NumError::NoInverse)];"]
            #[doc = "assert_eq![Int( 1_" $t ").modulo_mul_inv(m)?, 1];"]
            #[doc = "assert_eq![Int( 2_" $t ").modulo_mul_inv(m)?, 3];"]
            #[doc = "assert_eq![Int( 3_" $t ").modulo_mul_inv(m)?, 2];"]
            #[doc = "assert_eq![Int( 4_" $t ").modulo_mul_inv(m)?, 4];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_mul_inv(self, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (gcd, x, _) = self.gcd_ext(modulus).as_tuple_const();
                    if gcd.0 != 1 {
                        Err(NoInverse)
                    } else {
                        Ok(Int(x.0.rem_euclid(modulus)))
                    }
                }
            }

            /// Calculates the modular multiplicative inverse,
            /// unchecked version.
            ///
            /// The modular multiplicative inverse of *self* modulo *modulus*
            /// is an integer *b* such that $ ab \equiv 1 (\mod m) $.
            ///
            /// The modular multiplicative inverse exists only if `self` and
            /// `modulus` are coprime, meaning their greatest common divisor is 1.
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and if there's no inverse.
            #[inline]
            pub const fn modulo_mul_inv_unchecked(self, modulus: $t) -> Int<$t> {
                let (gcd, x, _) = self.gcd_ext(modulus).as_tuple_const();
                if gcd.0 != 1 {
                    panic!["No inverse"];
                } else {
                    Int(x.0.rem_euclid(modulus))
                }
            }
        }
    }};

    // implements unsigned ops
    (@unsigned
         ($t:ty : $cap:literal, $up:ty | $iup:ty : $icap:literal : $is_up:ident, $d:literal)
    ) => { paste! {
        #[doc = "# Integer modulo related methods for `" $t "`\n\n"]
        #[doc = "- [modulo](#method.modulo" $d
            ") *([uc](#method.modulo_unchecked" $d ")*)"]
        #[doc = "- [modulo_cycles](#method.modulo_cycles" $d
            ") *([uc](#method.modulo_cycles_unchecked" $d "))*"]
        //
        #[doc = "- [modulo_add](#method.modulo_add" $d
            ") *([uc](#method.modulo_add_unchecked" $d "))*"]
        #[doc = "- [modulo_add_cycles](#method.modulo_add_cycles" $d
            ") *([uc](#method.modulo_add_cycles_unchecked" $d "))*"]
        #[doc = "- [modulo_add_inv](#method.modulo_add_inv" $d
            ") *([uc](#method.modulo_add_inv_unchecked" $d "))*"]
        //
        #[doc = "- [modulo_sub](#method.modulo_sub" $d
            ") *([uc](#method.modulo_sub_unchecked" $d "))*"]
        #[doc = "- [modulo_sub_cycles](#method.modulo_sub_cycles" $d
            ") *([uc](#method.modulo_sub_cycles_unchecked" $d "))*"]
        //
        #[doc = "- [modulo_mul](#method.modulo_mul" $d
            ") *([uc](#method.modulo_mul_unchecked" $d "))*"]
        #[doc = "- [modulo_mul_cycles](#method.modulo_mul_cycles" $d
            ") *([uc](#method.modulo_mul_cycles_unchecked" $d "))*"]
        #[doc = "- [modulo_mul_inv](#method.modulo_mul_inv" $d
            ") *([uc](#method.modulo_mul_inv_unchecked" $d "))*"]
        ///
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
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

            /* modulo cycles (unsigned) */

            /// Computes the non-negative modulo of `self` over `modulus`,
            /// and the number of cycles it is reduced.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(0_" $t ").modulo_cycles(m)?, (0, 0)];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo_cycles(m)?, (1, 0)];"]
            #[doc = "assert_eq![Int(2_" $t ").modulo_cycles(m)?, (2, 0)];"]
            #[doc = "assert_eq![Int(3_" $t ").modulo_cycles(m)?, (0, 1)];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_cycles(self, modulus: $t) -> Result<ValueQuant<Int<$t>, Int<$t>>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    Ok(ValueQuant::new(Int(self.0 % modulus), Int(self.0 / modulus)))
                }
            }

            /// Computes the non-negative modulo of `self` over `modulus`,
            /// and the number of cycles it is reduced,
            /// unchecked version.
            ///
            /// # Panics
            /// Panics if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            /// let m = 3;
            #[doc = "assert_eq![Int(0_" $t ").modulo_cycles_unchecked(m), (0, 0)];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo_cycles_unchecked(m), (1, 0)];"]
            #[doc = "assert_eq![Int(2_" $t ").modulo_cycles_unchecked(m), (2, 0)];"]
            #[doc = "assert_eq![Int(3_" $t ").modulo_cycles_unchecked(m), (0, 1)];"]
            /// ```
            #[inline]
            pub const fn modulo_cycles_unchecked(self, modulus: $t)
                -> ValueQuant<Int<$t>, Int<$t>> {
                ValueQuant::new(Int(self.0 % modulus), Int(self.0 / modulus))
            }

            /* modulo add (unsigned) */

            /// Computes the modulo of `self + other` over `modulus`.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
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
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(0, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(1, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(2, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(3, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add(4, m)?, 2];"]
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
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `u128` it could also panic on overflow.
            #[inline]
            pub const fn modulo_add_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                let sum = upcastop![reduce +add(a, b) % m, $is_up];
                Int((sum % m) as $t)
            }

            /* modulo add cycles (unsigned) */

            /// Computes the modulo of `self + other` over `modulus`,
            /// and the number of cycles the result is reduced.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
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
            /// let m = 3;
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(0, m)?, (1, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(1, m)?, (2, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(2, m)?, (0, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(3, m)?, (1, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_add_cycles(4, m)?, (2, 2)];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_add_cycles(self, other: $t, modulus: $t)
                -> Result<ValueQuant<Int<$t>, Int<$t>>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    // not reducing for u128 makes overflow more likely,
                    // but we can't if we want to calculate `times`.
                    let sum = upcastop![err +add(a, b) $is_up];
                    let modulo = Int((sum % m) as $t);
                    let times = Int((sum / m) as $t);
                    Ok(ValueQuant::new(modulo, times))
                }
            }

            /// Computes the modulo of `self + other` over `modulus`,
            /// and the number of cycles the result is reduced,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `u128` it can also panic on overflow,
            /// more probably than in [`modulo_add_unchecked`][Self::modulo_add_unchecked]
            /// since we can't reduce the operands beforehand in order to calculate *times*.
            #[inline]
            pub const fn modulo_add_cycles_unchecked(self, other: $t, modulus: $t)
                -> ValueQuant<Int<$t>, Int<$t>> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                // not reducing for u128 makes overflow more likely,
                // but we can't if we want to calculate `times`.
                let sum = a + b;
                let modulo = Int((sum % m) as $t);
                let times = Int((sum / m) as $t);
                ValueQuant::new(modulo, times)
            }

            /* modulo add inverse (unsigned) */

            /// Calculates the modular additive inverse.
            ///
            /// The modular additive inverse of *self* modulo *modulus*
            /// is an integer *b* such that $ a+b \equiv 0 (\mod m) $.
            ///
            /// The modular multiplicative inverse always exists and is simply
            /// `modulus - self` if `self != 0`, or 0 otherwise.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(0_" $t ").modulo_add_inv(m)?, 0];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo_add_inv(m)?, 2];"]
            #[doc = "assert_eq![Int(2_" $t ").modulo_add_inv(m)?, 1];"]
            #[doc = "assert_eq![Int(3_" $t ").modulo_add_inv(m)?, 0];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_add_inv(self, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let rem = (self.0.rem_euclid(modulus));
                    iif![rem == 0; Ok(Int(0)); Ok(Int(modulus - rem))]
                }
            }

            /// Calculates the modular additive inverse,
            /// unchecked version.
            ///
            /// The modular additive inverse of *self* modulo *modulus*
            /// is an integer *b* such that $ a+b \equiv 0 (\mod m) $.
            ///
            /// The modular multiplicative inverse always exists and is simply
            /// `modulus - self` if `self != 0`, or 0 otherwise.
            ///
            /// # Panics
            /// Panics if `modulus == 0`.
            #[inline]
            pub const fn modulo_add_inv_unchecked(self, modulus: $t) -> Int<$t> {
                let rem = (self.0.rem_euclid(modulus));
                iif![rem == 0; Int(0); Int(modulus - rem)]
            }

            /* modulo sub (unsigned) */

            /// Computes the modulo of `self - other` over `modulus`.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and it can also
            /// return [`Overflow`] if the result would be a negative value.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(0, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(1, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(2, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(3, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub(4, m)?, 0];"]
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
            /// Panics if `modulus == 0`, and if the result would be a negative value.
            #[inline]
            pub const fn modulo_sub_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                Int(((self.0 - other) % modulus))
            }

            /* modulo sub cycles (unsigned) */

            /// Computes the modulo of `self - other` over `modulus`,
            /// and the number of cycles the result is reduced.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and it can also
            /// return [`Overflow`] if the result would be a negative value.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(0, m)?, (1, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(1, m)?, (0, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(2, m)?, (2, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(3, m)?, (1, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_sub_cycles(4, m)?, (0, 0)];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_sub_cycles(self, other: $t, modulus: $t)
                -> Result<ValueQuant<Int<$t>, Int<$t>>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    if let Some(res) = self.0.checked_sub(other) {
                        let modulo = Int((res % modulus) as $t);
                        let times = Int((res / modulus) as $t);
                        Ok(ValueQuant::new(modulo, times))
                    } else {
                        Err(Overflow(None))
                    }
                }
            }

            /// Computes the modulo of `self - other` over `modulus`,
            /// and the number of cycles the result is reduced,
            /// unchecked version.
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and if the result would be a negative value.
            #[inline]
            pub const fn modulo_sub_cycles_unchecked(self, other: $t, modulus: $t)
                -> ValueQuant<Int<$t>, Int<$t>> {
                let res = self.0 - other;
                let modulo = Int((res % modulus) as $t);
                let times = Int((res / modulus) as $t);
                ValueQuant::new(modulo, times)
            }

            /* modulo mul (unsigned) */

            /// Computes the modulo of `self + other` over `modulus`.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
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
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(0, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(1, m)?, 1];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(2, m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(3, m)?, 0];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul(4, m)?, 1];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_mul(self, other: $t, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    let sum = upcastop![reduce_err *mul(a, b) % m, $is_up];
                    Ok((Int((sum % m) as $t)))
                }
            }

            /// Computes the modulo of `self + other` over `modulus`,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `u128` it could also panic on overflow.
            #[inline]
            pub const fn modulo_mul_unchecked(self, other: $t, modulus: $t) -> Int<$t> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                let sum = upcastop![reduce *mul(a, b) % m, $is_up];
                Int((sum % m) as $t)
            }

            /* modulo mul cycles (unsigned) */

            /// Computes the modulo of `self + other` over `modulus`,
            /// and the number of cycles the result is reduced.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`, and for `u128`
            /// it can also return [`Overflow`], more probably than in
            /// [`modulo_mul`][Self::modulo_mul] since we can't reduce
            /// the operands beforehand in order to calculate *times*.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 3;
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(0, m)?, (0, 0)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(1, m)?, (1, 1)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(2, m)?, (2, 2)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(3, m)?, (0, 4)];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_cycles(4, m)?, (1, 5)];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            pub const fn modulo_mul_cycles(self, other: $t, modulus: $t)
                -> Result<ValueQuant<Int<$t>, Int<$t>>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                    // not reducing for u128 makes overflow more likely,
                    // but we can't if we want to calculate `times`.
                    let sum = upcastop![err *mul(a, b) $is_up];
                    let modulo = Int((sum % m) as $t);
                    let times = Int((sum / m) as $t);
                    Ok(ValueQuant::new(modulo, times))
                }
            }

            /// Computes the modulo of `self + other` over `modulus`,
            /// and the number of cycles the result is reduced,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Panics
            /// Panics if `modulus == 0`, and for `u128` it can also panic on overflow,
            /// more probably than in [`modulo_mul_unchecked`][Self::modulo_mul_unchecked]
            /// since we can't reduce the operands beforehand in order to calculate *times*.
            #[inline]
            pub const fn modulo_mul_cycles_unchecked(self, other: $t, modulus: $t)
                -> ValueQuant<Int<$t>, Int<$t>> {
                let (a, b, m) = (self.0 as $up, other as $up, modulus as $up);
                // not reducing for u128 makes overflow more likely,
                // but we can't if we want to calculate `times`.
                let sum = a + b;
                let modulo = Int((sum % m) as $t);
                let times = Int((sum / m) as $t);
                ValueQuant::new(modulo, times)
            }

            /* modulo mul inv (unsigned) */

            /// Calculates the modular multiplicative inverse.
            ///
            #[doc = "It upcasts internally to [`" $iup "`]."]
            ///
            /// The modular multiplicative inverse of *a* modulo *m*
            /// is an integer *b* such that $ ab \equiv 1 (\mod m) $.
            ///
            /// The modular multiplicative inverse exists only if `self` and
            /// `modulus` are coprime, meaning their greatest common divisor is 1.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `modulus == 0`,
            /// [`NoInverse`] if there's no inverse,
            /// and for `u128` it could return [`Overflow`] when casting
            /// in the [`gcd_ext`][Self::gcd_ext] calculation.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumResult, NumError};
            /// # fn main() -> NumResult<()> {
            /// let m = 5;
            #[doc = "assert_eq![Int(0_" $t ").modulo_mul_inv(m), Err(NumError::NoInverse)];"]
            #[doc = "assert_eq![Int(1_" $t ").modulo_mul_inv(m)?, 1];"]
            #[doc = "assert_eq![Int(2_" $t ").modulo_mul_inv(m)?, 3];"]
            #[doc = "assert_eq![Int(3_" $t ").modulo_mul_inv(m)?, 2];"]
            #[doc = "assert_eq![Int(4_" $t ").modulo_mul_inv(m)?, 4];"]
            /// # Ok(()) }
            /// ```
            #[inline]
            #[cfg(feature = $icap )]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $icap)))]
            pub const fn modulo_mul_inv(self, modulus: $t) -> Result<Int<$t>> {
                if modulus == 0 {
                    cold_err_zero()
                } else {
                    let (gcd, x, _) = unwrap![ok? self.gcd_ext(modulus)].as_tuple_const();
                    if gcd.0 != 1 {
                        Err(NoInverse)
                    } else {
                        Ok(Int((x.0.rem_euclid(modulus as $iup) as $t)))
                    }
                }
            }

            /// Calculates the modular multiplicative inverse,
            /// unchecked version.
            ///
            #[doc = "It upcasts internally to [`" $iup "`]."]
            ///
            /// The modular multiplicative inverse of *a* modulo *m*
            /// is an integer *b* such that $ ab \equiv 1 (\mod m) $.
            ///
            /// The modular multiplicative inverse exists only if `self` and
            /// `modulus` are coprime, meaning their greatest common divisor is 1.
            ///
            /// # Panics
            /// Panics if `modulus == 0`, if there's no inverse,
            /// and for `u128` it could overflow when casting
            /// in the [`gcd_ext`][Self::gcd_ext] calculation.
            #[inline]
            #[cfg(feature = $icap )]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $icap)))]
            pub const fn modulo_mul_inv_unchecked(self, modulus: $t) -> Int<$t> {
                let (gcd, x, _) = unwrap![ok self.gcd_ext(modulus)].as_tuple_const();
                if gcd.0 != 1 {
                    panic!["no inverse"]; // CHECK without checking
                } else {
                    Int((x.0.rem_euclid(modulus as $iup) as $t))
                }
            }
        }
    }};
}
impl_int![signed
    (i8:"i8", i16:Y, ""), (i16:"i16", i32:Y, "-1"), (i32:"i32", i64:Y, "-2"),
    (i64:"i64", i128:Y, "-3"), (i128:"i128", i128:N, "-4"),
    (isize:"isize", isize_up:Y, "-5")
];
impl_int![unsigned
    (u8:"u8", u16|i16:"i16":Y, "-6"), (u16:"u16", u32|i32:"i32":Y, "-7"),
    (u32:"u32", u64|i64:"i64":Y, "-8"), (u64:"u64", u128|i128:"i128":Y, "-9"),
    (u128:"u128", u128|i128:"i128":N, "-10")
    // (usize:"usize", usize_up|isize_up:Y, "-11")
];
#[cfg(target_pointer_width = "32")]
impl_int![unsigned (usize:"usize", usize_up|isize_up:"i64":Y, "-11")];
#[cfg(target_pointer_width = "64")]
impl_int![unsigned (usize:"usize", usize_up|isize_up:"i128":Y, "-11")];
