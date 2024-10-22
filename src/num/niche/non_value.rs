// devela::num::niche::non_value
//
//! Creates const generic customizable wrappers over the `NonZero` primitives.
//!
//! Always available for internal use.
//

// Centralized automatic definitions based on enabled features & flags
#[cfg(any(doc, test))]
impl_non_value![i 8];
#[cfg(any(feature = "_char7", feature = "_char24"))]
impl_non_value![u 8];
#[cfg(feature = "_char16")]
impl_non_value![u 16];

/// Implements a `NonValue[I|U]B<V>`,
///
/// - `i` or `u` represents signed or unsigned, respectively.
/// - `B` represents the bit-size, from [8, 16, 32, 64, 128].
/// - `V` is the prohibited value in the bit-sized range.
///
/// # Example
/// ```
/// # use devela::impl_non_value;
/// impl_non_value![i 8];
///
/// assert![NonValueI8::<3>::new(2).is_some()];
/// assert![NonValueI8::<3>::new(3).is_none()];
///
/// assert![NonExtremeI8::new(i8::MIN).is_none()];
/// assert![NonExtremeI8::new(i8::MAX).is_some()];
/// assert![NonExtremeI8::new(0).is_some()];
/// ```
///
/// See for example: [`NonValueI8`] and [`NonExtremeI8`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! impl_non_value{
    (
        // Defines a new signed non-value type. E.g.: impl_non_value![i 32]
        // would generate NonValueI32 and NonExtremeI32
        i $bits:literal) => {
        $crate::impl_non_value![@MIN, "A signed", i, $bits];
    };
    (
        // Defines a new signed non-value type. E.g.: impl_non_value![u 32]
        // would generate NonValueU32 and NonExtremeU32
        u $bits:literal) => {
        $crate::impl_non_value![@MAX, "An unsigned", u, $bits];
    };
    (
     /* private arms */
     @$XTR:ident, $doc:literal, $s:ident, $b:literal) => {
        $crate::paste!{
            $crate::impl_non_value![@
                [<NonValue $s:upper $b>],   // $name
                [<NonZero $s:upper $b>],   // $n0
                [<NonExtreme $s:upper $b>], // $ne
                $XTR,
                $doc,
                [<$s $b>], // $IP
                $s,
                $b
            ];
        }
    };

    // $name: the full name of the new type. E.g. NonValueI8.
    // $n0: the full name of the inner NonZero. E.g. NonZeroI8.
    // $ne: the full name of the new type. E.g. NonExtremeI8.
    //
    // $XTR:  the *extreme* value constant for this type. (MIN | MAX).
    // $doc:  the specific beginning of the documentation.
    // $IP:   the type of the corresponding integer primitive. E.g. i8
    // $s:    the sign identifier: i or u.
    // $b:    the bits of the type, from 8 to 128, or the `size` suffix.
    (@$name:ident, $n0:ident, $ne:ident, $XTR:ident, $doc:literal, $IP:ty, $s:ident, $b:literal)
        => { $crate::paste! {

        pub use [<__impls_ $name >]::*;
        #[allow(non_snake_case)]
        mod [<__impls_ $name >] {
            #[cfg(all(feature = "dep_bytemuck", feature = "unsafe_niche", not(feature = "safe_num")))]
            use $crate::_dep::bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};
            // #[cfg(feature = "unsafe_layout")]
            // use $crate::mem::MemPod;
            #[cfg(feature = "mem_bit")]
            use $crate::mem::{BitSized, ByteSized};
            use $crate::{
                _dep::_core::{fmt, num::*, str::FromStr},
                code::{iif, ConstDefault},
                error::{unwrap, NumError::{Invalid, Overflow}, NumResult},
            };

            /* definition */

            #[doc = $doc " integer that is known not to equal some specific value." ]
            ///
            #[doc = "It has the same memory layout optimization as [`" $n0 "`][core::num::" $n0 "],"]
            #[doc = " so that `Option<" $name ">` is the same size as `" $name "`."]
            ///
            /// # Examples
            /// ```ignore
            /// # use devela::NonValueI8;
            /// assert![NonValueI8::<13>::new(13).is_none()];
            /// assert![NonValueI8::<13>::new(12).unwrap().get() == 12];
            /// ```
            #[doc = "See also [`NonExtreme" $s:upper $b "`]."]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name <const V: $IP>($n0);

            /* aliases */

            #[doc = $doc " integer that is known not to equal its most extreme value ([`"
                $XTR "`][" $IP "::" $XTR "])."]
            ///
            /// Unlike the `NonValue*` types in general, this type alias implements
            /// the [`Default`] and [`ConstDefault`][crate::code::ConstDefault] traits.
            pub type [<NonExtreme $s:upper $b>] = $name <{$IP::$XTR}>;

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

            impl<const V: $IP> $name<V> {
                /* constants */

                /// Returns the maximum possible value.
                pub const MAX: Self = {
                    if $IP::MAX > V {
                        unwrap![some Self::new($IP::MAX)]
                    } else {
                        unwrap![some Self::new($IP::MAX - 1)]
                    }
                };
                /// Returns the minimum possible value.
                pub const MIN: Self = {
                    if $IP::MIN < V {
                        unwrap![some Self::new($IP::MIN)]
                    } else {
                        unwrap![some Self::new($IP::MIN + 1)]
                    }
                };

                /// Returns the number of valid values.
                pub const VALID_VALUES: [<u $b>] = [<u $b>]::MAX;

                /// Returns the number of invalid values.
                pub const INVALID_VALUES: [<u $b>] = 1;

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
                #[inline] #[must_use]
                pub const fn get(&self) -> $IP {
                    self.0.get() ^ V
                }

                /// Returns `true` if it is equal to the maximum value ([`MAX`][Self::MAX]).
                #[inline] #[must_use]
                pub const fn is_max(&self) -> bool {
                    self.get() == $IP::MAX
                }

                /// Returns `true` if it is equal to the minimum value ([`MIN`][Self::MIN]).
                #[inline] #[must_use]
                pub const fn is_min(&self) -> bool {
                    self.get() == $IP::MIN
                }

                /// Checked integer addition. Computes `self + rhs`.
                ///
                /// # Errors
                /// Returns [`Overflow`] if the operations overflows, or
                /// [`Invalid`] if the result equals the forbidden value `V`.
                pub const fn checked_add(&self, other: $IP) -> NumResult<Self> {
                    let res = unwrap![some_ok_or? self.get().checked_add(other), Overflow(None)];
                    unwrap![some_ok_or Self::new(res), Invalid]
                }
                /// Checked integer substration. Computes `self - rhs`.
                ///
                /// # Errors
                /// Returns [`Overflow`] if the operations overflows, or
                /// [`Invalid`] if the result equals the forbidden value `V`.
                pub const fn checked_sub(&self, other: $IP) -> NumResult<Self> {
                    let res = unwrap![some_ok_or? self.get().checked_sub(other), Overflow(None)];
                    unwrap![some_ok_or Self::new(res), Invalid]
                }

                /// Strict integer addition. Computes `self + rhs`.
                ///
                /// # Panics
                /// Panics on overflow or if the result equals the forbidden value `V`.
                pub const fn strict_add(&self, other: $IP) -> Self {
                    let res = unwrap![some self.get().checked_add(other)];
                    unwrap![some Self::new(res)]
                }
                /// Strict integer substration. Computes `self - rhs`.
                ///
                /// # Panics
                /// Panics on overflow or if the result equals the forbidden value `V`.
                pub const fn strict_sub(&self, other: $IP) -> Self {
                    let res = unwrap![some self.get().checked_sub(other)];
                    unwrap![some Self::new(res)]
                }

                /// Saturating integer addition. Computes `self + rhs`.
                ///
                /// Saturates at the numeric bounds instead of overflowing.
                /// If the result would equal `V` it will return `V - 1`.
                pub const fn saturating_add(&self, other: $IP) -> Self {
                    let res = self.get().saturating_add(other);
                    unwrap![some Self::new(iif![res == V; res - 1; res])]
                }
                /// Saturating integer substration. Computes `self - rhs`.
                ///
                /// Saturates at the numeric bounds instead of overflowing.
                /// If the result would equal `V` it will return `V + 1`.
                pub const fn saturating_sub(&self, other: $IP) -> Self {
                    let res = self.get().saturating_sub(other);
                    unwrap![some Self::new(iif![res == V; res + 1; res])]
                }

                /// Wraping integer addition. Computes `self + rhs`.
                ///
                /// Wraps at the numeric bounds instead of overflowing.
                /// If the result would equal `V` it will return `V + 1`.
                pub const fn wrapping_add(&self, other: $IP) -> Self {
                    let res = self.get().wrapping_add(other);
                    unwrap![some Self::new(iif![res == V; res + 1; res])]
                }
                /// Wraping integer subtraction. Computes `self - rhs`.
                ///
                /// Wraps at the numeric bounds instead of overflowing.
                /// If the result would equal `V` it will return `V - 1`.
                pub const fn wrapping_sub(&self, other: $IP) -> Self {
                    let res = self.get().wrapping_sub(other);
                    unwrap![some Self::new(iif![res == V; res - 1; res])]
                }
            }

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
            impl<const V: $IP> BitSized<{$IP::BYTE_SIZE * 8}> for $name<V> {}

            // NOTE: due to the orphan rule we can't implement MemPod for Option<NonValue*>
            // #[cfg(feature = "unsafe_layout")]
            // unsafe impl<const V: $IP> MemPod for Option<$name<V>> {}

            /* external impls*/

            #[cfg(all(feature = "dep_bytemuck", feature = "unsafe_niche", not(feature = "safe_num")))]
            #[cfg_attr(feature = "nightly_doc",
                doc(cfg(all(feature = "dep_bytemuck", feature = "unsafe_niche"))))]
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
#[doc(inline)]
pub use impl_non_value;
