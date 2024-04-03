// devela::num::niche::non_range
//
//! Creates const generic customizable wrappers over the `NonZero` primitives.
//

#[cfg(feature = "data_bit")]
use crate::data::{bit_size, ByteSize};
#[cfg(feature = "_-ints-_")]
use crate::{
    _core::{fmt, num::*, str::FromStr},
    code::iif,
};

#[cfg(all(
    feature = "_-ints-_",
    feature = "unsafe_niche",
    not(feature = "safe_num")
))]
use crate::_deps::bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};

impl_non_range![NonRange];
macro_rules! impl_non_range {
    // Entry point, generates NonRange structures for each sign and size.
    ($name:ident) => {
        impl_non_range![$name, "A signed", i,
        8:"_i8", 16:"_i16", 32:"_i32", 64:"_i64", 128:"_i128", size:"_isize"];
        impl_non_range![$name, "An unsigned", u,
        8:"_u8", 16:"_u16", 32:"_u32", 64:"_u64", 128:"_u128", size:"_usize"];
    };
    ($name:ident, $doc:literal, $s:ident, $( $b:tt : $cap:literal),+) => {
        $( impl_non_range![@$name, $doc, $s, $b : $cap]; )+
    };

    // $name: the base name of the new type. E.g. NonRange.
    // $doc:  the specific beginning of the documentation.
    // $s:    the sign identifier, lowercase: i or u.
    // $b:    the bits of the type, from 8 to 128, or the `size` suffix.
    // $cap:  the capability feature that enables the given implementation. E.g "_i8".
    (@$name:ident, $doc:literal, $s:ident, $b:tt : $cap:literal) => { $crate::paste! {
        /* definition */

        #[doc = $doc " integer that is known to be excluded from some inclusive range." ]
        ///
        /// It has an optimized memory layout, so that
        #[doc = "`Option<"[<$name $s:upper $b>]">` is the same size as `"[<$name $s:upper $b>]"`."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::num::" [<$name $s:upper $b>] ";"]
        ///
        #[doc = "assert![" [<$name $s:upper $b>] "::<5, 25>::new(5).is_none()];"]
        #[doc = "assert![" [<$name $s:upper $b>] "::<5, 25>::new(25).is_none()];"]
        #[doc = "assert![" [<$name $s:upper $b>] "::<5, 25>::new(4).unwrap().get() == 4];"]
        #[doc = "assert![" [<$name $s:upper $b>] "::<5, 25>::new(26).unwrap().get() == 26];"]
        ///
        #[doc = "// Self::INVALID_VALUES + Self::VALID_VALUES == "[<u $b>]"::MAX + 1."]
        #[doc = "assert![" [<$name $s:upper $b>] "::<5, 25>::VALID_VALUES == "
            [<u $b>]"::MAX - 21 + 1];"]
        #[doc = "assert![" [<$name $s:upper $b>] "::<5, 25>::INVALID_VALUES == 21];"]
        /// ```
        ///
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        pub struct [<$name $s:upper $b>]<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
            ([<NonZero $s:upper $b>]);

        /* methods */

        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]> [<$name $s:upper $b>]<RMIN, RMAX> {
            #[doc = "Returns a `" [<$name $s:upper $b>] "` with the given `value`,"
            " only if it's **not** between `RMIN` and `RMAX`."]
            ///
            /// Returns `None` if `value` is between `RMIN` and `RMAX`, inclusive,
            /// or if `RMIN > RMAX`.
            #[must_use]
            pub const fn new(value: [<$s $b>]) -> Option<Self> {
                if RMIN <= RMAX && (value < RMIN || value > RMAX) {
                    match [<NonZero $s:upper $b>]::new(value ^ RMIN) {
                        None => None,
                        Some(v) => Some(Self(v)),
                    }
                } else {
                    None
                }
            }

            #[doc = "Returns a `" [<$name $s:upper $b>]
            "` if the given value is **not** between `RMIN` and `RMAX`."]
            ///
            /// # Panics
            /// Panics in debug if the given `value` is between `RMIN` and `RMAX`,
            /// inclusive, or if `RMIN > RMAX`.
            /// # Safety
            /// The given `value` must never be between `RMIN` and `RMAX`, inclusive.
            #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            pub const unsafe fn new_unchecked(value: [<$s $b>]) -> Self {
                #[cfg(debug_assertions)]
                if RMIN > RMAX { panic!("RMIN must be less or equal than RMAX.") }
                #[cfg(debug_assertions)]
                if value >= RMIN || value <= RMAX {
                    panic!("The given value was inside the given prohibited range.")
                }
                Self([<NonZero $s:upper $b>]::new_unchecked(value ^ RMIN))
            }

            /// Returns the value as a primitive type.
            #[must_use]
            #[inline]
            pub const fn get(&self) -> [<$s $b>] {
                self.0.get() ^ RMIN
            }

            // The bit representation of `RMIN`.
            #[allow(unused)]
            const RMIN_BITS: [<$s $b>] = 0;

            // The bit representation of `RMAX`.
            #[allow(unused)]
            const RMAX_BITS: [<$s $b>] = {
                iif![RMIN > RMAX; panic!("RMIN must be less or equal than RMAX.")];
                RMAX.wrapping_sub(RMIN)
            };

            /// Returns the number of valid values outside the range from `RMIN` to `RMAX`,
            /// inclusive, as an unsigned integer with equal bit size.
            ///
            /// # Notice
            /// A range where `RMAX == RMIN` will result in `VALID_VALUES ==
            #[doc = "`[`" [<u $b>]"::MAX`], as expected."]
            ///
            #[doc = "A range from [`"[<$s $b>]"::MIN`] to [`"[<$s $b>]"::MAX`] will result"]
            /// in `VALID_VALUES == 0`, also as expected.
            /// Just be aware that in this case `INVALID_VALUES` will also be `== 0` instead of
            #[doc = " [`" [<u $b>]"::MAX`]` + 1`."]
            pub const VALID_VALUES: [<u $b>] = ([<u $b>]::MAX - Self::INVALID_VALUES)
                .wrapping_add(1);

            /// Returns the number of invalid values in the range from `RMIN` to `RMAX`,
            /// inclusive, as an unsigned integer with equal bit size.
            ///
            /// # Notice
            /// A range where `RMAX == RMIN` will result in `INVALID_VALUES == 1`, as expected.
            ///
            #[doc = "A range from [`"[<$s $b>]"::MIN`] to [`"[<$s $b>]"::MAX`] will result"
            " in `INVALID_VALUES == 0` instead of [`"[<u $b>]"::MAX`]` + 1`."]
            /// This is because the maximum number of representable values for a given
            /// bit-size can't be represented using the same number of bits.
            /// In this case `VALID_VALUES` will also be `== 0`.
            ///
            #[doc = "This doesn't matter as much because a [`"[<$name $s:upper $b>]"`] with the"]
            /// largest range of invalid values can't ever be constructed.
            ///
            #[doc = "Remember that `INVALID_VALUES + VALID_VALUES == "[<u $b>]"::MAX + 1`,"]
            /// which would wrap to `0`.
            pub const INVALID_VALUES: [<u $b>] = {
                iif![RMIN > RMAX; panic!("RMIN must be less or equal than RMAX.")];
                RMAX.wrapping_add((RMIN.wrapping_neg()).wrapping_add(1)) as [<u $b>]
            };
        }

        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        mod [<extra_impls_ $s $b >] {
            use super::*;

            /* core impls */

            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                fmt::Display for [<$name $s:upper $b>]<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}", self.get())
                }
            }
            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                fmt::Debug for [<$name $s:upper $b>]<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}::<{}, {}>({})",
                        stringify!([<$name $s:upper $b>]), RMIN, RMAX, self.get())
                }
            }
            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                fmt::Binary for [<$name $s:upper $b>]<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Binary::fmt(&self.get(), f)
                }
            }
            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                fmt::Octal for [<$name $s:upper $b>]<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Octal::fmt(&self.get(), f)
                }
            }
            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                fmt::LowerHex for [<$name $s:upper $b>]<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::LowerHex::fmt(&self.get(), f)
                }
            }
            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                fmt::UpperHex for [<$name $s:upper $b>]<RMIN, RMAX> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::UpperHex::fmt(&self.get(), f)
                }
            }

            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                FromStr for [<$name $s:upper $b>]<RMIN, RMAX> {
                type Err = ParseIntError;
                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::new([<$s $b>]::from_str(s)?).ok_or_else(||"".parse::<i32>().unwrap_err())
                }
            }

            /* conversions */

            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                From<[<$name $s:upper $b>]<RMIN, RMAX>> for [<$s $b>] {
                #[inline]
                #[must_use]
                fn from(value: [<$name $s:upper $b>]<RMIN, RMAX>) -> [<$s $b>] {
                    value.get()
                }
            }

            impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                TryFrom<[<$s $b>]> for [<$name $s:upper $b>]<RMIN, RMAX> {
                type Error = core::num::TryFromIntError;

                /// # Features
                /// Makes use of the `unsafe_niche` feature if enabled.
                #[inline]
                fn try_from(value: [<$s $b>]) -> Result<Self, Self::Error> {
                    // We generate a TryFromIntError by intentionally causing a failed conversion.
                    #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                    return Self::new(value).ok_or_else(|| i8::try_from(255_u8).unwrap_err());

                    #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                    return Self::new(value)
                        .ok_or_else(|| unsafe { i8::try_from(255_u8).unwrap_err_unchecked() });
                }
            }

            /* internal impls */

            // BitSize
            #[cfg(feature = "data_bit")]
            bit_size![<const RMIN: [<$s $b>], const RMAX: [<$s $b>]> =
                { [<$s $b>]::BYTE_SIZE * 8}; for [<$name $s:upper $b>]<RMIN, RMAX>];

            /* external impls*/

            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            #[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
            unsafe impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                ZeroableInOption for [<$name $s:upper $b>]<RMIN, RMAX> {}

            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            #[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
            unsafe impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                PodInOption for [<$name $s:upper $b>]<RMIN, RMAX> {}

            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            #[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
            unsafe impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                NoUninit for [<$name $s:upper $b>]<RMIN, RMAX> {}

            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            #[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
            unsafe impl<const RMIN: [<$s $b>], const RMAX: [<$s $b>]>
                CheckedBitPattern for [<$name $s:upper $b>]<RMIN, RMAX> {
                type Bits = [<$s $b>];

                #[inline]
                fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
                    // if RMAX bit representation wraps around its type's MAX value
                    // (e.g between 0_i8 and i8::MAX or - i8::MIN and Self::RMAX_BITS):
                    if Self::RMAX_BITS < Self::RMIN_BITS {
                        !(*bits >= Self::RMIN_BITS || *bits <= Self::RMAX_BITS)
                    // if it doesn't (e.g. between 0_i8 and Self::RMAX_BITS):
                    } else {
                        !(*bits >= Self::RMIN_BITS && *bits <= Self::RMAX_BITS)
                    }
                }
            }
        }
    }};
}
use impl_non_range;
