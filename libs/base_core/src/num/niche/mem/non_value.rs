// devela_base_core::num::niche::mem::non_value
//
//! Creates const generic customizable wrappers over the `NonZero` primitives.
//!
//! Always available for internal use.
//

use crate::items;

impl_non_value![U 8, u8];
impl_non_value![U 16, u16];
impl_non_value![U 32, u32];
impl_non_value![U 64, u64];
impl_non_value![U 128, u128];

impl_non_value![I 8, i8];
impl_non_value![I 16, i16];
impl_non_value![I 32, i32];
impl_non_value![I 64, i64];
impl_non_value![I 128, i128];

#[cfg(target_pointer_width = "8")]
items! { impl_non_value![U 8, usize]; impl_non_value![I 8, isize]; }
#[cfg(target_pointer_width = "16")]
items! { impl_non_value![U 16, usize]; impl_non_value![I 16, isize]; }
#[cfg(target_pointer_width = "32")]
items! { impl_non_value![U 32, usize]; impl_non_value![I 32, isize]; }
#[cfg(target_pointer_width = "64")]
items! { impl_non_value![U 64, usize]; impl_non_value![I 64, isize]; }
#[cfg(target_pointer_width = "128")]
items! { impl_non_value![U 128, usize]; impl_non_value![I 128, isize]; }

