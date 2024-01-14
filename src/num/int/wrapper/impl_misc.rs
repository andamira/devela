// devela::num::int::wrapper::impl_misc
//
//! implements miscellaneous functions
//
// TOC
// - signed|unsigned:
//   - is_even
//   - is_odd

use super::Int;
use crate::code::paste;

// $t:   the input/output type
// $dl:  the doclink suffix for the method name
macro_rules! impl_base {
    (signed $( $t:ty : $dl:literal ),+) => { $( impl_base![@signed $t:$dl]; )+ };
    (unsigned $( $t:ty : $dl:literal ),+) => { $( impl_base![@unsigned $t:$dl]; )+ };

    // implements signed ops
    (@signed $t:ty : $dl:literal) => { paste! {
        #[doc = "# Integer miscellaneous methods for `" $t "`\n\n"]
        #[doc = "- [is_even](#method.is_even" $dl ")"]
        #[doc = "- [is_odd](#method.is_odd" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /// Returns `true` if `self` is an even number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(2_" $t ").is_even()];"]
            #[doc = "assert![Int(-2_" $t ").is_even()];"]
            #[doc = "assert![!Int(3_" $t ").is_even()];"]
            #[doc = "assert![Int(0_" $t ").is_even()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_even(self) -> bool { self.0 & 1 == 0 }

            /// Returns `true` if `self` is an odd number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(3_" $t ").is_odd()];"]
            #[doc = "assert![Int(-3_" $t ").is_odd()];"]
            #[doc = "assert![!Int(2_" $t ").is_odd()];"]
            #[doc = "assert![!Int(0_" $t ").is_odd()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_odd(self) -> bool { self.0 & 1 == 1 }
        }
    }};

    // implements unsigned ops
    (@unsigned $t:ty : $dl:literal) => { paste! {
        #[doc = "# Integer miscellaneous methods for `" $t "`\n\n"]
        #[doc = "- [is_even](#method.is_even" $dl ")"]
        #[doc = "- [is_odd](#method.is_odd" $dl ")"]
        ///
        /// See the related trait [`NumInt`][crate::num::NumInt].
        impl Int<$t> {
            /// Returns `true` if `self` is an even number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(2_" $t ").is_even()];"]
            #[doc = "assert![!Int(3_" $t ").is_even()];"]
            #[doc = "assert![Int(0_" $t ").is_even()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_even(self) -> bool { self.0 & 1 == 0 }

            /// Returns `true` if `self` is an odd number.
            /// # Examples
            /// ```
            /// # use devela::num::Int;
            #[doc = "assert![Int(3_" $t ").is_odd()];"]
            #[doc = "assert![!Int(2_" $t ").is_odd()];"]
            #[doc = "assert![!Int(0_" $t ").is_odd()];"]
            /// ```
            #[inline] #[must_use]
            pub const fn is_odd(self) -> bool { self.0 & 1 == 1 }
        }
    }};
}
impl_base![signed i8:"", i16:"-1", i32:"-2", i64:"-3", i128:"-4", isize:"-5"];
impl_base![unsigned u8:"-6", u16:"-7", u32:"-8", u64:"-9", u128:"-10", usize:"-11"];
