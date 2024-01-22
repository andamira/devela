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

use crate::{
    code::{iif, paste},
    num::Int,
};
#[cfg(feature = "num_int_niche")]
use {
    crate::num::{impl_niche, niche::*, NumErrors, NumResult as Result},
    NumErrors::Invalid,
};

// $t:  the integer primitive input/output type, and the niche inner type
// $d:  the doclink suffix for the method name
macro_rules! impl_base {
    (prim_signed $( $t:ty : $d:literal ),+) => { $( impl_base![@prim_signed $t:$d]; )+ };
    (prim_unsigned $( $t:ty : $d:literal ),+) => { $( impl_base![@prim_unsigned $t:$d]; )+ };

    // implements ops on signed primitives
    (@prim_signed $t:ty : $d:literal) => { paste! {
        /* signed digits */

        #[doc = "# Integer base related methods for `" $t "`\n\n"]
        #[doc = "- [digits](#method.digits" $d ")"]
        #[doc = "- [digits_sign](#method.digits_sign" $d ")"]
        #[doc = "- [digits_base](#method.digits_base" $d ")"]
        #[doc = "- [digits_base_sign](#method.digits_base_sign" $d ")"]
        #[doc = "- [digital_root](#method.digital_root" $d ")"]
        #[doc = "- [digital_root_base](#method.digital_root_base" $d ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /// Returns the number of digits in base 10.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits()];"]
            #[doc = "assert_eq![Int(1), Int(-1_" $t ").digits()];"]
            #[doc = "assert_eq![Int(3), Int(127_" $t ").digits()];"]
            #[doc = "assert_eq![Int(3), Int(-128_" $t ").digits()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits(self) -> Int<$t> {
                let a = self.0; let n = iif![a == $t::MIN; $t::MAX; a.abs()];
                iif![let Some(c) = n.checked_ilog10(); Int(c as $t + 1); Int(1)]
            }

            /// Returns the number of digits in base 10, including the negative sign.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits_sign()];"]
            #[doc = "assert_eq![Int(2), Int(-1_" $t ").digits_sign()];"]
            #[doc = "assert_eq![Int(3), Int(127_" $t ").digits_sign()];"]
            #[doc = "assert_eq![Int(4), Int(-128_" $t ").digits_sign()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits_sign(self) -> Int<$t> {
                let a = self.0; let mut res = (a < 0) as $t;
                let n = iif![a == $t::MIN; $t::MAX; a.abs()];
                res += iif![let Some(c) = n.checked_ilog10(); c as $t + 1; 1];
                Int(res)
            }

            /// Returns the number of digits in the given positive `base`.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(2), Int(3_" $t ").digits_base(2)];"]
            #[doc = "assert_eq![Int(2), Int(127_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![Int(2), Int(-128_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![Int(2), Int(-128_" $t ").digits_base(-16)];"]
            #[doc = "assert_eq![Int(0), Int(100_" $t ").digits_base(0)];"]
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits_base(100)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits_base(self, mut base: $t) -> Int<$t> {
                let mut a = self.0;
                iif![base == 0; return Int(0)];
                base = base.abs();
                a = iif![a == $t::MIN; $t::MAX; a.abs()];
                iif![let Some(c) = a.checked_ilog(base); Int(c as $t + 1); Int(1)]
            }

            /// Returns the number of digits in the given positive `base`,
            /// including the negative sign.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(2), Int(3_" $t ").digits_base_sign(2)];"]
            #[doc = "assert_eq![Int(2), Int(127_" $t ").digits_base_sign(16)];"]
            #[doc = "assert_eq![Int(3), Int(-128_" $t ").digits_base_sign(16)];"]
            #[doc = "assert_eq![Int(3), Int(-128_" $t ").digits_base_sign(-16)];"]
            #[doc = "assert_eq![Int(0), Int(100_" $t ").digits_base_sign(0)];"]
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits_base_sign(100)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits_base_sign(self, mut base: $t) -> Int<$t> {
                let mut a = self.0;
                iif![base == 0; return Int(0)];
                base = base.abs();
                let mut res = (a < 0) as $t;
                a = iif![a == $t::MIN; $t::MAX; a.abs()];
                res += iif![let Some(c) = a.checked_ilog(base); c as $t + 1; 1];
                Int(res)
            }

            /* signed digital_root */

            /// Returns the digital root in base 10.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root()];"]
            #[doc = "assert_eq![Int(1), Int(-127_" $t ").digital_root()];"]
            #[doc = "assert_eq![Int(9), Int(126_" $t ").digital_root()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digital_root(self) -> Int<$t> {
                let mut n = self.0.abs();
                let mut sum = 0;
                while n > 0 {
                    sum += n % 10;
                    n /= 10;
                    iif![n == 0 && sum >= 10; { n = sum; sum = 0; }];
                }
                Int(sum)
            }

            /// Returns the digital root in in the given `positive` base.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root_base(-10)];"]
            #[doc = "assert_eq![Int(1), Int(-127_" $t ").digital_root_base(-10)];"]
            #[doc = "assert_eq![Int(9), Int(-126_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![Int(3), Int(-33_" $t ").digital_root_base(16)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digital_root_base(self, base: $t) -> Int<$t> {
                let (mut n, base) = (self.0.abs(), base.abs());
                let mut sum = 0;
                while n > 0 {
                    sum += n % base;
                    n /= base;
                    iif![n == 0 && sum >= base; { n = sum; sum = 0; }];
                }
                Int(sum)
            }
        }
    }};

    // implements ops on unsigned primitives
    (@prim_unsigned $t:ty : $d:literal) => { paste! {
        #[doc = "# Integer base related methods for `" $t "`\n\n"]
        #[doc = "- [digits](#method.digits" $d ")"]
        #[doc = "- [digits_sign](#method.digits_sign" $d ")"]
        #[doc = "- [digits_base](#method.digits_base" $d ")"]
        #[doc = "- [digits_base_sign](#method.digits_base_sign" $d ")"]
        #[doc = "- [digital_root](#method.digital_root" $d ")"]
        #[doc = "- [digital_root_base](#method.digital_root_base" $d ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /* unsigned digits */

            /// Returns the number of digits in base 10.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits()];"]
            #[doc = "assert_eq![Int(3), Int(127_" $t ").digits()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits(self) -> Int<$t> {
                iif![let Some(c) = self.0.checked_ilog10(); Int(c as $t + 1); Int(1)]
            }

            /// An alias of [`digits`][Self#digits].
            #[inline] #[must_use]
            pub const fn digits_sign(self) -> Int<$t> { self.digits() }

            /// Returns the number of digits in the given `base`.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(2), Int(3_" $t ").digits_base(2)];"]
            #[doc = "assert_eq![Int(2), Int(127_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![Int(0), Int(100_" $t ").digits_base(0)];"]
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits_base(100)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digits_base(self, base: $t) -> Int<$t> {
                let a = self.0; iif![base == 0; return Int(0)];
                iif![let Some(c) = a.checked_ilog(base); Int(c as $t + 1); Int(1)]
            }

            /// An alias of [`digits_base`][Self#digits_base].
            #[inline] #[must_use]
            pub const fn digits_base_sign(self, base: $t) -> Int<$t> { self.digits_base(base) }

            /* unsigned digital_root */

            /// Returns the digital root in base 10.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root()];"]
            #[doc = "assert_eq![Int(9), Int(126_" $t ").digital_root()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digital_root(self) -> Int<$t> {
                let [mut a, mut sum] = [self.0, 0];
                while a > 0 {
                    sum += a % 10;
                    a /= 10;
                    iif![a == 0 && sum >= 10; { a = sum; sum = 0; }];
                }
                Int(sum)
            }

            /// Returns the digital root in in the given `positive` base.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![Int(3), Int(33_" $t ").digital_root_base(16)];"]
            /// ```
            #[inline] #[must_use]
            pub const fn digital_root_base(self, base: $t) -> Int<$t> {
                let [mut a, mut sum] = [self.0, 0];
                while a > 0 {
                    sum += a % base;
                    a /= base;
                    iif![a == 0 && sum >= base; { a = sum; sum = 0; }];
                }
                Int(sum)
            }
        }
    }};

    // $n:  the niche type name prefix (e.g. NonRange)
    // $t:  the niche inner type (the associated primitive integer) (e.g. u8)
    // $($g)*: an optional list of const generics (e.g. RMIN, RMAX)
    // $d:  the doclink suffix for the method name
    // $dt: the doclink suffix for the associated method name implemented for the inner primitive
    (niche $( $n:ident : $t:ident <$($g:ident),*> : $d:literal : $dt: literal),+ $(,)? ) => {
        $( impl_base![@niche $n:$t <$($g),*> : $d:$dt ]; )+
    };
    (@niche $n:ident : $t:ident <$($g:ident),*> : $d:literal : $dt: literal) => { paste! {
        #[doc = "# Integer base related methods for `" [<$n$t:camel>] "`\n\n"]
        #[doc = "- [digits](#method.digits" $d ")"]
        #[doc = "- [digits_sign](#method.digits_sign" $d ")"]
        #[doc = "- [digits_base](#method.digits_base" $d ")"]
        #[doc = "- [digits_base_sign](#method.digits_base_sign" $d ")"]
        #[doc = "- [digital_root](#method.digital_root" $d ")"]
        #[doc = "- [digital_root_base](#method.digital_root_base" $d ")"]
        ///
        /// Each method calls its specific inner primitive implementation.
        /// # Errors
        /// Every method can return [`Invalid`] if the result is invalid for the niche type.
        impl<$(const $g:$t,)*> Int<[<$n$t:camel>]<$($g,)*>> {
            impl_niche![Int $n:$t:$dt<$($g),*>, +const digits, self];
            impl_niche![Int $n:$t:$dt<$($g),*>, +const digits_sign, self];
            impl_niche![Int $n:$t:$dt<$($g),*>, +const digits_base, self, base:$t];
            impl_niche![Int $n:$t:$dt<$($g),*>, +const digits_base_sign, self, base:$t];
            impl_niche![Int $n:$t:$dt<$($g),*>, +const digital_root, self];
            impl_niche![Int $n:$t:$dt<$($g),*>, +const digital_root_base, self, base:$t];
        }
    }};
}
impl_base![prim_signed i8:"", i16:"-1", i32:"-2", i64:"-3", i128:"-4", isize:"-5"];
impl_base![prim_unsigned u8:"-6", u16:"-7", u32:"-8", u64:"-9", u128:"-10", usize:"-11"];
#[cfg(feature = "num_int_niche")]
impl_base![niche
    NonZero:i8<>:"-18":"", NonZero:i16<>:"-19":"-1",
    NonZero:i32<>:"-20":"-2", NonZero:i64<>:"-21":"-3",
    NonZero:i128<>:"-22":"-4", NonZero:isize<>:"-23":"-5",
    NonZero:u8<>:"-12":"-6", NonZero:u16<>:"-13":"-7",
    NonZero:u32<>:"-14":"-8", NonZero:u64<>:"-15":"-9",
    NonZero:u128<>:"-16":"-10", NonZero:usize<>:"-17":"-11",
    //
    NonSpecific:i8<V>:"-30":"", NonSpecific:i16<V>:"-31":"-1",
    NonSpecific:i32<V>:"-32":"-2", NonSpecific:i64<V>:"-33":"-3",
    NonSpecific:i128<V>:"-34":"-4", NonSpecific:isize<V>:"-35":"-5",
    NonSpecific:u8<V>:"-24":"-6", NonSpecific:u16<V>:"-25":"-7",
    NonSpecific:u32<V>:"-26":"-8", NonSpecific:u64<V>:"-27":"-9",
    NonSpecific:u128<V>:"-28":"-10", NonSpecific:usize<V>:"-29":"-11",
    //
    NonRange:i8<RMIN,RMAX>:"-42":"", NonRange:i16<RMIN,RMAX>:"-43":"-1",
    NonRange:i32<RMIN,RMAX>:"-44":"-2", NonRange:i64<RMIN,RMAX>:"-45":"-3",
    NonRange:i128<RMIN,RMAX>:"-46":"-4", NonRange:isize<RMIN,RMAX>:"-47":"-5",
    NonRange:u8<RMIN,RMAX>:"-36":"-6", NonRange:u16<RMIN,RMAX>:"-37":"-7",
    NonRange:u32<RMIN,RMAX>:"-38":"-8", NonRange:u64<RMIN,RMAX>:"-39":"-9",
    NonRange:u128<RMIN,RMAX>:"-40":"-10", NonRange:usize<RMIN,RMAX>:"-41":"11",
    //
    Range:i8<RMIN,RMAX>:"-54":"", Range:i16<RMIN,RMAX>:"-55":"-1",
    Range:i32<RMIN,RMAX>:"-56":"-2", Range:i64<RMIN,RMAX>:"-57":"-3",
    Range:i128<RMIN,RMAX>:"-58":"-4", Range:isize<RMIN,RMAX>:"-59":"-5",
    Range:u8<RMIN,RMAX>:"-48":"-6", Range:u16<RMIN,RMAX>:"-49":"-7",
    Range:u32<RMIN,RMAX>:"-50":"-8", Range:u64<RMIN,RMAX>:"-51":"-9",
    Range:u128<RMIN,RMAX>:"-52":"-10", Range:usize<RMIN,RMAX>:"-53":"-11",
];
