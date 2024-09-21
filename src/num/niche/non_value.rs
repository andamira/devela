// devela::num::niche::non_value
//
//! Creates const generic customizable wrappers over the `NonZero` primitives.
//!
//! Always available for internal use.
//

#![allow(unused)]

#[cfg(all(feature = "bytemuck", feature = "unsafe_niche", not(feature = "safe_num")))]
use crate::_dep::bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};
#[cfg(feature = "unsafe_layout")]
use crate::mem::MemPod;
#[cfg(feature = "mem_bit")]
use crate::mem::{bit_sized, ByteSized};
use crate::{
    _dep::_core::{fmt, num::*, str::FromStr},
    code::{paste, ConstDefault},
    error::unwrap,
};

macro_rules! impl_non_value {
    () => {
        impl_non_value![MIN, "A signed", i,
            8:"_non_value_i8", 16:"_non_value_i16", 32:"_non_value_i32",
            64:"_non_value_i64", 128:"_non_value_i128", size:"_non_value_isize"];
        impl_non_value![MAX, "An unsigned", u,
            8:"_non_value_u8", 16:"_non_value_u16", 32:"_non_value_u32",
            64:"_non_value_u64", 128:"_non_value_u128", size:"_non_value_usize"];
    };
    ($XTR:ident, $doc:literal, $s:ident, $( $b:tt: $cap:literal ),+) => {
        $( paste!{
            impl_non_value![@
                [<NonValue $s:upper $b>], // $name
                $XTR,
                $doc,
                [<$s $b>], // $IP
                $s,
                $b:
                $cap
            ];
        })+
    };

    // $name: the full name of the new type. E.g. NonValueI8.
    // $XTR:  the *extreme* value constant for this type. (MIN | MAX).
    // $doc:  the specific beginning of the documentation.
    // $IP:   the type of the corresponding integer primitive. E.g. i8
    // $s:    the sign identifier: i or u.
    // $b:    the bits of the type, from 8 to 128, or the `size` suffix.
    // $cap:  the capability feature that enables the given implementation. E.g "_non_value_i8".
    (@$name:ident, $XTR:ident, $doc:literal, $IP:ty, $s:ident, $b:tt : $cap:literal) => { paste! {
        /* definition */

        #[doc = $doc " integer that is known not to equal some specific value." ]
        ///
        #[doc = "It has the same memory layout optimization as [`NonZero" $s:upper $b "`],"]
        #[doc = " so that `Option<" $name ">` is the same size as `" $name "`."]
        ///
        /// # Examples
        /// ```
        #[doc = "use devela::num::" $name ";"]
        ///
        #[doc = "assert![" $name "::<13>::new(13).is_none()];"]
        #[doc = "assert![" $name "::<13>::new(12).unwrap().get() == 12];"]
        /// ```
        ///
        #[doc = "See also [`NonExtreme" $s:upper $b "`]."]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        pub struct $name <const V: $IP>([<NonZero $s:upper $b>]);

        /* aliases */

        #[doc = $doc " integer that is known not to equal its most extreme value ([`"
            $XTR "`][" $IP "::" $XTR "])."]
        ///
        /// Unlike the `NonValue*` types in general, this type alias implements
        /// the [`Default`] and [`ConstDefault`][crate::code::ConstDefault] traits.
        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        pub type [<NonExtreme $s:upper $b>] = $name <{$IP::$XTR}>;

        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl Default for [<NonExtreme $s:upper $b>] {
            /// # Features
            /// Makes use of the `unsafe_niche` feature if enabled.
            #[inline] #[must_use]
            fn default() -> Self {
                #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                return [<NonExtreme $s:upper $b>]::new($IP::default()).unwrap();

                #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                // SAFETY: the default primitive value is always 0, and their MAX is never 0.
                unsafe { return [<NonExtreme $s:upper $b>]::new_unchecked($IP::default()); }
            }
        }

        #[cfg(feature = $cap )]
        impl ConstDefault for [<NonExtreme $s:upper $b>] {
            /// # Features
            /// Makes use of the `unsafe_niche` feature if enabled.
            const DEFAULT: Self = {
                #[cfg(any(feature = "safe_num", not(feature = "unsafe_niche")))]
                if let Some(v) = Self::new($IP::DEFAULT) { v } else { unreachable![] }

                #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
                // SAFETY: the default primitive value is always 0, and their MAX is never 0.
                unsafe { [<NonExtreme $s:upper $b>]::new_unchecked($IP::DEFAULT) }
            };
        }

        #[cfg(feature = $cap )]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = $cap)))]
        impl<const V: $IP> $name<V> {
            /* constants */

            /// Returns the maximum possible value.
            #[inline] #[must_use]
            pub const MAX: Self = {
                if $IP::MAX > V {
                    unwrap![some Self::new($IP::MAX)]
                } else {
                    unwrap![some Self::new($IP::MAX - 1)]
                }
            };
            /// Returns the minimum possible value.
            #[inline] #[must_use]
            pub const MIN: Self = {
                if $IP::MIN < V {
                    unwrap![some Self::new($IP::MIN)]
                } else {
                    unwrap![some Self::new($IP::MIN + 1)]
                }
            };

            /* methods */

            #[doc = "Returns a `" $name "` with the given `value`,"
                " if it is not equal to `V`."]
            #[must_use]
            pub const fn new(value: $IP) -> Option<Self> {
                match [<NonZero $s:upper $b>]::new(value ^ V) {
                    None => None,
                    Some(v) => Some(Self(v)),
                }
            }

            #[doc = "Returns a `" $name "` if the given `value`" " if it is not equal to `V`."]
            ///
            /// # Panics
            /// Panics in debug if the given `value` is equal to `V`.
            /// # Safety
            /// The given `value` must never be equal to `V`.
            #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
            #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_niche")))]
            pub const unsafe fn new_unchecked(value: $IP) -> Self {
                #[cfg(debug_assertions)]
                if value == V { panic!("The given value was specifically prohibited.") }

                // SAFETY: caller must ensure safety
                Self(unsafe { [<NonZero $s:upper $b>]::new_unchecked(value ^ V) })
            }

            /// Returns the value as a primitive type.
            #[inline]
            #[must_use]
            pub const fn get(&self) -> $IP {
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

            impl<const V: $IP> fmt::Display for $name <V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}", self.get())
                }
            }
            impl<const V: $IP> fmt::Debug for $name <V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}::<{}>({})", stringify!($name), V, self.get())
                }
            }
            impl<const V: $IP> fmt::Binary for $name<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Binary::fmt(&self.get(), f)
                }
            }
            impl<const V: $IP> fmt::Octal for $name<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Octal::fmt(&self.get(), f)
                }
            }
            impl<const V: $IP> fmt::LowerHex for $name<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::LowerHex::fmt(&self.get(), f)
                }
            }
            impl<const V: $IP> fmt::UpperHex for $name<V> {
                #[inline]
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::UpperHex::fmt(&self.get(), f)
                }
            }

            impl<const V: $IP> FromStr for $name<V> {
                type Err = ParseIntError;
                #[inline]
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::new($IP::from_str(s)?).ok_or_else(||"".parse::<i32>().unwrap_err())
                }
            }

            /* conversions */

            impl<const V: $IP> From<$name<V>> for $IP {
                #[inline]
                #[must_use]
                fn from(value: $name<V>) -> $IP {
                    value.get()
                }
            }

            impl<const V: $IP> TryFrom<$IP> for $name<V> {
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
            bit_sized![<const V: $IP> =
                { $IP::BYTE_SIZE * 8}; for $name<V>];

            #[cfg(feature = "unsafe_layout")]
            unsafe impl<const V: $IP> MemPod for Option<$name<V>> {}

            /* external impls*/

            #[cfg(all(feature = "bytemuck", feature = "unsafe_niche", not(feature = "safe_num")))]
            #[cfg_attr(feature = "nightly_doc",
                doc(cfg(all(feature = "bytemuck", feature = "unsafe_niche"))))]
            #[allow(non_snake_case)]
            mod [<$name $s $b>] {
                use super::*;

                unsafe impl<const V: $IP> ZeroableInOption for $name<V> {}
                unsafe impl<const V: $IP> PodInOption for $name<V> {}
                unsafe impl<const V: $IP> NoUninit for $name<V> {}
                unsafe impl<const V: $IP> CheckedBitPattern for $name<V> {
                    type Bits = $IP;

                    #[inline]
                    fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
                        // Since inner repr is NonZero, 0 is the only invalid bit pattern
                        *bits != 0
                    }
                }
            }
        }
    }};
}
impl_non_value!();
