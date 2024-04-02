// devela::num::niche::non_specific
//
//! Creates const generic customizable wrappers over the `NonZero` primitives.
//!
//! Always available for internal use.
//

#![allow(unused)]

use crate::{
    code::{paste, ConstDefault},
    data::{bit_size, ByteSize},
};
use core::{fmt, num::*, str::FromStr};

#[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
use crate::_deps::bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};

macro_rules! impl_non_specific {
    // Entry point, generates NonSpecific structures for each sign and size.
    ($name:ident) => {
        impl_non_specific![$name, MIN, "A signed", i,
        8:"_i8", 16:"_i16", 32:"_i32", 64:"_i64", 128:"_i128", size:"_isize"];
        impl_non_specific![$name, MAX, "An unsigned", u,
        8:"_u8", 16:"_u16", 32:"_u32", 64:"_u64", 128:"_u128", size:"_usize"];
    };
    ($name:ident, $abs:ident, $doc:literal, $s:ident, $( $b:tt: $cap:literal ),+) => {
        $( impl_non_specific![@$name, $abs, $doc, $s, $b : $cap]; )+
    };

    // $name: the base name of the new type. E.g. NonSpecific.
    // $abs:  the absolute maximum value constant for this type
    // $doc:  the specific beginning of the documentation.
    // $s:    the sign identifier: i or u.
    // $b:    the bits of the type, from 8 to 128, or the `size` suffix.
    // $cap:  the capability feature that enables the given implementation. E.g "_i8".
    (@$name:ident, $abs:ident, $doc:literal, $s:ident, $b:tt : $cap:literal) => { paste! {
        /* definition */

        #[doc = $doc " integer that is known not to equal some specific value." ]
        ///
        /// It has an optimized memory layout, so that
        #[doc = "`Option<"[<$name $s:upper $b>]">` is the same size as `"
            [<$name $s:upper $b>]"`."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::num::" [<$name $s:upper $b>] ";"]
        ///
        #[doc = "assert![" [<$name $s:upper $b>] "::<13>::new(13).is_none()];"]
        #[doc = "assert![" [<$name $s:upper $b>] "::<13>::new(12).unwrap().get() == 12];"]
        /// ```
        ///
        #[doc = "See also [`NonEdge" $s:upper $b "`]."]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        pub struct [<$name $s:upper $b>]<const V: [<$s $b>]>([<NonZero $s:upper $b>]);

        /* aliases */

        #[doc = $doc " integer that is known not to equal its edgemost value ([`"
            $abs "`][" [<$s $b>] "::" $abs "])."]
        ///
        /// Unlike the `NonSpecific*` types in general, this type alias implements
        /// the [`Default`] and [`ConstDefault`][crate::code::ConstDefault] traits.
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        pub type [<NonEdge $s:upper $b>] = [<$name $s:upper $b>]<{[<$s $b>]::$abs}>;

        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Default for [<NonEdge $s:upper $b>] {
            /// # Features
            /// Makes use of the `unsafe_niche` feature if enabled.
            #[inline] #[must_use]
            fn default() -> Self {
                #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                return [<NonEdge $s:upper $b>]::new([<$s $b>]::default()).unwrap();

                #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                // SAFETY: the default numeric primitive value is always 0, and their MAX is never 0.
                unsafe { return [<NonEdge $s:upper $b>]::new_unchecked([<$s $b>]::default()); }
            }
        }

        #[cfg(feature = $cap )]
        impl ConstDefault for [<NonEdge $s:upper $b>] {
            /// # Features
            /// Makes use of the `unsafe_niche` feature if enabled.
            const DEFAULT: Self = {
                #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                if let Some(v) = Self::new([<$s $b>]::DEFAULT) { v } else { unreachable![] }

                #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                // SAFETY: the default numeric primitive value is always 0, and their MAX is never 0.
                unsafe { [<NonEdge $s:upper $b>]::new_unchecked([<$s $b>]::DEFAULT) }
            };
        }

        /* methods */

        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<const V: [<$s $b>]> [<$name $s:upper $b>]<V> {
            #[doc = "Returns a `" [<$name $s:upper $b>] "` with the given `value`,"
                " if it is not equal to `V`."]
            #[must_use]
            pub const fn new(value: [<$s $b>]) -> Option<Self> {
                match [<NonZero $s:upper $b>]::new(value ^ V) {
                    None => None,
                    Some(v) => Some(Self(v)),
                }
            }

            #[doc = "Returns a `" [<$name $s:upper $b>] "` if the given `value`"
                " if it is not equal to `V`."]
            ///
            /// # Panics
            /// Panics in debug if the given `value` is equal to `V`.
            /// # Safety
            /// The given `value` must never be equal to `V`.
            #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            pub const unsafe fn new_unchecked(value: [<$s $b>]) -> Self {
                #[cfg(debug_assertions)]
                if value == V { panic!("The given value was specifically prohibited.") }

                Self([<NonZero $s:upper $b>]::new_unchecked(value ^ V))
            }

            /// Returns the value as a primitive type.
            #[inline]
            #[must_use]
            pub const fn get(&self) -> [<$s $b>] {
                self.0.get() ^ V
            }

            /// Returns the number of valid values.
            pub const VALID_VALUES: [<u $b>] = [<u $b>]::MAX;

            /// Returns the number of invalid values.
            pub const INVALID_VALUES: [<u $b>] = 1;
        }


        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        mod [<extra_impls_ $s $b >] {
            use super::*;

            /* core impls */

            impl<const V: [<$s $b>]> fmt::Display for [<$name $s:upper $b>]<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}", self.get())
                }
            }
            impl<const V: [<$s $b>]> fmt::Debug for [<$name $s:upper $b>]<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}::<{}>({})", stringify!([<$name $s:upper $b>]), V, self.get())
                }
            }
            impl<const V: [<$s $b>]> fmt::Binary for [<$name $s:upper $b>]<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Binary::fmt(&self.get(), f)
                }
            }
            impl<const V: [<$s $b>]> fmt::Octal for [<$name $s:upper $b>]<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Octal::fmt(&self.get(), f)
                }
            }
            impl<const V: [<$s $b>]> fmt::LowerHex for [<$name $s:upper $b>]<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::LowerHex::fmt(&self.get(), f)
                }
            }
            impl<const V: [<$s $b>]> fmt::UpperHex for [<$name $s:upper $b>]<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::UpperHex::fmt(&self.get(), f)
                }
            }

            impl<const V: [<$s $b>]> FromStr for [<$name $s:upper $b>]<V> {
                type Err = ParseIntError;
                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::new([<$s $b>]::from_str(s)?).ok_or_else(||"".parse::<i32>().unwrap_err())
                }
            }

            /* conversions */

            impl<const V: [<$s $b>]> From<[<$name $s:upper $b>]<V>> for [<$s $b>] {
                #[inline]
                #[must_use]
                fn from(value: [<$name $s:upper $b>]<V>) -> [<$s $b>] {
                    value.get()
                }
            }

            impl<const V: [<$s $b>]> TryFrom<[<$s $b>]> for [<$name $s:upper $b>]<V> {
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
            bit_size![<const V: [<$s $b>]> =
                { [<$s $b>]::BYTE_SIZE * 8}; for [<$name $s:upper $b>]<V>];

            /* external impls*/

            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            #[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
            unsafe impl<const V: [<$s $b>]> ZeroableInOption for [<$name $s:upper $b>]<V> {}

            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            #[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
            unsafe impl<const V: [<$s $b>]> PodInOption for [<$name $s:upper $b>]<V> {}

            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            #[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
            unsafe impl<const V: [<$s $b>]> NoUninit for [<$name $s:upper $b>]<V> {}

            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            #[cfg(all(feature = "unsafe_niche", not(feature = "safe_num")))]
            unsafe impl<const V: [<$s $b>]> CheckedBitPattern for [<$name $s:upper $b>]<V> {
                type Bits = [<$s $b>];

                #[inline]
                fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
                    // Since inner repr is NonZero, 0 is the only invalid bit pattern
                    *bits != 0
                }
            }
        }
    }};
}
impl_non_specific![NonSpecific];
