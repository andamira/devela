// devela::num::niche::in_range
//
//! Creates const generic customizable wrappers over the `NonZero` numbers,
//! that represents a numeric range.
//

#[cfg(all(feature = "dep_bytemuck", feature = "unsafe_niche", not(feature = "safe_num")))]
use crate::_dep::bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};
#[cfg(feature = "mem_bit")]
use crate::mem::{bit_sized, ByteSized};
use crate::{
    _dep::_core::{fmt, num::*, str::FromStr},
    code::{iif, paste},
};

macro_rules! impl_in_range {
    // Entry point, generates InRange structures for each sign and size.
    () => {
        impl_in_range!["A signed", i,
            8:"_in_range_i8", 16:"_in_range_i16", 32:"_in_range_i32",
            64:"_in_range_i64", 128:"_in_range_i128", size:"_in_range_isize"];
        impl_in_range!["An unsigned", u,
            8:"_in_range_u8", 16:"_in_range_u16", 32:"_in_range_u32",
            64:"_in_range_u64", 128:"_in_range_u128", size:"_in_range_usize"];
    };
    ($doc:literal, $s:ident, $( $b:tt : $cap:literal),+) => {
        $( paste!{
            impl_in_range![@
                [<InRange $s:upper $b>], // $name
                $doc,
                [<$s $b>], // $IP
                $s,
                $b:
                $cap
            ];
        })+
    };

    // $name: the full name of the new type. E.g. InRange.
    // $doc:  the specific beginning of the documentation.
    // $IP:   the type of the corresponding integer primitive. E.g. i8
    // $s:    the sign identifier, lowercase: i or u.
    // $b:    the bits of the type, from 8 to 128, or the `size` suffix.
    // $cap:  the capability feature that enables the given implementation. E.g "_in_range_i8".
    (@$name:ident, $doc:literal, $IP:ty, $s:ident, $b:tt : $cap:literal) => { paste! {
        /* definition */

        #[doc = $doc " integer that is known to be included in some inclusive range." ]
        ///
        #[doc = "It has the same memory layout optimization as [`NonZero" $s:upper $b "`],"]
        #[doc = " so that `Option<" $name ">` is the same size as `" $name "`."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::num::" $name ";"]
        ///
        #[doc = "assert![" $name "::<5, 25>::new(4).is_none()];"]
        #[doc = "assert![" $name "::<5, 25>::new(26).is_none()];"]
        #[doc = "assert![" $name "::<5, 25>::new(5).unwrap().get() == 5];"]
        #[doc = "assert![" $name "::<5, 25>::new(25).unwrap().get() == 25];"]
        ///
        #[doc = "assert![" $name "::<5, 25>::VALID_VALUES == 21];"]
        #[doc = "// Self::INVALID_VALUES + Self::VALID_VALUES == "[<u $b>]"::MAX + 1."]
        #[doc = "assert![" $name "::<5, 25>::INVALID_VALUES == "
           [<u $b>]"::MAX - 21 + 1];"]
        /// ```
        ///
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        pub struct $name<const RMIN: $IP, const RMAX: $IP>
            ([<NonZero $s:upper $b>]);

        /* methods */

        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<const RMIN: $IP, const RMAX: $IP> $name<RMIN, RMAX> {
            // The constant value used to shift the range of values away from 0,
            const XOR_VALUE: $IP = RMIN.wrapping_sub(1);

            #[doc = "Returns a `" $name "` with the given `value`,"
            " only if it's between `RMIN` and `RMAX`."]
            ///
            /// Returns `None` if `value` is not between `RMIN` and `RMAX`, inclusive,
            /// or if `RMIN > RMAX`.
            #[must_use]
            pub const fn new(value: $IP) -> Option<Self> {
                if RMIN <= value && value <= RMAX {
                    match [<NonZero $s:upper $b>]::new(value ^ Self::XOR_VALUE) {
                        None => None,
                        Some(v) => Some(Self(v)),
                    }
                } else {
                    None
                }
            }

            #[doc = "Returns a `" $name "` if the given value is between `RMIN` and `RMAX`."]
            ///
            /// # Panics
            /// Panics in debug if the given `value` is **not** between `RMIN` and
            /// `RMAX`, inclusive, or if `RMIN > RMAX`.
            /// # Safety
            /// The given `value` must be between `RMIN` and `RMAX`, inclusive.
            #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            pub const unsafe fn new_unchecked(value: $IP) -> Self {
                #[cfg(debug_assertions)]
                if RMIN > RMAX {
                    panic!("RMIN must be less or equal than RMAX.")
                }
                #[cfg(debug_assertions)]
                if value < RMIN || value > RMAX {
                    panic!("The given value was outside the given range.")
                }
                // SAFETY: caller must ensure safety
                Self(unsafe { [<NonZero $s:upper $b>]::new_unchecked(value ^ Self::XOR_VALUE) })
            }

            /// Returns the value as a primitive type.
            #[inline]
            #[must_use]
            pub const fn get(&self) -> $IP {
                self.0.get() ^ Self::XOR_VALUE
            }

            /// Returns the number of valid values within the range from `RMIN` to `RMAX`,
            /// inclusive, as an unsigned integer with equal bit size.
            ///
            /// # Notice
            /// A range where `RMAX == RMIN` will result in `VALID_VALUES == 1`, as expected.
            ///
            #[doc = "But a range from [`" $IP "::MIN`] to [`" $IP "::MAX`] will result"
            " in `VALID_VALUES == 0`, instead of [`"[<u $b>]"::MAX`]` + 1`,"]
            /// This is because the maximum number of representable values for a given
            /// bit-size can't be represented using the same number of bits.
            /// In this case `INVALID_VALUES` will also be `== 0`.
            ///
            #[doc = "This doesn't matter as much because a [`" $name "`] with"
            " the largest range of values is really just a [`" $IP "`]."]
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
            #[doc = "A range from [`" $IP "::MIN`] to [`" $IP "::MAX`] will result"]
            /// in `INVALID_VALUES == 0`, also as expected.
            /// Just be aware that in this case `VALID_VALUES` will also be `== 0` instead of
            #[doc = " [`" [<u $b>]"::MAX`]` + 1`."]
            pub const INVALID_VALUES: [<u $b>] = ([<u $b>]::MAX - Self::VALID_VALUES)
                .wrapping_add(1);
        }

        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        mod [<extra_impls_ $s $b >] {
            use super::*;

            /* core impls */

            impl<const RMIN: $IP, const RMAX: $IP>
                fmt::Display for $name<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}", self.get())
                }
            }
            impl<const RMIN: $IP, const RMAX: $IP>
                fmt::Debug for $name<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}::<{}, {}>({})",
                        stringify!($name), RMIN, RMAX, self.get())
                }
            }
            impl<const RMIN: $IP, const RMAX: $IP>
                fmt::Binary for $name<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Binary::fmt(&self.get(), f)
                }
            }
            impl<const RMIN: $IP, const RMAX: $IP>
                fmt::Octal for $name<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Octal::fmt(&self.get(), f)
                }
            }
            impl<const RMIN: $IP, const RMAX: $IP>
                fmt::LowerHex for $name<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::LowerHex::fmt(&self.get(), f)
                }
            }
            impl<const RMIN: $IP, const RMAX: $IP>
                fmt::UpperHex for $name<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::UpperHex::fmt(&self.get(), f)
                }
            }

            impl<const RMIN: $IP, const RMAX: $IP>
                FromStr for $name<RMIN, RMAX> {
                type Err = ParseIntError;
                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::new($IP::from_str(s)?).ok_or_else(||"".parse::<i32>().unwrap_err())
                }
            }

            /* conversions */

            impl<const RMIN: $IP, const RMAX: $IP>
                From<$name<RMIN, RMAX>> for $IP {
                #[inline]
                #[must_use]
                fn from(value: $name<RMIN, RMAX>) -> $IP {
                    value.get()
                }
            }

            impl<const RMIN: $IP, const RMAX: $IP>
                TryFrom<$IP> for $name<RMIN, RMAX> {
                type Error = core::num::TryFromIntError;

                /// # Features
                /// Makes use of the `unsafe_niche` feature if enabled.
                #[inline]
                fn try_from(value: $IP) -> Result<Self, Self::Error> {
                    // We generate a TryFromIntError by intentionally causing a failed conversion.
                    #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                    return Self::new(value).ok_or_else(|| i8::try_from(255_u8).unwrap_err());

                    #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                    return Self::new(value)
                        .ok_or_else(|| unsafe { i8::try_from(255_u8).unwrap_err_unchecked() });
                }
            }

            /* internal impls */

            // BitSized
            #[cfg(feature = "mem_bit")]
            bit_sized![<const RMIN: $IP, const RMAX: $IP> =
                { $IP::BYTE_SIZE * 8}; for $name<RMIN, RMAX>];

            /* external impls */

            #[cfg(all(feature = "dep_bytemuck", feature = "unsafe_niche", not(feature = "safe_num")))]
            #[cfg_attr(feature = "nightly_doc",
                doc(cfg(all(feature = "dep_bytemuck", feature = "unsafe_niche"))))]
            #[allow(non_snake_case)]
            mod [<$name $s $b>] {
                use super::*;

                unsafe impl<const RMIN: $IP, const RMAX: $IP>
                    ZeroableInOption for $name<RMIN, RMAX> {}
                unsafe impl<const RMIN: $IP, const RMAX: $IP>
                    PodInOption for $name<RMIN, RMAX> {}
                unsafe impl<const RMIN: $IP, const RMAX: $IP>
                    NoUninit for $name<RMIN, RMAX> {}
                unsafe impl<const RMIN: $IP, const RMAX: $IP>
                    CheckedBitPattern for $name<RMIN, RMAX> {
                    type Bits = $IP;

                    #[inline]
                    fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
                        // check if it's within the range after shifting the bit value
                        let value = bits ^ Self::XOR_VALUE;
                        RMIN <= value && value <= RMAX
                    }
                }
            }
        }
    }};
}
impl_in_range!();
