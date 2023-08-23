// devela::num::non_specific
//
//! Creates const generic customizable wrappers over the `NonZero` primitives.
//

use crate::codegen::paste;
use core::{fmt, num::*, str::FromStr};

#[cfg(all(feature = "bytemuck", not(feature = "safe")))]
use bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};

macro_rules! impl_non_specific {
    // Entry point, generates NonSpecific structures for each sign and size.
    ($name:ident) => {
        impl_non_specific![NonSpecific, i, I, 8, 16, 32, 64, 128, size];
        impl_non_specific![NonSpecific, u, U, 8, 16, 32, 64, 128, size];
    };
    ($name:ident, $s:ident, $S:ident, $( $b:expr ),+) => {
        $( impl_non_specific![@NonSpecific, $s, $S, $b]; )+
    };

    // $name: the base name of the new type. E.g. NonSpecific.
    // $s: the sign identifier, lowercase: i or u.
    // $S: the sign identifier, uppercase: I or U.
    // $b: the bits of the type, from 8 to 128, or the `size` suffix.
    (@$name:ident, $s:ident, $S:ident, $b:expr) => { paste! {
        /* definition */

        /// An integer that is known not to equal some specific value.
        ///
        /// It has an optimized memory layout, so that
        #[doc = "`Option<"[<$name $S $b>]">` is the same size as `"[<$name $S $b>]"`."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::num::" [<$name $S $b>] ";"]
        ///
        #[doc = "assert![" [<$name $S $b>] "::<13>::new(13).is_none()];"]
        #[doc = "assert![" [<$name $S $b>] "::<13>::new(12).unwrap().get() == 12];"]
        /// ```
        ///
        #[doc = "See also [`NonMax" $S $b "`] and [`NonMin" $S $b "`]."]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(C)]
        pub struct [<$name $S $b>]<const V: [<$s $b>]>([<NonZero $S $b>]);

        /* aliases */

        /// An integer that is known not to equal its maximum value
        #[doc = "[`" [< $s $b >] "::MAX`]."]
        pub type [<NonMax $S $b>] = [<$name $S $b>]<{[<$s $b>]::MAX}>;

        impl Default for [<NonMax $S $b>] {
            #[inline]
            fn default() -> Self {
                #[cfg(not(feature = "unsafe_num"))]
                return [<NonMax $S $b>]::new([<$s $b>]::default()).unwrap();

                #[cfg(feature = "unsafe_num")]
                // SAFETY: the default numeric primitive values is always 0,
                // and their maximum value is never 0.
                unsafe { return [<NonMax $S $b>]::new_unchecked([<$s $b>]::default()); }
            }
        }

        /// An integer that is known not to equal its minimum value
        #[doc = "[`" [< $s $b >] "::MIN`]."]
        pub type [<NonMin $S $b>] = [<$name $S $b>]<{[<$s $b>]::MIN}>;

        /* methods */

        impl<const V: [<$s $b>]> [<$name $S $b>]<V> {
            #[doc = "Returns a `" [<$name $S $b>] "` with the given `value`,"
                " if it is not equal to `V`."]
            pub const fn new(value: [<$s $b>]) -> Option<Self> {
                // [<NonZero $S $b>]::new(value ^ V).map(Self) // non-const
                match [<NonZero $S $b>]::new(value ^ V) {
                    None => None,
                    Some(v) => Some(Self(v)),
                }
            }

            #[doc = "Returns a `" [<$name $S $b>] "` if the given `value`"
                " if it is not equal to `V`."]
            ///
            /// # Panics
            /// Panics in debug if the given `value` is equal to `V`.
            /// # Safety
            /// The given `value` must never be equal to `V`.
            #[cfg(feature = "unsafe_num")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_num")))]
            pub const unsafe fn new_unchecked(value: [<$s $b>]) -> Self {
                // debug_assert_ne![value, V]; // non-const
                #[cfg(debug_assertions)]
                if value == V { panic!("The given value was specifically prohibited.") }

                Self([<NonZero $S $b>]::new_unchecked(value ^ V))
            }

            /// Returns the value as a primitive type.
            #[inline]
            pub const fn get(&self) -> [<$s $b>] {
                self.0.get() ^ V
            }

            /// Returns the number of valid values.
            pub const VALID_VALUES: [<u $b>] = [<u $b>]::MAX;

            /// Returns the number of invalid values.
            pub const INVALID_VALUES: [<u $b>] = 1;
        }

        /* core impls */

        impl<const V: [<$s $b>]> fmt::Display for [<$name $S $b>]<V> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.get())
            }
        }
        impl<const V: [<$s $b>]> fmt::Debug for [<$name $S $b>]<V> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}::<{}>({})", stringify!([<$name $S $b>]), V, self.get())
            }
        }
        impl<const V: [<$s $b>]> fmt::Binary for [<$name $S $b>]<V> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Binary::fmt(&self.get(), f)
            }
        }
        impl<const V: [<$s $b>]> fmt::Octal for [<$name $S $b>]<V> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Octal::fmt(&self.get(), f)
            }
        }
        impl<const V: [<$s $b>]> fmt::LowerHex for [<$name $S $b>]<V> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::LowerHex::fmt(&self.get(), f)
            }
        }
        impl<const V: [<$s $b>]> fmt::UpperHex for [<$name $S $b>]<V> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::UpperHex::fmt(&self.get(), f)
            }
        }

        impl<const V: [<$s $b>]> FromStr for [<$name $S $b>]<V> {
            type Err = ParseIntError;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new([<$s $b>]::from_str(s)?).ok_or_else(||"".parse::<i32>().unwrap_err())
            }
        }

        /* conversions */

        impl<const V: [<$s $b>]> From<[<$name $S $b>]<V>> for [<$s $b>] {
            #[inline]
            fn from(value: [<$name $S $b>]<V>) -> [<$s $b>] {
                value.get()
            }
        }

        impl<const V: [<$s $b>]> TryFrom<[<$s $b>]> for [<$name $S $b>]<V> {
            type Error = core::num::TryFromIntError;

            #[inline]
            fn try_from(value: [<$s $b>]) -> Result<Self, Self::Error> {
                // We generate a TryFromIntError by intentionally causing a failed conversion.
                #[cfg(not(feature = "unsafe_num"))]
                return Self::new(value).ok_or_else(|| i8::try_from(255_u8).unwrap_err());
                #[cfg(feature = "unsafe_num")]
                return Self::new(value)
                    .ok_or_else(|| unsafe { i8::try_from(255_u8).unwrap_err_unchecked() });
            }
        }

        /* external impls*/

        #[cfg(all(feature = "bytemuck", feature = "unsafe_num"))]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = "bytemuck", feature = "unsafe_num"))))]
        unsafe impl<const V: [<$s $b>]> ZeroableInOption for [<$name $S $b>]<V> {}

        #[cfg(all(feature = "bytemuck", feature = "unsafe_num"))]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = "bytemuck", feature = "unsafe_num"))))]
        unsafe impl<const V: [<$s $b>]> PodInOption for [<$name $S $b>]<V> {}

        #[cfg(all(feature = "bytemuck", feature = "unsafe_num"))]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = "bytemuck", feature = "unsafe_num"))))]
        unsafe impl<const V: [<$s $b>]> NoUninit for [<$name $S $b>]<V> {}

        #[cfg(all(feature = "bytemuck", feature = "unsafe_num"))]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = "bytemuck", feature = "unsafe_num"))))]
        unsafe impl<const V: [<$s $b>]> CheckedBitPattern for [<$name $S $b>]<V> {
            type Bits = [<$s $b>];

            #[inline]
            fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
                // Since inner repr is NonZero, 0 is the only invalid bit pattern
                *bits != 0
            }
        }
    }};
}
impl_non_specific![NonSpecific];
