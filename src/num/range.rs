// devela::num::range
//
//! Creates const generic customizable wrappers over the `NonZero` primitives.
//

use crate::{codegen::paste, iif};
use core::{fmt, num::*, str::FromStr};

#[cfg(all(feature = "bytemuck", not(feature = "safe")))]
use bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};

macro_rules! impl_range {
    // Entry point, generates Range structures for each sign and size.
    ($name:ident) => {
        impl_range![Range, i, I, 8, 16, 32, 64, 128, size];
        impl_range![Range, u, U, 8, 16, 32, 64, 128, size];
    };
    ($name:ident, $s:ident, $S:ident, $( $b:expr ),+) => {
        $( impl_range![@Range, $s, $S, $b]; )+
    };

    // $name: the base name of the new type. E.g. Range.
    // $s: the sign identifier, lowercase: i or u.
    // $S: the sign identifier, uppercase: I or U.
    // $b: the bits of the type, from 8 to 128, or the `size` suffix.
    (@$name:ident, $s:ident, $S:ident, $b:expr) => { paste! {
        /* definition */

        /// An integer that is known to be included in some inclusive range.
        ///
        /// It has an optimized memory layout, so that
        #[doc = "`Option<"[<$name $S $b>]">` is the same size as `"[<$name $S $b>]"`."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::num::" [<$name $S $b>] ";"]
        ///
        #[doc = "assert![" [<$name $S $b>] "::<5, 25>::new(4).is_none()];"]
        #[doc = "assert![" [<$name $S $b>] "::<5, 25>::new(26).is_none()];"]
        #[doc = "assert![" [<$name $S $b>] "::<5, 25>::new(5).unwrap().get() == 5];"]
        #[doc = "assert![" [<$name $S $b>] "::<5, 25>::new(25).unwrap().get() == 25];"]
        ///
        #[doc = "assert![" [<$name $S $b>] "::<5, 25>::VALID_VALUES == 21];"]
        #[doc = "// Self::INVALID_VALUES + Self::VALID_VALUES == "[<u $b>]"::MAX + 1."]
        #[doc = "assert![" [<$name $S $b>] "::<5, 25>::INVALID_VALUES == "
           [<u $b>]"::MAX - 21 + 1];"]
        /// ```
        ///
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(C)]
        pub struct [<$name $S $b>]<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            ([<NonZero $S $b>]);

        /* methods */

        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]> [<$name $S $b>]<RMIN, RMAX> {
            // The constant value used to shift the range of values away from 0,
            pub const XOR_VALUE: [<$s $b>] = RMIN.wrapping_sub(1);

            #[doc = "Returns a `" [<$name $S $b>] "` with the given `value`,"
            " only if it's between `RMIN` and `RMAX`."]
            ///
            /// Returns `None` if `value` is not between `RMIN` and `RMAX`, inclusive,
            /// or if `RMIN > RMAX`.
            pub const fn new(value: [<$s $b>]) -> Option<Self> {
                if RMIN <= value && value <= RMAX {
                    match [<NonZero $S $b>]::new(value ^ Self::XOR_VALUE) {
                        None => None,
                        Some(v) => Some(Self(v)),
                    }
                } else {
                    None
                }
            }

            #[doc = "Returns a `" [<$name $S $b>]
            "` if the given value is between `RMIN` and `RMAX`."]
            ///
            /// # Panics
            /// Panics in debug if the given `value` is **not** between `RMIN` and
            /// `RMAX`, inclusive, or if `RMIN > RMAX`.
            /// # Safety
            /// The given `value` must be between `RMIN` and `RMAX`, inclusive.
            #[cfg(feature = "unsafe_num")]
            #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_num")))]
            pub const unsafe fn new_unchecked(value: [<$s $b>]) -> Self {
                #[cfg(debug_assertions)]
                if RMIN > RMAX {
                    panic!("RMIN must be less or equal than RMAX.")
                }
                #[cfg(debug_assertions)]
                if value < RMIN || value > RMAX {
                    panic!("The given value was outside the given range.")
                }
                Self([<NonZero $S $b>]::new_unchecked(value ^ Self::XOR_VALUE))
            }

            /// Returns the value as a primitive type.
            #[inline]
            pub const fn get(&self) -> [<$s $b>] {
                self.0.get() ^ Self::XOR_VALUE
            }

            /// Returns the number of valid values within the range from `RMIN` to `RMAX`,
            /// inclusive, as an unsigned integer with equal bit size.
            ///
            /// # Notice
            /// A range where `RMAX == RMIN` will result in `VALID_VALUES == 1`, as expected.
            ///
            #[doc = "But a range from [`"[<$s $b>]"::MIN`] to [`"[<$s $b>]"::MAX`] will result"
            " in `VALID_VALUES == 0`, instead of [`"[<u $b>]"::MAX`]` + 1`,"]
            /// This is because the maximum number of representable values for a given
            /// bit-size can't be represented using the same number of bits.
            /// In this case `INVALID_VALUES` will also be `== 0`.
            ///
            #[doc = "This doesn't matter as much because a [`"[<$name $S $b>]"`] with"
            " the largest range of values is really just a [`" [<$s $b>] "`]."]
            ///
            #[doc = "Remember that `INVALID_VALUES + VALID_VALUES == "[<u $b>]"::MAX + 1`,"]
            /// which would wrap to `0`.
            pub const VALID_VALUES: [<u $b>] = {
                iif![RMIN > RMAX; panic!("RMIN must be less or equal than RMAX.")];
                RMAX.wrapping_sub(RMIN).wrapping_add(1) as [<u $b>]
            };

            /// Returns the number of invalid values outside the range from `RMIN` to `RMAX`,
            /// inclusive, as an unsigned integer with equal bit size.
            ///
            /// # Notice
            /// A range where `RMAX == RMIN` will result in `INVALID_VALUES ==
            #[doc = "`[`" [<u $b>]"::MAX`], as expected."]
            ///
            #[doc = "A range from [`"[<$s $b>]"::MIN`] to [`"[<$s $b>]"::MAX`] will result"]
            /// in `INVALID_VALUES == 0`, also as expected.
            /// Just be aware that in this case `VALID_VALUES` will also be `== 0` instead of
            #[doc = " [`" [<u $b>]"::MAX`]` + 1`."]
            pub const INVALID_VALUES: [<u $b>] = ([<u $b>]::MAX - Self::VALID_VALUES)
                .wrapping_add(1);
        }

        /* core impls */

        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            fmt::Display for [<$name $S $b>]<RMIN, RMAX> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.get())
            }
        }
        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            fmt::Debug for [<$name $S $b>]<RMIN, RMAX> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}::<{}, {}>({})", stringify!([<$name $S $b>]), RMIN, RMAX, self.get())
            }
        }
        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            fmt::Binary for [<$name $S $b>]<RMIN, RMAX> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Binary::fmt(&self.get(), f)
            }
        }
        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            fmt::Octal for [<$name $S $b>]<RMIN, RMAX> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Octal::fmt(&self.get(), f)
            }
        }
        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            fmt::LowerHex for [<$name $S $b>]<RMIN, RMAX> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::LowerHex::fmt(&self.get(), f)
            }
        }
        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            fmt::UpperHex for [<$name $S $b>]<RMIN, RMAX> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::UpperHex::fmt(&self.get(), f)
            }
        }

        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            FromStr for [<$name $S $b>]<RMIN, RMAX> {
            type Err = ParseIntError;
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::new([<$s $b>]::from_str(s)?).ok_or_else(||"".parse::<i32>().unwrap_err())
            }
        }

        /* conversions */

        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            From<[<$name $S $b>]<RMIN, RMAX>> for [<$s $b>] {
            #[inline]
            fn from(value: [<$name $S $b>]<RMIN, RMAX>) -> [<$s $b>] {
                value.get()
            }
        }

        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            TryFrom<[<$s $b>]> for [<$name $S $b>]<RMIN, RMAX> {
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
        unsafe impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            ZeroableInOption for [<$name $S $b>]<RMIN, RMAX> {}

        #[cfg(all(feature = "bytemuck", feature = "unsafe_num"))]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = "bytemuck", feature = "unsafe_num"))))]
        unsafe impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            PodInOption for [<$name $S $b>]<RMIN, RMAX> {}

        #[cfg(all(feature = "bytemuck", feature = "unsafe_num"))]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = "bytemuck", feature = "unsafe_num"))))]
        unsafe impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            NoUninit for [<$name $S $b>]<RMIN, RMAX> {}

        #[cfg(all(feature = "bytemuck", feature = "unsafe_num"))]
        #[cfg_attr(feature = "nightly",
            doc(cfg(all(feature = "bytemuck", feature = "unsafe_num"))))]
        unsafe impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            CheckedBitPattern for [<$name $S $b>]<RMIN, RMAX> {
            type Bits = [<$s $b>];

            #[inline]
            fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
                // check if it's within the range after shifting the bit value
                let value = bits ^ Self::XOR_VALUE;
                RMIN <= value && value <= RMAX
            }
        }
    }};
}
impl_range![Range];
