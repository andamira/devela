// devela::num::int::wrapper::impl_root
//
//! Implements root related methods for [`Int`].
//
// TOC
// - signed|unsigned:
// - is_square
// - sqrt_ceil
// - sqrt_floor
// - sqrt_round
// - is_power TODO
// - root_ceil
// - root_floor
// - root_round TODO

use super::super::shared_docs::*;
#[allow(unused_imports, reason = "various reasons")]
use crate::NumError::{self, NonNegativeRequired, NonZeroRequired, Overflow};
#[cfg(any(feature = "_int_isize", feature = "_int_usize"))]
use crate::isize_up;
#[cfg(feature = "_int_usize")]
use crate::usize_up;
use crate::{Int, NumResult as Result, is, num::upcasted_op, paste};

// helper function to be called from the cold path branch when nth == 0 in root_*.
#[cold] #[inline(never)] #[rustfmt::skip] #[cfg(_int··)]
const fn cold_err_zero<T>() -> Result<T> { Err(NonZeroRequired) }
// helper function to be called from the cold path branches with an ok result.
#[cold] #[inline(never)] #[rustfmt::skip] #[cfg(_int··)]
const fn cold_ok_int<T>(t: T) -> Result<T> { Ok(t) }

/// Implements root related methods for [`Int`].
///
/// # Args
/// $t:   the input/output type. E.g. i8.
/// $up:  the upcasted input/output type. E.g. i16.
/// $cap: the capability feature that enables the given implementation. E.g "_int_i8".
/// $cmp: the feature that enables the given implementation. E.g "_cmp_i8".
///
/// $d:   the doclink suffix for the method name
macro_rules! impl_root {
    () => {
        impl_root![signed
            i8    |i16      :"_int_i8"    :"_cmp_i8"     |"",
            i16   |i32      :"_int_i16"   :"_cmp_i16"    |"-1",
            i32   |i64      :"_int_i32"   :"_cmp_i32"    |"-2",
            i64   |i128     :"_int_i64"   :"_cmp_i64"    |"-3",
            i128  |i128     :"_int_i128"  :"_cmp_i128"   |"-4",
            isize |isize_up :"_int_isize" :"_cmp_isize"  |"-5"
        ];
        impl_root![unsigned
            u8    |u16      :"_int_u8"    :"_cmp_u8"     |"-6",
            u16   |u32      :"_int_u16"   :"_cmp_u16"    | "-7",
            u32   |u64      :"_int_u32"   :"_cmp_u32"    | "-8",
            u64   |u128     :"_int_u64"   :"_cmp_u64"    | "-9",
            u128  |u128     :"_int_u128"  :"_cmp_u128"   | "-10",
            usize |usize_up :"_int_usize" /*_cmp_usize*/ | "-11" // always available
        ];
    };
    (signed $( $t:ty | $up:ty : $cap:literal $(: $cmp:literal)? | $d:literal ),+) => {
        $( impl_root![@signed $t|$up:$cap $(:$cmp)? | $d]; )+
    };
    (unsigned $( $t:ty | $up:ty : $cap:literal $(: $cmp:literal)? | $d:literal ),+) => {
        $( impl_root![@unsigned $t|$up:$cap $(:$cmp)? | $d]; )+
    };
    (
    // implements signed ops
    @signed $t:ty | $up:ty : $cap:literal $(: $cmp:literal)? | $d:literal) => { paste! {
        /* sqrt (signed) */

        #[doc = crate::doc_availability!(feature = $cap)]
        ///
        #[doc = "# Integer root related methods for `" $t "`\n\n"]
        #[doc = "- [is_square](#method.is_square" $d ")"]
        #[doc = "- [sqrt_ceil](#method.sqrt_ceil" $d ")"]
        #[doc = "- [sqrt_floor](#method.sqrt_floor" $d ")"]
        #[doc = "- [sqrt_round](#method.sqrt_round" $d ")"]
        #[doc = "- [root_ceil](#method.root_ceil" $d ")"]
        #[doc = "- [root_floor](#method.root_floor" $d ")"]
        #[cfg(feature = $cap )]
        impl Int<$t> {
            /// Returns `true` if it's a perfect square.
            ///
            /// Returns `false` otherwise, which includes all negative values.
            #[doc = FORMULA_IS_SQUARE!()]
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc="assert_eq![Int(12_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(13_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(16_" $t ").is_square(), true];"]
            #[doc="assert_eq![Int(20_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(21_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(-16_" $t ").is_square(), false];"]
            /// ```
            $(
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[cfg(feature = $cmp)]
            )? // $cmp
            #[must_use]
            pub const fn is_square(self) -> bool {
                let a = self.0;
                is![let Ok(sqrt) = self.sqrt_floor(); sqrt.0 * sqrt.0 == a; false]
            }
            $( // $cmp
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn is_square(self) -> bool {
                let a = self.0;
                is![let Ok(sqrt) = self.sqrt_floor(); sqrt.0 * sqrt.0 == a; false]
            }
            )?

            /// Returns the ceiled integer square root.
            /// # Errors
            /// Returns [`NonNegativeRequired`] if `self` is negative.
            /// # Formulation
            #[doc = ALGORITHM_SQRT_CEIL!()]
            /// # Examples
            /// ```
            /// # use devela::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_ceil(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_ceil(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_ceil(), Err(NonNegativeRequired)];"]
            /// ```
            $(
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[cfg(feature = $cmp)]
            )? // $cmp
            pub const fn sqrt_ceil(self) -> Result<Int<$t>> {
                let a = self.0;
                if let Ok(floor) = self.sqrt_floor() {
                    is![floor.0 * floor.0 == a; Ok(floor); Ok(Int(floor.0 + 1))]
                } else {
                    Err(NonNegativeRequired)
                }
            }
            $( // $ cmp
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn sqrt_ceil(self) -> Result<Int<$t>> {
                let a = self.0;
                if let Ok(floor) = self.sqrt_floor() {
                    is![floor.0 * floor.0 == a; Ok(floor); Ok(Int(floor.0 + 1))]
                } else {
                    Err(NonNegativeRequired)
                }
            }
            )?

            /// Returns the floored integer square root.
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if `self` is negative.
            ///
            /// # Formulation
            #[doc = ALGORITHM_SQRT_FLOOR!()]
            /// # Examples
            /// ```
            /// # use devela::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_floor(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_floor(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_floor(), Err(NonNegativeRequired)];"]
            /// ```
            $(
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[cfg(feature = $cmp)]
            )? // $cmp
            pub const fn sqrt_floor(self) -> Result<Int<$t>> {
                let a = crate::Compare(self.0).min(<$t>::MAX - 1); // avoid overflow on MAX
                if a.is_negative() {
                    Err(NonNegativeRequired)
                } else if a < 2 {
                    Ok(self)
                } else {
                    let (mut x, mut y) = (a, (a + a / a) / 2);
                    while y < x {
                        x = y;
                        y = (x + a / x) / 2;
                    }
                    Ok(Int(x))
                }
            }
            $( // $cmp
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn sqrt_floor(self) -> Result<Int<$t>> {
                let a = self.0.min(<$t>::MAX - 1); // avoid overflow on MAX
                if a.is_negative() {
                    Err(NonNegativeRequired)
                } else if a < 2 {
                    Ok(self)
                } else {
                    let (mut x, mut y) = (a, (a + a / a) / 2);
                    while y < x {
                        x = y;
                        y = (x + a / x) / 2;
                    }
                    Ok(Int(x))
                }
            }
            )?

            /// Returns the rounded integer square root.
            ///
            #[doc = "Performs operations internally as [`" $up "`]."]
            ///
            /// # Features
            /// Uses `unsafe_hint` for performance optimizations with upcasted arithmetic.
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if `self` is negative, or possibly [`Overflow`]
            /// if there's no larger type to upcast and the value is close to its maximum.
            /// # Formulation
            #[doc = ALGORITHM_SQRT_ROUND!()]
            /// # Examples
            /// ```
            /// # use devela::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_round(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_round(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_round(), Err(NonNegativeRequired)];"]
            /// ```
            pub const fn sqrt_round(self) -> Result<Int<$t>> {
                let a = self.0 as $up;
                if a.is_negative() {
                    Err(NonNegativeRequired)
                } else if a < 2 {
                    Ok(self)
                } else {
                    // sqrt_floor
                    let sum = upcasted_op![add_err(a, a / a) $t => $up];
                    let (mut x, mut y) = (a, sum / 2);
                    while y < x {
                        x = y;
                        let sum = upcasted_op![add_err(x, a / x) $t => $up];
                        y = sum / 2;
                    }
                    // do we have to round up?
                    let mul = upcasted_op![mul_err(x, x) $t => $up];
                    is![a - mul >= (x + 1) * (x + 1) - a; Ok(Int(x as $t + 1)); Ok(Int(x as $t))]
                }
            }

            /* root (signed) */

            /// Returns the ceiled integer `nth` root.
            ///
            #[doc = FORMULA_ROOT_CEIL_SIGNED!()]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `nth` is 0, or
            /// [`NonNegativeRequired`] if `self` is negative and `nth` is even.
            ///
            /// # Examples
            /// ```
            /// # use devela::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(48_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(70_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(81_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(88_" $t ").root_ceil(4), Ok(Int(4))];"]
            #[doc="assert_eq![Int(114_" $t ").root_ceil(4), Ok(Int(4))];"]
            #[doc="assert_eq![Int(-81" $t ").root_ceil(4), Err(NonNegativeRequired)];"]
            #[doc="assert_eq![Int(" $t "::MAX).root_ceil(1), Ok(Int(" $t "::MAX))];"]
            /// ```
            /// # Formulation
            /// ## Piece-wise
            #[doc = PIECEWISE_ROOT_CEIL_SIGNED!()]
            /// ## Algorithm
            #[doc = ALGORITHM_ROOT_CEIL_SIGNED!()]
            pub const fn root_ceil(self, nth: u32) -> Result<Int<$t>> {
                if nth == 0 {
                    cold_err_zero()
                } else if nth == 1 {
                    cold_ok_int(self)
                } else if self.0 == 0 {
                    cold_ok_int(Int(0))
                } else if self.0 < 0 && nth % 2 == 0 {
                    Err(NonNegativeRequired)
                } else {
                    let mut x = 1 as $t;
                    let positive_base = self.0.abs();
                    while let Some(val) = x.checked_pow(nth) {
                        if val > positive_base { break; }
                        x += 1;
                    }
                    let floor_root = x - 1;
                    let ceil_root = if floor_root.pow(nth) == positive_base {
                        floor_root
                    } else {
                        floor_root + 1
                    };
                    Ok(Int(ceil_root * self.0.signum()))
                }
            }

            /// Returns the floored integer `nth` root.
            ///
            #[doc = FORMULA_ROOT_FLOOR_SIGNED!()]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `nth` is 0, or
            /// [`NonNegativeRequired`] if `self` is negative and `nth` is even.
            ///
            /// # Examples
            /// ```
            /// # use devela::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(48_" $t ").root_floor(4), Ok(Int(2))];"]
            #[doc="assert_eq![Int(70_" $t ").root_floor(4), Ok(Int(2))];"]
            #[doc="assert_eq![Int(81_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(88_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(114_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(-81_" $t ").root_floor(4), Err(NonNegativeRequired)];"]
            #[doc="assert_eq![Int(" $t "::MAX).root_floor(1), Ok(Int(" $t "::MAX))];"]
            /// ```
            /// # Formulations
            /// ## Piece-wise
            #[doc = PIECEWISE_ROOT_FLOOR_SIGNED!()]
            /// ## Algorithm
            #[doc = ALGORITHM_ROOT_FLOOR_SIGNED!()]
            pub const fn root_floor(self, nth: u32) -> Result<Int<$t>> {
                if nth == 0 {
                    cold_err_zero()
                } else if nth == 1 {
                    cold_ok_int(self)
                } else if self.0 == 0 {
                    cold_ok_int(Int(0))
                } else if self.0 < 0 && nth % 2 == 0 {
                    Err(NonNegativeRequired)
                } else {
                    let mut x = 1 as $t;
                    let positive_base = self.0.abs();
                    while let Some(val) = x.checked_pow(nth) {
                        if val > positive_base { break; }
                        x += 1;
                    }
                    Ok(Int((x - 1) * self.0.signum()))
                }
            }

            // TODO: root_round
        }
    }};
    (
    // implements unsigned ops
    @unsigned $t:ty | $up:ty : $cap:literal $(: $cmp:literal)? | $d:literal) => { paste! {
        #[doc = crate::doc_availability!(feature = $cap)]
        ///
        #[doc = "# Integer root related methods for `" $t "`\n\n"]
        #[doc = "- [is_square](#method.is_square" $d ")"]
        #[doc = "- [sqrt_ceil](#method.sqrt_ceil" $d ")"]
        #[doc = "- [sqrt_floor](#method.sqrt_floor" $d ")"]
        #[doc = "- [sqrt_round](#method.sqrt_round" $d ")"]
        #[doc = "- [root_ceil](#method.root_ceil" $d ")"]
        #[doc = "- [root_floor](#method.root_floor" $d ")"]
        #[cfg(feature = $cap )]
        impl Int<$t> {
            /* sqrt (unsigned) */

            /// Returns `true` if it's a perfect square, false otherwise.
            /// # Formulation
            #[doc = FORMULA_IS_SQUARE!()]
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc="assert_eq![Int(12_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(13_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(16_" $t ").is_square(), true];"]
            #[doc="assert_eq![Int(20_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(21_" $t ").is_square(), false];"]
            /// ```
            $(
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[cfg(feature = $cmp)]
            )? // $cmp
            #[must_use]
            pub const fn is_square(self) -> bool {
                let a = self.0;
                let sqrt = self.sqrt_floor();
                sqrt.0 * sqrt.0 == a
            }
            $( // $cmp
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn is_square(self) -> bool {
                let a = self.0;
                let sqrt = self.sqrt_floor();
                sqrt.0 * sqrt.0 == a
            }
            )?

            /// Returns the ceiled integer square root.
            ///
            /// # Formulation
            #[doc = ALGORITHM_SQRT_CEIL!()]
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc="assert_eq![Int(12_" $t ").sqrt_ceil(), Int(4)];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_ceil(), Int(4)];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_ceil(), Int(4)];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_ceil(), Int(5)];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_ceil(), Int(5)];"]
            /// ```
            $(
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[cfg(feature = $cmp)]
            )? // $cmp
            pub const fn sqrt_ceil(self) -> Int<$t> {
                let a = self.0; let floor = self.sqrt_floor();
                is![floor.0 * floor.0 == a; floor; Int(floor.0 + 1)]
            }
            $( // $cmp
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn sqrt_ceil(self) -> Int<$t> {
                let a = self.0; let floor = self.sqrt_floor();
                is![floor.0 * floor.0 == a; floor; Int(floor.0 + 1)]
            }
            )?

            /// Returns the floored integer square root.
            ///
            /// # Formulation
            #[doc = ALGORITHM_SQRT_FLOOR!()]
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc="assert_eq![Int(12_" $t ").sqrt_floor(), Int(3)];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_floor(), Int(3)];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_floor(), Int(4)];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_floor(), Int(4)];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_floor(), Int(4)];"]
            /// ```
            $(
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[cfg(feature = $cmp)]
            )? // $cmp
            pub const fn sqrt_floor(self) -> Int<$t> {
                let a = crate::Compare(self.0).min(<$t>::MAX - 1); // avoid overflow on MAX
                if a < 2 {
                    self
                } else {
                    let (mut x, mut y) = (a, (a + a / a) / 2);
                    while y < x {
                        x = y;
                        y = (x + a / x) / 2;
                    }
                    Int(x)
                }
            }
            $( // $cmp
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn sqrt_floor(self) -> Int<$t> {
                let a = self.0.min(<$t>::MAX - 1); // avoid overflow on MAX
                if a < 2 {
                    self
                } else {
                    let (mut x, mut y) = (a, (a + a / a) / 2);
                    while y < x {
                        x = y;
                        y = (x + a / x) / 2;
                    }
                    Int(x)
                }
            }
            )?

            /// Returns the rounded integer square root.
            ///
            #[doc = "Performs operations internally as [`" $up "`]."]
            ///
            /// # Features
            /// Uses `unsafe_hint` for performance optimizations with upcasted arithmetic.
            ///
            /// # Errors
            /// Can returns [`Overflow`] if there's no larger type to upcast and the value
            /// is close to its maximum.
            ///
            /// # Formulation
            #[doc = ALGORITHM_SQRT_ROUND!()]
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc="assert_eq![Int(12_" $t ").sqrt_round(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_round(), Ok(Int(5))];"]
            /// ```
            pub const fn sqrt_round(self) -> Result<Int<$t>> {
                let a = self.0 as $up;
                if a < 2 {
                    Ok(self)
                } else {
                    // sqrt_floor
                    let sum = upcasted_op![add_err(a, a / a) $t => $up];
                    let (mut x, mut y) = (a, sum / 2);
                    while y < x {
                        x = y;
                        let sum = upcasted_op![add_err(x, a / x) $t => $up];
                        y = sum / 2;
                    }
                    // do we have to round up?
                    let mul = upcasted_op![mul_err(x, x) $t => $up];
                    is![a - mul >= (x + 1) * (x + 1) - a; Ok(Int(x as $t + 1)); Ok(Int(x as $t))]
                }
            }

            /* root (unsigned) */

            /// Returns the ceiled integer `nth` root.
            ///
            #[doc = FORMULA_ROOT_CEIL_UNSIGNED!()]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `nth` is 0.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc="assert_eq![Int(48_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(70_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(81_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(88_" $t ").root_ceil(4), Ok(Int(4))];"]
            #[doc="assert_eq![Int(114_" $t ").root_ceil(4), Ok(Int(4))];"]
            #[doc="assert_eq![Int(" $t "::MAX).root_ceil(1), Ok(Int(" $t "::MAX))];"]
            /// ```
            /// # Formulation
            /// ## Piece-wise
            #[doc = PIECEWISE_ROOT_CEIL_UNSIGNED!()]
            /// ## Algorithm
            #[doc = ALGORITHM_ROOT_CEIL_UNSIGNED!()]
            pub const fn root_ceil(self, nth: u32) -> Result<Int<$t>> {
                match self.root_floor(nth) {
                    Ok(floor_root) => {
                        if floor_root.0.pow(nth) == self.0 {
                            Ok(floor_root)
                        } else {
                            Ok(Int(floor_root.0 + 1))
                        }
                    },
                    Err(e) => Err(e),
                }
            }

            /// Returns the floored integer `nth` root.
            ///
            #[doc = FORMULA_ROOT_FLOOR_UNSIGNED!()]
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `nth` is 0.
            ///
            /// # Examples
            /// ```
            /// # use devela::Int;
            #[doc="assert_eq![Int(48_" $t ").root_floor(4), Ok(Int(2))];"]
            #[doc="assert_eq![Int(70_" $t ").root_floor(4), Ok(Int(2))];"]
            #[doc="assert_eq![Int(81_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(88_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(114_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(" $t "::MAX).root_floor(1), Ok(Int(" $t "::MAX))];"]
            /// ```
            /// # Formulations
            /// ## Piece-wise
            #[doc = PIECEWISE_ROOT_FLOOR_UNSIGNED!()]
            /// ## Algorithm
            #[doc = ALGORITHM_ROOT_FLOOR_UNSIGNED!()]
            pub const fn root_floor(self, nth: u32) -> Result<Int<$t>> {
                if nth == 0 {
                    cold_err_zero()
                } else if nth == 1 {
                    cold_ok_int(self)
                } else if self.0 == 0 {
                    cold_ok_int(Int(0))
                } else {
                    let mut x = 1 as $t;
                    while let Some(val) = x.checked_pow(nth) {
                        is![val > self.0; break];
                        x += 1;
                    }
                    Ok(Int(x - 1))
                }
            }

            // TODO: root_round
        }
    }};
}
impl_root!();
