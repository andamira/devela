// devela::num::int::wrapper::impl_base
//
//! implements base related functions
//
// TOC
// - signed|unsigned:
//   - digits
//   - digits_sign
//   - digits_base
//   - digits_base_sign
//   - digital_root
//   - digital_root_base

use super::Int;
use crate::code::{iif, paste};

// $t:   the input/output type
// $dl:  the doclink suffix for the method name
macro_rules! impl_base {
    (signed $( $t:ty : $dl:literal ),+) => { $( impl_base![@signed $t:$dl]; )+ };
    (unsigned $( $t:ty : $dl:literal ),+) => { $( impl_base![@unsigned $t:$dl]; )+ };

    // implements signed ops
    (@signed $t:ty : $dl:literal) => { paste! {
        /* signed digits */

        #[doc = "# Numeric base related methods for `" $t "`\n\n"]
        #[doc = "- [digits](#method.digits" $dl ")"]
        #[doc = "- [digits_sign](#method.digits_sign" $dl ")"]
        #[doc = "- [digits_base](#method.digits_base" $dl ")"]
        #[doc = "- [digits_base_sign](#method.digits_base_sign" $dl ")"]
        #[doc = "- [digital_root](#method.digital_root" $dl ")"]
        #[doc = "- [digital_root_base](#method.digital_root_base" $dl ")"]
        ///
        /// See the related trait [`NumOpsBase`][crate::num::NumOpsBase].
        impl Int<$t> {
            /// Returns the number of digits in base 10.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(0_" $t ").digits()];"]
            #[doc = "assert_eq![1, Int(-1_" $t ").digits()];"]
            #[doc = "assert_eq![3, Int(127_" $t ").digits()];"]
            #[doc = "assert_eq![3, Int(-128_" $t ").digits()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits(self) -> $t {
                let n = iif![self.0 == $t::MIN; $t::MAX; self.0.abs()];
                iif![let Some(c) = n.checked_ilog10(); c as $t + 1; 1]
            }

            /// Returns the number of digits in base 10, including the negative sign.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(0_" $t ").digits_sign()];"]
            #[doc = "assert_eq![2, Int(-1_" $t ").digits_sign()];"]
            #[doc = "assert_eq![3, Int(127_" $t ").digits_sign()];"]
            #[doc = "assert_eq![4, Int(-128_" $t ").digits_sign()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits_sign(self) -> $t {
                let mut res = (self.0 < 0) as $t;
                let n = iif![self.0 == $t::MIN; $t::MAX; self.0.abs()];
                res += iif![let Some(c) = n.checked_ilog10(); c as $t + 1; 1];
                res
            }

            /// Returns the number of digits in the given positive `base`.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![2, Int(3_" $t ").digits_base(2)];"]
            #[doc = "assert_eq![2, Int(127_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![2, Int(-128_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![2, Int(-128_" $t ").digits_base(-16)];"]
            #[doc = "assert_eq![0, Int(100_" $t ").digits_base(0)];"]
            #[doc = "assert_eq![1, Int(0_" $t ").digits_base(100)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits_base(mut self, mut base: $t) -> $t {
                iif![base == 0; return 0];
                base = base.abs();
                self.0 = iif![self.0 == $t::MIN; $t::MAX; self.0.abs()];
                iif![let Some(c) = self.0.checked_ilog(base); c as $t + 1; 1]
            }

            /// Returns the number of digits in the given positive `base`,
            /// including the negative sign.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![2, Int(3_" $t ").digits_base_sign(2)];"]
            #[doc = "assert_eq![2, Int(127_" $t ").digits_base_sign(16)];"]
            #[doc = "assert_eq![3, Int(-128_" $t ").digits_base_sign(16)];"]
            #[doc = "assert_eq![3, Int(-128_" $t ").digits_base_sign(-16)];"]
            #[doc = "assert_eq![0, Int(100_" $t ").digits_base_sign(0)];"]
            #[doc = "assert_eq![1, Int(0_" $t ").digits_base_sign(100)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits_base_sign(mut self, mut base: $t) -> $t {
                iif![base == 0; return 0];
                base = base.abs();
                let mut res = (self.0 < 0) as $t;
                self.0 = iif![self.0 == $t::MIN; $t::MAX; self.0.abs()];
                res += iif![let Some(c) = self.0.checked_ilog(base); c as $t + 1; 1];
                res
            }

            /* signed digital_root */

            /// Returns the digital root in base 10.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(127_" $t ").digital_root()];"]
            #[doc = "assert_eq![1, Int(-127_" $t ").digital_root()];"]
            #[doc = "assert_eq![9, Int(126_" $t ").digital_root()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digital_root(self) -> $t {
                let mut n = self.0.abs();
                let mut sum = 0;
                while n > 0 {
                    sum += n % 10;
                    n /= 10;
                    iif![n == 0 && sum >= 10; { n = sum; sum = 0; }];
                }
                sum
            }

            /// Returns the digital root in in the given `positive` base.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(127_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![1, Int(127_" $t ").digital_root_base(-10)];"]
            #[doc = "assert_eq![1, Int(-127_" $t ").digital_root_base(-10)];"]
            #[doc = "assert_eq![9, Int(-126_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![3, Int(-33_" $t ").digital_root_base(16)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digital_root_base(self, base: $t) -> $t {
                let (mut n, base) = (self.0.abs(), base.abs());
                let mut sum = 0;
                while n > 0 {
                    sum += n % base;
                    n /= base;
                    iif![n == 0 && sum >= base; { n = sum; sum = 0; }];
                }
                sum
            }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $dl:literal) => { paste! {
        #[doc = "# Numeric base related methods for `" $t "`\n\n"]
        #[doc = "- [digits](#method.digits" $dl ")"]
        #[doc = "- [digits_sign](#method.digits_sign" $dl ")"]
        #[doc = "- [digits_base](#method.digits_base" $dl ")"]
        #[doc = "- [digits_base_sign](#method.digits_base_sign" $dl ")"]
        #[doc = "- [digital_root](#method.digital_root" $dl ")"]
        #[doc = "- [digital_root_base](#method.digital_root_base" $dl ")"]
        ///
        /// See the related trait [`NumOpsBase`][crate::num::NumOpsBase].
        impl Int<$t> {
            /* unsigned digits */

            /// Returns the number of digits in base 10.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(0_" $t ").digits()];"]
            #[doc = "assert_eq![3, Int(127_" $t ").digits()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits(self) -> $t {
                iif![let Some(c) = self.0.checked_ilog10(); c as $t + 1; 1]
            }

            /// An alias of [`digits`][Self#digits].
            #[inline] #[must_use]
            pub const fn digits_sign(self) -> $t {
                self.digits()
            }

            /// Returns the number of digits in the given `base`.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![2, Int(3_" $t ").digits_base(2)];"]
            #[doc = "assert_eq![2, Int(127_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![0, Int(100_" $t ").digits_base(0)];"]
            #[doc = "assert_eq![1, Int(0_" $t ").digits_base(100)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits_base(self, base: $t) -> $t {
                iif![base == 0; return 0];
                iif![let Some(c) = self.0.checked_ilog(base); c as $t + 1; 1]
            }

            /// An alias of [`digits_base`][Self#digits_base].
            #[inline] #[must_use]
            pub const fn digits_base_sign(self, base: $t) -> $t {
                self.digits_base(base)
            }

            /* unsigned digital_root */

            /// Returns the digital root in base 10.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(127_" $t ").digital_root()];"]
            #[doc = "assert_eq![9, Int(126_" $t ").digital_root()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digital_root(mut self) -> $t {
                let mut sum = 0;
                while self.0 > 0 {
                    sum += self.0 % 10;
                    self.0 /= 10;
                    iif![self.0 == 0 && sum >= 10; { self.0 = sum; sum = 0; }];
                }
                sum
            }

            /// Returns the digital root in in the given `positive` base.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![1, Int(127_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![3, Int(33_" $t ").digital_root_base(16)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digital_root_base(mut self, base: $t) -> $t {
                let mut sum = 0;
                while self.0 > 0 {
                    sum += self.0 % base;
                    self.0 /= base;
                    iif![self.0 == 0 && sum >= base; { self.0 = sum; sum = 0; }];
                }
                sum
            }
        }
    }};
}
impl_base![signed i8:"", i16:"-1", i32:"-2", i64:"-3", i128:"-4", isize:"-5"];
impl_base![unsigned u8:"-6", u16:"-7", u32:"-8", u64:"-9", u128:"-10", usize:"-11"];
