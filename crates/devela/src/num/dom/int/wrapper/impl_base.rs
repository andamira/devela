//
/// Implements base-related methods for [`Int`].
//
// TOC
// - signed|unsigned:
//   - digits
//   - digits_sign
//   - digits_base
//   - digits_base_sign
//   - digital_root
//   - digital_root_base
use crate::{Int, paste};

/// Implements base-related methods for [`Int`].
///
/// # Args
/// $t:   the integer primitive input/output type, and the niche inner type
///
/// $d:   the doclink suffix for the method name
macro_rules! __impl_int_base {
    () => {
        __impl_int_base![signed
            i8    |"",
            i16   |"-1",
            i32   |"-2",
            i64   |"-3",
            i128  |"-4",
            isize |"-5"
        ];
        __impl_int_base![unsigned
            u8    |"-6",
            u16   |"-7",
            u32   |"-8",
            u64   |"-9",
            u128  |"-10",
            usize |"-11"
        ];
    };
    (signed $( $t:ty | $d:literal ),+) => {
        $( __impl_int_base![@signed $t| $d]; )+
    };
    (unsigned $( $t:ty | $d:literal ),+) => {
        $( __impl_int_base![@unsigned $t| $d]; )+
    };
    (
    // implements ops on signed primitives
    @signed $t:ty | $d:literal) => { paste! {
        ///
        #[doc = "# Integer base related methods for `" $t "`\n\n"]
        #[doc = "- [digits](#method.digits" $d ")"]
        #[doc = "- [digits_sign](#method.digits_sign" $d ")"]
        #[doc = "- [digits_base](#method.digits_base" $d ")"]
        #[doc = "- [digits_base_sign](#method.digits_base_sign" $d ")"]
        #[doc = "- [digital_root](#method.digital_root" $d ")"]
        #[doc = "- [digital_root_base](#method.digital_root_base" $d ")"]
        ///
        impl Int<$t> {
            /// Returns the number of digits in base 10.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits()];"]
            #[doc = "assert_eq![Int(1), Int(-1_" $t ").digits()];"]
            #[doc = "assert_eq![Int(3), Int(127_" $t ").digits()];"]
            #[doc = "assert_eq![Int(3), Int(-128_" $t ").digits()];"]
            /// ```
            pub const fn digits(self) -> Int<$t> {
                Int(crate::__int_base_alg!(signed digits $t; self.0))
            }

            /// Returns the number of digits in base 10, including the negative sign.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits_sign()];"]
            #[doc = "assert_eq![Int(2), Int(-1_" $t ").digits_sign()];"]
            #[doc = "assert_eq![Int(3), Int(127_" $t ").digits_sign()];"]
            #[doc = "assert_eq![Int(4), Int(-128_" $t ").digits_sign()];"]
            /// ```
            pub const fn digits_sign(self) -> Int<$t> {
                Int(crate::__int_base_alg!(signed digits_sign $t; self.0))
            }

            /// Returns the number of digits in the given absolute `base`.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(2), Int(3_" $t ").digits_base(2)];"]
            #[doc = "assert_eq![Int(2), Int(127_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![Int(2), Int(-128_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![Int(2), Int(-128_" $t ").digits_base(-16)];"]
            #[doc = "assert_eq![Int(0), Int(100_" $t ").digits_base(0)];"]
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits_base(100)];"]
            /// ```
            pub const fn digits_base(self, base: $t) -> Int<$t> {
                Int(crate::__int_base_alg!(signed digits_base $t; self.0, base))
            }

            /// Returns the number of digits in the given absolute `base`,
            /// including the negative sign.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(2), Int(3_" $t ").digits_base_sign(2)];"]
            #[doc = "assert_eq![Int(2), Int(127_" $t ").digits_base_sign(16)];"]
            #[doc = "assert_eq![Int(3), Int(-128_" $t ").digits_base_sign(16)];"]
            #[doc = "assert_eq![Int(3), Int(-128_" $t ").digits_base_sign(-16)];"]
            #[doc = "assert_eq![Int(0), Int(100_" $t ").digits_base_sign(0)];"]
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits_base_sign(100)];"]
            /// ```
            pub const fn digits_base_sign(self, base: $t) -> Int<$t> {
                Int(crate::__int_base_alg!(signed digits_base_sign $t; self.0, base))
            }

            /* signed digital_root */

            /// Returns the digital root in base 10.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root()];"]
            #[doc = "assert_eq![Int(1), Int(-127_" $t ").digital_root()];"]
            #[doc = "assert_eq![Int(9), Int(126_" $t ").digital_root()];"]
            /// ```
            pub const fn digital_root(self) -> Int<$t> {
                Int(crate::__int_base_alg!(signed digital_root $t; self.0))
            }

            /// Returns the digital root in in the given absolute `base`.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root_base(-10)];"]
            #[doc = "assert_eq![Int(1), Int(-127_" $t ").digital_root_base(-10)];"]
            #[doc = "assert_eq![Int(9), Int(-126_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![Int(3), Int(-33_" $t ").digital_root_base(16)];"]
            /// ```
            pub const fn digital_root_base(self, base: $t) -> Int<$t> {
                Int(crate::__int_base_alg!(signed digital_root_base $t; self.0, base))
            }
        }
    }};
    (
    // implements ops on unsigned primitives
    @unsigned $t:ty | $d:literal) => { paste! {
        ///
        #[doc = "# Integer base related methods for `" $t "`\n\n"]
        #[doc = "- [digits](#method.digits" $d ")"]
        #[doc = "- [digits_sign](#method.digits_sign" $d ")"]
        #[doc = "- [digits_base](#method.digits_base" $d ")"]
        #[doc = "- [digits_base_sign](#method.digits_base_sign" $d ")"]
        #[doc = "- [digital_root](#method.digital_root" $d ")"]
        #[doc = "- [digital_root_base](#method.digital_root_base" $d ")"]
        ///
        impl Int<$t> {
            /* unsigned digits */

            /// Returns the number of digits in base 10.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits()];"]
            #[doc = "assert_eq![Int(3), Int(127_" $t ").digits()];"]
            /// ```
            pub const fn digits(self) -> Int<$t> {
                Int(crate::__int_base_alg!(unsigned digits $t; self.0))
            }

            /// An alias of [`digits`][Self#digits].
            pub const fn digits_sign(self) -> Int<$t> { self.digits() }

            /// Returns the number of digits in the given `base`.
            ///
            /// If the base is 0, it returns 0.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(2), Int(3_" $t ").digits_base(2)];"]
            #[doc = "assert_eq![Int(2), Int(127_" $t ").digits_base(16)];"]
            #[doc = "assert_eq![Int(0), Int(100_" $t ").digits_base(0)];"]
            #[doc = "assert_eq![Int(1), Int(0_" $t ").digits_base(100)];"]
            /// ```
            pub const fn digits_base(self, base: $t) -> Int<$t> {
                Int(crate::__int_base_alg!(unsigned digits_base $t; self.0, base))
            }

            /// An alias of [`digits_base`][Self#digits_base].
            pub const fn digits_base_sign(self, base: $t) -> Int<$t> { self.digits_base(base) }

            /* unsigned digital_root */

            /// Returns the digital root in base 10.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root()];"]
            #[doc = "assert_eq![Int(9), Int(126_" $t ").digital_root()];"]
            /// ```
            pub const fn digital_root(self) -> Int<$t> {
                Int(crate::__int_base_alg!(unsigned digital_root $t; self.0))
            }

            /// Returns the digital root in in the given absolute `base`.
            /// # Examples
            /// ```
            /// use devela::Int;
            ///
            #[doc = "assert_eq![Int(1), Int(127_" $t ").digital_root_base(10)];"]
            #[doc = "assert_eq![Int(3), Int(33_" $t ").digital_root_base(16)];"]
            /// ```
            pub const fn digital_root_base(self, base: $t) -> Int<$t> {
                Int(crate::__int_base_alg!(unsigned digital_root_base $t; self.0, base))
            }
        }
    }};
}
__impl_int_base!();
