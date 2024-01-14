// devela::num::int::wrapper::impl_sqrt
//
//! implements square root related functions
//
// TOC
// - signed|unsigned:
// - is_square
// - sqrt_ceil
// - sqrt_floor
// - sqrt_round

use super::Int;
use crate::code::{iif, paste};
use crate::num::{NumErrors as E, NumResult as Result};
use E::NonNegativeRequired;

// $t:   the input/output type
// $dl:  the doclink suffix for the method name
macro_rules! impl_sqrt {
    (signed $( $t:ty : $dl:literal ),+) => { $( impl_sqrt![@signed $t:$dl]; )+ };
    (unsigned $( $t:ty : $dl:literal ),+) => { $( impl_sqrt![@unsigned $t:$dl]; )+ };

    // implements signed ops
    (@signed $t:ty : $dl:literal) => { paste! {
        /* signed square root */

        #[doc = "# Integer square root related methods for `" $t "`\n\n"]
        #[doc = "- [is_square](#method.is_square" $dl ")"]
        #[doc = "- [sqrt_ceil](#method.sqrt_ceil" $dl ")"]
        #[doc = "- [sqrt_floor](#method.sqrt_floor" $dl ")"]
        #[doc = "- [sqrt_round](#method.sqrt_round" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
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
            #[inline] #[must_use]
            pub const fn is_square(self) -> bool {
                let a = self.0;
                iif![let Ok(sqrt) = self.sqrt_floor(); sqrt.0 * sqrt.0 == a; false]
            }

            /// Returns the ceiled integer square root.
            ///
            /// # Errors
            /// Returns [`NonNegativeRequired`] if `self` is negative.
            ///
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
            /// # use devela::num::{Int, NumErrors::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_ceil(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_ceil(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_ceil(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_ceil(), Err(NonNegativeRequired)];"]
            /// ```
            #[inline]
            pub const fn sqrt_ceil(self) -> Result<Int<$t>> {
                let a = self.0;
                if let Ok(floor) = self.sqrt_floor() {
                    iif![floor.0 * floor.0 == a; Ok(floor); Ok(Int(floor.0 + 1))] // IMPROVE ops
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
            ///
            /// $$ \large
            /// n_{i+1} = n_{i} - ( n_{i}^{2} - a) / 2n_{i},
            /// \quad \small\text{for} \quad i = 0, 1, \ldots, k,
            /// $$
            ///
            /// Where $n_{i}$ is the current estimate, $n_{i+1}$ is the next
            /// estimate, $a$ is self, and $k$ is the number of iterations
            /// needed to converge to a solution, on the order of the number of
            /// bits of self, about $O(\log_2 b)$, which for e.g. 128 bits would
            /// be $ ±7 $ iterations.
            ///
            /// Hence, the function continues updating the estimate until
            /// reaching $n_{k}$, which provides the largest integer less than
            /// or equal to the square root of `a`.
            /// # Examples
            /// ```
            /// # use devela::num::{Int, NumErrors::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_floor(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_floor(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_floor(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_floor(), Err(NonNegativeRequired)];"]
            /// ```
            #[inline]
            pub const fn sqrt_floor(self) -> Result<Int<$t>> {
                let a = self.0;
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
            /// # Errors
            /// Returns [`NonNegativeRequired`] if `self` is negative.
            ///
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
            /// # use devela::num::{Int, NumErrors::NonNegativeRequired};
            #[doc="assert_eq![Int(12_" $t ").sqrt_round(), Ok(Int(3))];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_round(), Ok(Int(4))];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_round(), Ok(Int(5))];"]
            #[doc="assert_eq![Int(-4_" $t ").sqrt_round(), Err(NonNegativeRequired)];"]
            /// ```
            #[inline]
            pub const fn sqrt_round(self) -> Result<Int<$t>> {
                let a = self.0;
                if a.is_negative() {
                    Err(NonNegativeRequired)
                } else if a < 2 {
                    Ok(self)
                } else {
                    // sqrt_floor
                    let (mut x, mut y) = (a, (a + a / a) / 2);
                    while y < x {
                        x = y;
                        y = (x + a / x) / 2;
                    }
                    // do we have to round up?
                    iif![a - x * x >= (x + 1) * (x + 1) - a; Ok(Int(x + 1)); Ok(Int(x))]
                }
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $dl:literal) => { paste! {
        #[doc = "# Integer square root related methods for `" $t "`\n\n"]
        #[doc = "- [is_square](#method.is_square" $dl ")"]
        #[doc = "- [sqrt_ceil](#method.sqrt_ceil" $dl ")"]
        #[doc = "- [sqrt_floor](#method.sqrt_floor" $dl ")"]
        #[doc = "- [sqrt_round](#method.sqrt_round" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /* unsigned square root */

            /// Returns `true` if it's a perfect square, false otherwise.
            ///
            /// # Algorithm
            /// $$ \large
            /// \text{is\textunderscore square}(a) = \begin{cases}
            /// \text{true} & \text{if } \left(\lfloor \sqrt{a} \rfloor\right)^2 = a \cr
            /// \text{false} & \text{if } \left(\lfloor \sqrt{a} \rfloor\right)^2 \neq a
            /// \end{cases}
            /// $$
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc="assert_eq![Int(12_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(13_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(16_" $t ").is_square(), true];"]
            #[doc="assert_eq![Int(20_" $t ").is_square(), false];"]
            #[doc="assert_eq![Int(21_" $t ").is_square(), false];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_square(self) -> bool {
                let a = self.0;
                let sqrt = self.sqrt_floor();
                sqrt.0 * sqrt.0 == a // IMPROVE ops
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
            #[inline] #[must_use]
            pub const fn sqrt_ceil(self) -> Int<$t> {
                let a = self.0; let floor = self.sqrt_floor();
                iif![floor.0 * floor.0 == a; floor; Int(floor.0 + 1)] // IMPROVE ops
            }

            /// Returns the floored integer square root.
            /// # Algorithm
            /// $$ \large \left\lfloor \sqrt{a} \right\rfloor = n_{k} $$
            ///
            /// Where $n_{k}$ is the result of a sequence of estimates that
            /// starts with an initial $n_{0} = a/2$ which is updated using
            /// [*Heron's method*](
            /// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method):
            ///
            /// $$ \large
            /// n_{i+1} = n_{i} - ( n_{i}^{2} - a) / 2n_{i},
            /// \quad \small\text{for} \quad i = 0, 1, \ldots, k,
            /// $$
            ///
            /// Where $n_{i}$ is the current estimate, $n_{i+1}$ is the next
            /// estimate, $a$ is `a`, and $k$ is the number of iterations
            /// needed to converge to a solution, on the order of the number of
            /// bits of `a`, about $O(\log_2 b)$, which for e.g. 128 bits would
            /// be $ ±7 $ iterations.
            ///
            /// Hence, the function continues updating the estimate until
            /// reaching $n_{k}$, which provides the largest integer less than
            /// or equal to the square root of `a`.
            ///
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc="assert_eq![Int(12_" $t ").sqrt_floor(), Int(3)];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_floor(), Int(3)];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_floor(), Int(4)];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_floor(), Int(4)];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_floor(), Int(4)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn sqrt_floor(self) -> Int<$t> {
                let a = self.0;
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
            #[doc="assert_eq![Int(12_" $t ").sqrt_round(), Int(3)];"]
            #[doc="assert_eq![Int(13_" $t ").sqrt_round(), Int(4)];"]
            #[doc="assert_eq![Int(16_" $t ").sqrt_round(), Int(4)];"]
            #[doc="assert_eq![Int(20_" $t ").sqrt_round(), Int(4)];"]
            #[doc="assert_eq![Int(21_" $t ").sqrt_round(), Int(5)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn sqrt_round(self) -> Int<$t> {
                let a = self.0;
                if a < 2 {
                    self
                } else {
                    // sqrt_floor
                    let (mut x, mut y) = (a, (a + a / a) / 2);
                    while y < x {
                        x = y;
                        y = (x + a / x) / 2;
                    }
                    // do we have to round up?
                    iif![a - x * x >= (x + 1) * (x + 1) - a; Int(x + 1); Int(x)]
                }
            }
        }
    }};
}
impl_sqrt![signed i8:"", i16:"-1", i32:"-2", i64:"-3", i128:"-4", isize:"-5"];
impl_sqrt![unsigned u8:"-6", u16:"-7", u32:"-8", u64:"-9", u128:"-10", usize:"-11"];