/// Implements a `NonValue[I|U]B<V>`.
///
/// - `I` or `U` means a signed or unsigned type, respectively.
/// - `B` represents the bit-size, from [8, 16, 32, 64, 128].
/// - `V` is the prohibited value in the bit-sized range.
///
/// # Example
/// ```
/// # use devela_base_core::{NonValueI8, NonValueU8, NonExtremeI8};
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
//
// NOTE: can't use doc(cfg) attributes in generated methods.
macro_rules! impl_non_value {
    // Defines a new unsigned non-value type.
    // E.g.: impl_non_value![U 32, u32] would generate NonValueU32 and NonExtremeU32
    (I $bits:literal, $IP:ty) => { impl_non_value![@MIN, "A signed", i, $bits, $IP]; };
    (U $bits:literal, $IP:ty) => { impl_non_value![@MAX, "An unsigned", u, $bits, $IP]; };
    (@$XTR:ident, $doc:literal, $s:ident, $b:literal, $IP:ty) => {
        $crate::paste!{
            impl_non_value![@
                [<NonValue $IP:camel>],   // $name
                [<NonZero $IP:camel>],    // $n0
                [<NonExtreme $IP:camel>], // $ne
                $XTR,
                $doc,
                $IP,
                $s,
                $b
            ];
        }
    };
    (
    // $name: the full name of the new type. E.g. NonValueI8.
    // $n0: the full name of the inner NonZero. E.g. NonZeroI8.
    // $ne: the full name of the new type. E.g. NonExtremeI8.
    //
    // $XTR:  the *extreme* value constant for this type. (MIN | MAX).
    // $doc:  the specific beginning of the documentation.
    // $IP:   the type of the corresponding integer primitive. E.g. i8
    // $s:    the sign identifier: i or u.
    // $b:    the bits of the type, from 8 to 128, or the `size` suffix.
    @$name:ident, $n0:ident, $ne:ident, $XTR:ident, $doc:literal, $IP:ty, $s:ident, $b:literal)
        => { $crate::paste! {

        pub use [<__impls_ $name >]::*;
        #[allow(non_snake_case)]
        mod [<__impls_ $name >] {
            // #[cfg(all(feature = "dep_bytemuck", feature = "unsafe_niche",
            // not(base_safe_num)))]
            // use $crate::_dep::bytemuck::{CheckedBitPattern, NoUninit, PodInOption, ZeroableInOption};

            use ::core::num::*;
            use $crate::{is, unwrap, FromStr, NicheValueError::{self, InvalidValue, Overflow},
                Debug, Display, FmtResult, Formatter, Binary, Octal, LowerHex, UpperHex,
            };

            /* definition */

            #[doc = $crate::TAG_NUM!()]
            #[doc = $crate::TAG_NICHE!()]
            #[doc = $doc " integer that is known not to equal some specific value." ]
            ///
            #[doc = "It has the same memory layout optimization as [`" $n0 "`][core::num::" $n0 "],"]
            #[doc = " so that `Option<" $name ">` is the same size as `" $name "`."]
            ///
            /// # Examples
            /// ```ignore
            /// # use devela_base_core::NonValueI8;
            /// assert![NonValueI8::<13>::new(13).is_none()];
            /// assert![NonValueI8::<13>::new(12).unwrap().get() == 12];
            /// ```
            #[doc = "See also [`" $ne "`]."]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name <const V: $IP>(pub(in crate::num::niche::mem) $n0);

            /* aliases */

            #[doc = $crate::TAG_NUM!()]
            #[doc = $crate::TAG_NICHE!()]
            #[doc = $doc " integer that is known not to equal its most extreme value ([`"
                $XTR "`][" $IP "::" $XTR "])."]
            ///
            /// Unlike the `NonValue*` types in general, this type alias implements
            /// the [`Default`] and [`ConstDefault`] traits.
            #[doc = crate::DOCLINK_CONST_DEFAULT!()]
            pub type $ne = $name <{$IP::$XTR}>;

            impl Default for $ne {
                /// # Features
                /// Makes use of the `unsafe_niche` feature if enabled.
                fn default() -> Self {
                    #[cfg(any(base_safe_num, not(feature = "unsafe_niche")))]
                    return $ne::new($IP::default()).unwrap();

                    #[cfg(all(not(base_safe_num), feature = "unsafe_niche"))]
                    // SAFETY: the default primitive value is always 0, and their MAX is never 0.
                    unsafe { return $ne::new_unchecked($IP::default()); }
                }
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
                pub const VALID_VALUES: $IP = $IP::MAX;

                /// Returns the number of invalid values.
                pub const INVALID_VALUES: $IP = 1;

                /* methods */

                #[doc = "Returns a `" $name "` with the given `value`,"
                    " if it is not equal to `V`."]
                #[must_use]
                pub const fn new(value: $IP) -> Option<Self> {
                    if $IP::MIN == 0 && V == $IP::MAX { // unsigned::MAX optimization
                        if value == V { return None; }
                        is![let Some(nz) = $n0::new(!value); Some(Self(nz)); None]
                    } else { // default case: XOR
                        // NOTE: `i*::MIN` uses `LEA`-optimized `value ^ MIN`
                        // (equivalent to `value.wrapping_sub(MIN)`).
                        is![let Some(nz) = $n0::new(value ^ V); Some(Self(nz)); None]
                    }
                }
                /// Creates a non-value integer, automatically converting the prohibited value `V`
                /// to the closest valid number (`V-1` for most cases, `V+1` for `MIN`).
                ///
                /// # Guarantees
                /// - For unsigned `MAX`: `V-1` → stored as `!(V-1)`
                /// - For signed `MIN`: `V+1` → stored as `(V+1) ^ MIN`
                /// - All others: `V-1` → stored as `(V-1) ^ V`
                ///
                /// # Features
                /// - Can use the `unsafe_niche` feature internally.
                ///
                /// # Example
                /// ```
                /// # use devela_base_core::{NonValueI8, NonValueU8};
                /// let x = assert_eq![NonValueU8::<255>::new_lossy(255).get(), 254];
                /// let y = assert_eq![NonValueI8::<-128>::new_lossy(-128).get(), -127];
                /// ```
                #[must_use]
                pub const fn new_lossy(value: $IP) -> Self {
                    let transformed = if $IP::MIN == 0 && V == $IP::MAX { // unsigned MAX case
                        let transformed = if value == V { V - 1 } else { value };
                        !transformed
                    } else { // For MIN: choose MIN+1, for others: V-1
                        let transformed = is![value == V; is![V == $IP::MIN; V + 1; V - 1]; value];
                        transformed ^ V
                    };

                    #[cfg(any(base_safe_num,
                        not(feature = "unsafe_niche")))]
                    { Self(unwrap![some $n0::new(transformed)]) }

                    #[cfg(all(not(base_safe_num),
                        feature = "unsafe_niche"))]
                    // SAFETY: We make sure the transformed value != 0
                    unsafe {
                        const { // compile-time verification:
                            if $IP::MIN == 0 && V == $IP::MAX {
                                assert!(!(V - 1) != 0); // unsigned MAX case
                            } else if V == $IP::MIN {
                                assert!((V + 1) ^ V != 0); // signed MIN case
                            } else {
                                assert!((V - 1) ^ V != 0); // all others
                            }
                        }
                        Self($n0::new_unchecked(transformed))
                    }
                }

                #[doc = "Returns a `" $name "` if the given `value`" " if it is not equal to `V`."]
                ///
                /// # Panics
                /// Panics in debug if the given `value` is equal to `V`.
                /// # Safety
                /// The given `value` must never be equal to `V`.
                #[cfg(all(not(base_safe_num), feature = "unsafe_niche"))]
                pub const unsafe fn new_unchecked(value: $IP) -> Self {
                    #[cfg(debug_assertions)]
                    if value == V { panic!("The given value was specifically prohibited.") }

                    // SAFETY: caller must ensure safety
                    Self(unsafe { $n0::new_unchecked(value ^ V) })
                }

                /// Returns the value as a primitive type.
                #[must_use]
                pub const fn get(&self) -> $IP {
                    if $IP::MIN == 0 && V == $IP::MAX { // unsigned::MAX optimization
                        !self.0.get()
                    } else {
                        self.0.get() ^ V
                    }
                }

                /// Returns `true` if it is equal to the maximum value ([`MAX`][Self::MAX]).
                #[must_use]
                pub const fn is_max(&self) -> bool { self.get() == $IP::MAX }

                /// Returns `true` if it is equal to the minimum value ([`MIN`][Self::MIN]).
                #[must_use]
                pub const fn is_min(&self) -> bool { self.get() == $IP::MIN }

                /// Checked integer addition. Computes `self + rhs`.
                ///
                /// # Errors
                /// Returns [`Overflow`] if the operations overflows, or
                /// [`InvalidValue`] if the result equals the forbidden value `V`.
                pub const fn checked_add(&self, other: $IP) -> Result<Self, NicheValueError> {
                    let res = unwrap![some_ok_or? self.get().checked_add(other), Overflow(None)];
                    unwrap![some_ok_or Self::new(res), InvalidValue]
                }
                /// Checked integer substration. Computes `self - rhs`.
                ///
                /// # Errors
                /// Returns [`Overflow`] if the operations overflows, or
                /// [`InvalidValue`] if the result equals the forbidden value `V`.
                pub const fn checked_sub(&self, other: $IP) -> Result<Self, NicheValueError> {
                    let res = unwrap![some_ok_or? self.get().checked_sub(other), Overflow(None)];
                    unwrap![some_ok_or Self::new(res), InvalidValue]
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
                    unwrap![some Self::new(is![res == V; res - 1; res])]
                }
                /// Saturating integer substration. Computes `self - rhs`.
                ///
                /// Saturates at the numeric bounds instead of overflowing.
                /// If the result would equal `V` it will return `V + 1`.
                pub const fn saturating_sub(&self, other: $IP) -> Self {
                    let res = self.get().saturating_sub(other);
                    unwrap![some Self::new(is![res == V; res + 1; res])]
                }

                /// Wraping integer addition. Computes `self + rhs`.
                ///
                /// Wraps at the numeric bounds instead of overflowing.
                /// If the result would equal `V` it will return `V + 1`.
                pub const fn wrapping_add(&self, other: $IP) -> Self {
                    let res = self.get().wrapping_add(other);
                    unwrap![some Self::new(is![res == V; res + 1; res])]
                }
                /// Wraping integer subtraction. Computes `self - rhs`.
                ///
                /// Wraps at the numeric bounds instead of overflowing.
                /// If the result would equal `V` it will return `V - 1`.
                pub const fn wrapping_sub(&self, other: $IP) -> Self {
                    let res = self.get().wrapping_sub(other);
                    unwrap![some Self::new(is![res == V; res - 1; res])]
                }
            }

            /* core impls */

            impl<const V: $IP> Display for $name <V> {
                fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
                    write!(f, "{}", self.get()) } }
            impl<const V: $IP> Debug for $name <V> {
                fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
                    write!(f, "{}::<{}>({})", stringify!($name), V, self.get()) } }
            impl<const V: $IP> Binary for $name<V> {
                fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
                    Binary::fmt(&self.get(), f) } }
            impl<const V: $IP> Octal for $name<V> {
                fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
                    Octal::fmt(&self.get(), f) } }
            impl<const V: $IP> LowerHex for $name<V> {
                fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
                    LowerHex::fmt(&self.get(), f) } }
            impl<const V: $IP> UpperHex for $name<V> {
                fn fmt(&self, f: &mut Formatter) -> FmtResult<()> {
                    UpperHex::fmt(&self.get(), f) } }
            impl<const V: $IP> FromStr for $name<V> {
                type Err = ParseIntError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::new($IP::from_str(s)?).ok_or_else(||"".parse::<i32>().unwrap_err()) } }

            /* conversions */

            impl<const V: $IP> From<$name<V>> for $IP {
                fn from(value: $name<V>) -> $IP { value.get() }
            }

            impl<const V: $IP> TryFrom<$IP> for $name<V> {
                type Error = $crate::TryFromIntError;

                /// # Features
                /// Makes use of the `unsafe_niche` feature if enabled.
                fn try_from(value: $IP) -> Result<Self, Self::Error> {
                    // We generate a TryFromIntError by intentionally causing a failed conversion.
                    #[cfg(any(base_safe_num,
                        not(feature = "unsafe_niche")))]
                    return Self::new(value).ok_or_else(|| i8::try_from(255_u8).unwrap_err());

                    #[cfg(all(not(base_safe_num),
                        feature = "unsafe_niche"))]
                    return Self::new(value)
                        .ok_or_else(|| unsafe { i8::try_from(255_u8).unwrap_err_unchecked() });
                }
            }
        }
    }};
}
use impl_non_value;
