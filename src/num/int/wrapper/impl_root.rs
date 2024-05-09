// devela::num::int::wrapper::impl_root
//
//! implements root related functions
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

#![allow(unused_imports)] // TEMP WIP unwrap

#[cfg(any(feature = "_int_isize", feature = "_int_usize"))]
use crate::num::isize_up;
#[cfg(feature = "_int_usize")]
use crate::num::usize_up;
use crate::{
    code::{iif, unwrap},
    num::{upcasted_op, Int, NumError, NumResult as Result},
};
#[cfg(_some_int_i)]
use NumError::NonNegativeRequired;
use NumError::NonZeroRequired;
#[cfg(doc)]
use NumError::Overflow;

// helper function to be called from the cold path branch when nth == 0 in root_*.
#[cold] #[inline(never)] #[rustfmt::skip] #[cfg(_some_int)]
const fn cold_err_zero<T>() -> Result<T> { Err(NonZeroRequired) }
// helper function to be called from the cold path branches with an ok result.
#[cold] #[inline(never)] #[rustfmt::skip] #[cfg(_some_int)]
const fn cold_ok_int<T>(t: T) -> Result<T> { Ok(t) }

// $t:   the input/output type. E.g. i8.
// $up:  the upcasted input/output type. E.g. i16.
// $cap: the capability feature that enables the given implementation. E.g "_int_i8".
// $cmp: the feature that enables the given implementation. E.g "_cmp_i8".
// $d:   the doclink suffix for the method name
macro_rules! impl_int {
    () => {
        impl_int![signed
            i8|i16:"_int_i8":"_cmp_i8":"",
            i16|i32:"_int_i16":"_cmp_i16":"-1",
            i32|i64:"_int_i32":"_cmp_i32":"-2",
            i64|i128:"_int_i64":"_cmp_i64":"-3",
            i128|i128:"_int_i128":"_cmp_i128":"-4",
            isize|isize_up:"_int_isize":"_cmp_isize":"-5"
        ];
        impl_int![unsigned
            u8|u16:"_int_u8":"_cmp_u8":"-6",
            u16|u32:"_int_u16":"_cmp_u16":"-7",
            u32|u64:"_int_u32":"_cmp_u32":"-8",
            u64|u128:"_int_u64":"_cmp_u64":"-9",
            u128|u128:"_int_u128":"_cmp_u128":"-10",
            usize|usize_up:"_int_usize":"_cmp_usize":"-11"
        ];
    };

    (signed $( $t:ty | $up:ty : $cap:literal : $cmp:literal : $d:literal ),+) => {
        $( impl_int![@signed $t|$up:$cap:$cmp:$d]; )+
    };
    (unsigned $( $t:ty | $up:ty : $cap:literal : $cmp:literal : $d:literal ),+) => {
        $( impl_int![@unsigned $t|$up:$cap:$cmp:$d]; )+
    };

    // implements signed ops
    (@signed $t:ty | $up:ty : $cap:literal : $cmp:literal : $d:literal) => { $crate::paste! {
        /* sqrt (signed) */

        #[doc = "# Integer root related methods for `" $t "`\n\n"]
        #[doc = "- [is_square](#method.is_square" $d ")"]
        #[doc = "- [sqrt_ceil](#method.sqrt_ceil" $d ")"]
        #[doc = "- [sqrt_floor](#method.sqrt_floor" $d ")"]
        #[doc = "- [sqrt_round](#method.sqrt_round" $d ")"]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Int<$t> {
            /// Returns `true` if it's a perfect square.
            ///
            /// Returns `false` otherwise, which includes all negative values.
            /// # Algorithm
            /// $$ \large
            /// \text{is\textunderscore square}(a) = \begin{cases}
            /// \text{true} & \text{if } \left(\lfloor \sqrt{a} \rfloor\right)^2 = a \cr
            /// \text{false} & \text{if } \left(\lfloor \sqrt{a} \rfloor\right)^2 \neq a
            /// \end{cases}
            /// $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc="assert_eq![Int(12_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(13_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(16_" $t ").is_square(), true];"]
            #[doc="assert_eq![Int(20_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(21_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(-16_" $t ").is_square(), false];"]
            /// ```
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[inline] #[must_use]
            #[cfg(feature = $cmp)]
            pub const fn is_square(self) -> bool {
                let a = self.0;
                iif![let Ok(sqrt) = self.sqrt_floor(); sqrt.0 * sqrt.0 == a; false]
            }
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn is_square(self) -> bool {
                let a = self.0;
                iif![let Ok(sqrt) = self.sqrt_floor(); sqrt.0 * sqrt.0 == a; false]
            }

            /// Returns the ceiled integer square root.
            /// # Errors
            /// Returns [`NonNegativeRequired`] if `self` is negative.
            /// # Algorithm
            /// $$ \large
            /// \begin{align}
            /// \notag \left\lceil \sqrt{a} \thinspace\right\rceil = \begin{cases}
            /// n & \text{if } n^2 = a \cr
            /// n+1 & \text{if } n^2 < a \end{cases} \cr
            /// \notag \normalsize\text{where } n = \lfloor \sqrt{a} \rfloor &
            /// \end{align}
            /// $$
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_ceil(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_ceil(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_ceil(), Err(NonNegativeRequired)];"]
            /// ```
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[inline]
            #[cfg(feature = $cmp)]
            pub const fn sqrt_ceil(self) -> Result<Int<$t>> {
                let a = self.0;
                if let Ok(floor) = self.sqrt_floor() {
                    iif![floor.0 * floor.0 == a; Ok(floor); Ok(Int(floor.0 + 1))]
                } else {
                    Err(NonNegativeRequired)
                }
            }
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn sqrt_ceil(self) -> Result<Int<$t>> {
                let a = self.0;
                if let Ok(floor) = self.sqrt_floor() {
                    iif![floor.0 * floor.0 == a; Ok(floor); Ok(Int(floor.0 + 1))]
                } else {
                    Err(NonNegativeRequired)
                }
            }

            /// Returns the floored integer square root.
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if `self` is negative.
            ///
            /// # Algorithm
            /// $$ \large \left\lfloor \sqrt{a} \right\rfloor = n_{k} $$
            ///
            /// Where $n_{k}$ is the result of a sequence of estimates that
            /// starts with an initial $n_{0} = a/2$ which is updated using
            /// [*Heron's method*](
            /// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method):
            /// $$ \large
            /// n_{i+1} = n_{i} - ( n_{i}^{2} - a) / 2n_{i},
            /// \quad \small\text{for} \quad i = 0, 1, \ldots, k,
            /// $$
            /// Where $n_{i}$ is the current estimate, $n_{i+1}$ is the next
            /// estimate, $a$ is self, and $k$ is the number of iterations
            /// needed to converge to a solution, on the order of the number of
            /// bits of self, about $O(\log_2 b)$, which for e.g. 128 bits would
            /// be $ ±7 $ iterations.
            ///
            /// Hence, the function continues updating the estimate until
            /// reaching $n_{k}$, which provides the largest integer less than
            /// or equal to the square root of `a`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_floor(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_floor(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_floor(), Err(NonNegativeRequired)];"]
            /// ```
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[inline]
            #[cfg(feature = $cmp)]
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
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn sqrt_floor(self) -> Result<Int<$t>> {
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

            /// Returns the rounded integer square root.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if `self` is negative, or possibly [`Overflow`]
            /// if there's no larger type to upcast and the value is close to its maximum.
            /// # Algorithm
            /// $$ \large
            /// \begin{align}
            /// \notag \left\lfloor\sqrt{a} \thinspace\right\rceil = \begin{cases}
            /// n & \text{if } a - n^2 < (n+1)^2 - a \cr
            /// n+1 & \text{if } a - n^2 \geq (n+1)^2 - a \end{cases} \cr
            /// \notag \normalsize\text{where } n = \lfloor \sqrt{a} \rfloor &
            /// \end{align}
            /// $$
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_round(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_round(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_round(), Err(NonNegativeRequired)];"]
            /// ```
            #[inline]
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
                    iif![a - mul >= (x + 1) * (x + 1) - a; Ok(Int(x as $t + 1)); Ok(Int(x as $t))]
                }
            }

            /* root (signed) */

            /// Returns the ceiled integer `nth` root.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `nth` is 0, or
            /// [`NonNegativeRequired`] if `self` is negative and `nth` is even.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(48_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(70_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(81_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(88_" $t ").root_ceil(4), Ok(Int(4))];"]
            #[doc="assert_eq![Int(114_" $t ").root_ceil(4), Ok(Int(4))];"]
            #[doc="assert_eq![Int(-81" $t ").root_ceil(4), Err(NonNegativeRequired)];"]
            #[doc="assert_eq![Int(" $t "::MAX).root_ceil(1), Ok(Int(" $t "::MAX))];"]
            /// ```
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
            /// # Errors
            /// Returns [`NonZeroRequired`] if `nth` is 0, or
            /// [`NonNegativeRequired`] if `self` is negative and `nth` is even.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(48_" $t ").root_floor(4), Ok(Int(2))];"]
            #[doc="assert_eq![Int(70_" $t ").root_floor(4), Ok(Int(2))];"]
            #[doc="assert_eq![Int(81_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(88_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(114_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(-81_" $t ").root_floor(4), Err(NonNegativeRequired)];"]
            #[doc="assert_eq![Int(" $t "::MAX).root_floor(1), Ok(Int(" $t "::MAX))];"]
            /// ```
            #[inline]
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

    // implements unsigned ops
    (@unsigned $t:ty | $up:ty : $cap:literal : $cmp:literal : $d:literal) => { $crate::paste! {
        #[doc = "# Integer root related methods for `" $t "`\n\n"]
        #[doc = "- [is_square](#method.is_square" $d ")"]
        #[doc = "- [sqrt_ceil](#method.sqrt_ceil" $d ")"]
        #[doc = "- [sqrt_floor](#method.sqrt_floor" $d ")"]
        #[doc = "- [sqrt_round](#method.sqrt_round" $d ")"]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Int<$t> {
            /* sqrt (unsigned) */

            /// Returns `true` if it's a perfect square, false otherwise.
            /// # Algorithm
            /// $$ \large
            /// \text{is\textunderscore square}(a) = \begin{cases}
            /// \text{true} & \text{if } \left(\lfloor \sqrt{a} \rfloor\right)^2 = a \cr
            /// \text{false} & \text{if } \left(\lfloor \sqrt{a} \rfloor\right)^2 \neq a
            /// \end{cases}
            /// $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc="assert_eq![Int(12_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(13_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(16_" $t ").is_square(), true];"]
            #[doc="assert_eq![Int(20_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(21_" $t ").is_square(), false];"]
            /// ```
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[inline] #[must_use]
            #[cfg(feature = $cmp)]
            pub const fn is_square(self) -> bool {
                let a = self.0;
                let sqrt = self.sqrt_floor();
                sqrt.0 * sqrt.0 == a
            }
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn is_square(self) -> bool {
                let a = self.0;
                let sqrt = self.sqrt_floor();
                sqrt.0 * sqrt.0 == a
            }

            /// Returns the ceiled integer square root.
            /// # Algorithm
            /// $$ \large
            /// \begin{align}
            /// \notag \left\lceil \sqrt{a} \thinspace\right\rceil = \begin{cases}
            /// n & \text{if } n^2 = a \cr
            /// n+1 & \text{if } n^2 < a \end{cases} \cr
            /// \notag \normalsize\text{where } n = \lfloor \sqrt{a} \rfloor &
            /// \end{align}
            /// $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc="assert_eq![Int(12_" $t ").sqrt_ceil(), Int(4)];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_ceil(), Int(4)];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_ceil(), Int(4)];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_ceil(), Int(5)];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_ceil(), Int(5)];"]
            /// ```
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[inline] #[must_use]
            #[cfg(feature = $cmp)]
            pub const fn sqrt_ceil(self) -> Int<$t> {
                let a = self.0; let floor = self.sqrt_floor();
                iif![floor.0 * floor.0 == a; floor; Int(floor.0 + 1)]
            }
            #[cfg(not(feature = $cmp))] #[allow(missing_docs)]
            pub fn sqrt_ceil(self) -> Int<$t> {
                let a = self.0; let floor = self.sqrt_floor();
                iif![floor.0 * floor.0 == a; floor; Int(floor.0 + 1)]
            }

            /// Returns the floored integer square root.
            /// # Algorithm
            /// $$ \large \left\lfloor \sqrt{a} \right\rfloor = n_{k} $$
            ///
            /// Where $n_{k}$ is the result of a sequence of estimates that
            /// starts with an initial $n_{0} = a/2$ which is updated using
            /// [*Heron's method*](
            /// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method):
            /// $$ \large
            /// n_{i+1} = n_{i} - ( n_{i}^{2} - a) / 2n_{i},
            /// \quad \small\text{for} \quad i = 0, 1, \ldots, k,
            /// $$
            /// Where $n_{i}$ is the current estimate, $n_{i+1}$ is the next
            /// estimate, $a$ is `a`, and $k$ is the number of iterations
            /// needed to converge to a solution, on the order of the number of
            /// bits of `a`, about $O(\log_2 b)$, which for e.g. 128 bits would
            /// be $ ±7 $ iterations.
            ///
            /// Hence, the function continues updating the estimate until
            /// reaching $n_{k}$, which provides the largest integer less than
            /// or equal to the square root of `a`.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc="assert_eq![Int(12_" $t ").sqrt_floor(), Int(3)];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_floor(), Int(3)];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_floor(), Int(4)];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_floor(), Int(4)];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_floor(), Int(4)];"]
            /// ```
            /// # Features
            #[doc = "This will only be *const* if the " $cmp " feature is enabled."]
            #[inline] #[must_use]
            #[cfg(feature = $cmp)]
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

            /// Returns the rounded integer square root.
            ///
            #[doc = "It upcasts internally to [`" $up "`]."]
            ///
            /// # Errors
            /// Can returns [`Overflow`] if there's no larger type to upcast and the value
            /// is close to its maximum.
            /// # Algorithm
            /// $$ \large
            /// \begin{align}
            /// \notag \left\lfloor\sqrt{a} \thinspace\right\rceil = \begin{cases}
            /// n & \text{if } a - n^2 < (n+1)^2 - a \cr
            /// n+1 & \text{if } a - n^2 \geq (n+1)^2 - a \end{cases} \cr
            /// \notag \normalsize\text{where } n = \lfloor \sqrt{a} \rfloor &
            /// \end{align}
            /// $$
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc="assert_eq![Int(12_" $t ").sqrt_round(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_round(), Ok(Int(5))];"]
            /// ```
            #[inline]
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
                    iif![a - mul >= (x + 1) * (x + 1) - a; Ok(Int(x as $t + 1)); Ok(Int(x as $t))]
                }
            }

            /* root (unsigned) */

            /// Returns the ceiled integer `nth` root.
            ///
            /// # Errors
            /// Returns [`NonZeroRequired`] if `nth` is 0.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(48_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(70_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(81_" $t ").root_ceil(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(88_" $t ").root_ceil(4), Ok(Int(4))];"]
            #[doc="assert_eq![Int(114_" $t ").root_ceil(4), Ok(Int(4))];"]
            #[doc="assert_eq![Int(" $t "::MAX).root_ceil(1), Ok(Int(" $t "::MAX))];"]
            /// ```
            #[inline]
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
            /// # Errors
            /// Returns [`NonZeroRequired`] if `nth` is 0.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumError::NonNegativeRequired};
            #[doc="assert_eq![Int(48_" $t ").root_floor(4), Ok(Int(2))];"]
            #[doc="assert_eq![Int(70_" $t ").root_floor(4), Ok(Int(2))];"]
            #[doc="assert_eq![Int(81_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(88_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(114_" $t ").root_floor(4), Ok(Int(3))];"]
            #[doc="assert_eq![Int(" $t "::MAX).root_floor(1), Ok(Int(" $t "::MAX))];"]
            /// ```
            #[inline]
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
                        if val > self.0 {
                            break;
                        }
                        x += 1;
                    }
                    Ok(Int(x - 1))
                }
            }
        }

        // WIP TODO
        // ///
        // #[inline]
        // const fn root_round_u(self, nth: u32) -> Option<u32> {
        //     let floor_root = unwrap![ok? self.root_floor_u(nth)];
        //     let ceil_root_value = (floor_root + 1).pow(nth);
        //     let floor_root_value = floor_root.pow(nth);
        //     let midpoint = floor_root_value + ((ceil_root_value - floor_root_value) / 2);
        //     if self.0 > midpoint {
        //         Some(floor_root + 1)
        //     } else {
        //         Some(floor_root)
        //     }
        // }
    }};
}
impl_int!();
