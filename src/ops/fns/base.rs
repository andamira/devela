// devela::ops::fns::base
//
//! Functions for numeric operations.
//
// TOC
// - sint & uint
//   - count_digits
//   - count_digits_sign (only signed)
//   - count_digits_base
//   - count_digits_base_sign (only signed)
//   - digital_root
//   - digital_root_base

use crate::meta::{iif, paste};

// signed|unsigned
// $t:   the input/output type
// $ut:  the upcasted type to do the operations on (the ones that can overflow)
// $ft:  the floating-point type to do the operations on (for lerp)
macro_rules! impl_ops {
    (signed $( ($t:ty, $up:ty) ),+) => { $( impl_ops![@signed($t, $up)]; )+ };
    (unsigned $( ($t:ty, $up:ty) ),+) => { $( impl_ops![@unsigned($t, $up)]; )+ };

    // implements signed ops
    (@signed($t:ty, $up:ty) ) => { paste! {
        /* signed count_digits */

        #[doc = "Returns the number of digits of an [`" $t "`] in base 10."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::count_digits_" $t ";\n\n"]
        #[doc = "assert_eq![count_digits_" $t "(0), 1];"]
        #[doc = "assert_eq![count_digits_" $t "(-1), 1];"]
        #[doc = "assert_eq![count_digits_" $t "(127), 3];"]
        #[doc = "assert_eq![count_digits_" $t "(-128), 3];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<count_digits_ $t>](n: $t) -> $t {
            let n = iif![n == $t::MIN; $t::MAX; n.abs()];
            iif![let Some(c) = n.checked_ilog10(); c as $t + 1; 1]
        }

        #[doc = "Returns the number of digits of an [`" $t
        "`] in base 10, plus the negative sign."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::count_digits_sign_" $t ";\n\n"]
        #[doc = "assert_eq![count_digits_sign_" $t "(0), 1];"]
        #[doc = "assert_eq![count_digits_sign_" $t "(-1), 2];"]
        #[doc = "assert_eq![count_digits_sign_" $t "(127), 3];"]
        #[doc = "assert_eq![count_digits_sign_" $t "(-128), 4];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<count_digits_sign_ $t>](n: $t) -> $t {
            let mut res = (n < 0) as $t;
            let n = iif![n == $t::MIN; $t::MAX; n.abs()];
            res += iif![let Some(c) = n.checked_ilog10(); c as $t + 1; 1];
            res
        }

        #[doc = "Returns the number of digits of an [`" $t "`] in the given positive `base`."]
        ///
        /// If the base is 0, it returns 0.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::count_digits_base_" $t ";\n\n"]
        #[doc = "assert_eq![count_digits_base_" $t "(3, 2), 2];"]
        #[doc = "assert_eq![count_digits_base_" $t "(127, 16), 2];"]
        #[doc = "assert_eq![count_digits_base_" $t "(-128, 16), 2];"]
        #[doc = "assert_eq![count_digits_base_" $t "(-128, -16), 2];"]
        #[doc = "assert_eq![count_digits_base_" $t "(100, 0), 0];"]
        #[doc = "assert_eq![count_digits_base_" $t "(0, 100), 1];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<count_digits_base_ $t>](mut n: $t, mut base: $t) -> $t {
            iif![base == 0; return 0];
            base = base.abs();
            n = iif![n == $t::MIN; $t::MAX; n.abs()];
            iif![let Some(c) = n.checked_ilog(base); c as $t + 1; 1]
        }

        #[doc = "Returns the number of digits of an [`" $t
        "`] in the given positive `base`, plus the negative sign."]
        ///
        /// If the base is 0, it returns 0.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::count_digits_base_sign_" $t ";\n\n"]
        #[doc = "assert_eq![count_digits_base_sign_" $t "(3, 2), 2];"]
        #[doc = "assert_eq![count_digits_base_sign_" $t "(127, 16), 2];"]
        #[doc = "assert_eq![count_digits_base_sign_" $t "(-128, 16), 3];"]
        #[doc = "assert_eq![count_digits_base_sign_" $t "(-128, -16), 3];"]
        #[doc = "assert_eq![count_digits_base_sign_" $t "(100, 0), 0];"]
        #[doc = "assert_eq![count_digits_base_sign_" $t "(0, 100), 1];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<count_digits_base_sign_ $t>](mut n: $t, mut base: $t) -> $t {
            iif![base == 0; return 0];
            base = base.abs();
            let mut res = (n < 0) as $t;
            n = iif![n == $t::MIN; $t::MAX; n.abs()];
            res += iif![let Some(c) = n.checked_ilog(base); c as $t + 1; 1];
            res
        }

        /* signed digital_root */

        #[doc = "Returns the digital root of an [`" $t "`] in base 10."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::digital_root_" $t ";\n\n"]
        #[doc = "assert_eq![digital_root_" $t "(127), 1];"]
        #[doc = "assert_eq![digital_root_" $t "(-127), 1];"]
        #[doc = "assert_eq![digital_root_" $t "(-126), 9];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<digital_root_ $t>](n: $t) -> $t {
            let mut n = n.abs();
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
                iif![n == 0 && sum >= 10; { n = sum; sum = 0; }];
            }
            sum
        }

        #[doc = "Returns the digital root of an [`" $t "`] in the given positive `base`."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::digital_root_base_" $t ";\n\n"]
        #[doc = "assert_eq![digital_root_base_" $t "(127, 10), 1];"]
        #[doc = "assert_eq![digital_root_base_" $t "(127, -10), 1];"]
        #[doc = "assert_eq![digital_root_base_" $t "(-127, -10), 1];"]
        #[doc = "assert_eq![digital_root_base_" $t "(-126, 10), 9];"]
        #[doc = "assert_eq![digital_root_base_" $t "(-33, 16), 3];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<digital_root_base_ $t>](n: $t, base: $t) -> $t {
            let (mut n, base) = (n.abs(), base.abs());
            let mut sum = 0;
            while n > 0 {
                sum += n % base;
                n /= base;
                iif![n == 0 && sum >= base; { n = sum; sum = 0; }];
            }
            sum
        }
    }};

    // implements unsigned ops
    (@unsigned($t:ty, $up:ty) ) => { paste! {
        /* signed count_digits */

        #[doc = "Returns the number of digits of a [`" $t "`] in base 10."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::count_digits_" $t ";\n\n"]
        #[doc = "assert_eq![count_digits_" $t "(0), 1];"]
        #[doc = "assert_eq![count_digits_" $t "(127), 3];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<count_digits_ $t>](n: $t) -> $t {
            iif![let Some(c) = n.checked_ilog10(); c as $t + 1; 1]
        }

        #[doc = "Returns the number of digits of a [`" $t "`] in the given `base`."]
        ///
        /// If the base is 0, it returns 0.
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::count_digits_base_" $t ";\n\n"]
        #[doc = "assert_eq![count_digits_base_" $t "(3, 2), 2];"]
        #[doc = "assert_eq![count_digits_base_" $t "(127, 16), 2];"]
        #[doc = "assert_eq![count_digits_base_" $t "(100, 0), 0];"]
        #[doc = "assert_eq![count_digits_base_" $t "(0, 100), 1];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<count_digits_base_ $t>](n: $t, base: $t) -> $t {
            iif![base == 0; return 0];
            iif![let Some(c) = n.checked_ilog(base); c as $t + 1; 1]
        }

        /* unsigned digital_root */

        #[doc = "Returns the digital root of a [`" $t "`] in base 10."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::digital_root_" $t ";\n\n"]
        #[doc = "assert_eq![digital_root_" $t "(127), 1];"]
        #[doc = "assert_eq![digital_root_" $t "(126), 9];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<digital_root_ $t>](mut n: $t) -> $t {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
                iif![n == 0 && sum >= 10; { n = sum; sum = 0; }];
            }
            sum
        }

        #[doc = "Returns the digital root of a [`" $t "`] in the given `base`."]
        ///
        /// # Examples
        /// ```
        #[doc ="use devela::ops::digital_root_base_" $t ";\n\n"]
        #[doc = "assert_eq![digital_root_base_" $t "(127, 10), 1];"]
        #[doc = "assert_eq![digital_root_base_" $t "(33, 16), 3];"]
        /// ```
        #[inline]
        #[must_use]
        pub const fn [<digital_root_base_ $t>](mut n: $t, base: $t) -> $t {
            let mut sum = 0;
            while n > 0 {
                sum += n % base;
                n /= base;
                iif![n == 0 && sum >= base; { n = sum; sum = 0; }];
            }
            sum
        }
    }};
}
impl_ops![
    signed(i8, i16),
    (i16, i32),
    (i32, i64),
    (i64, i128),
    (i128, i128),
    (isize, isize)
];
impl_ops![
    unsigned(u8, u16),
    (u16, u32),
    (u32, u64),
    (u64, u128),
    (u128, u128),
    (usize, usize)
];
