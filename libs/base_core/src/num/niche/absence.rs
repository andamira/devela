// devela_base_core::num::niche::absence
//
//! Defines [`MaybeNiche`] and [`NonNiche`].
//
// TOC
// - struct MaybeNiche
// - struct NonNiche

use crate::{
    Cast, ConstInitCore, NonValueI8, NonValueI16, NonValueI32, NonValueI64, NonValueI128,
    NonValueIsize, NonValueU8, NonValueU16, NonValueU32, NonValueU64, NonValueU128, NonValueUsize,
    NonZero, Overflow, unwrap,
};

/// A zero-cost wrapper over both niche and non-niche integer primitives.
///
/// It' implemented for all the integer primitives, `NonNiche*` `NonZero*` and `NonValue*`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaybeNiche<T: Copy>(T);

/// impl helper for [`MaybeNiche`].
macro_rules! impl_maybe {
    () => {
        impl_maybe!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    };
    ($($T:ty),+) => {
        // $niche, $prim, $T $(,<const V: $V>) $(,*$get)? $(,^$new)?
        // ---------------------------------------------------------
        $( impl_maybe![%false, $T, $T]; )+
        $( impl_maybe![%false, $T, NonNiche<$T>, *get, ^new]; )+
        $( impl_maybe![%true, $T, NonZero<$T>, *get, ^new, @non0]; )+
        $crate::paste!{ $(
            impl_maybe![%true, $T, [<NonValue $T:camel>] <V>, <const V: $T>,
            *get, ^new];
        )+ }
    };
    (%
     $niche:literal,
     $prim:ty,
     $T:ty $(, <const $V:ident : $v:ty>)?
     $(, *$get:ident)? // for: as_prim
     $(, ^$new:ident)? // for: from_prim, *_unchecked
     $(, @$non0:ident)? // to identify nonzero types, for from_prim_lossy
    ) => {
        impl $(<const $V: $v>)? ConstInitCore for MaybeNiche<$T> where $T: ConstInitCore {
            const INIT: Self = Self::new(<$T>::INIT);
        }

        impl $(<const $V: $v>)? MaybeNiche<$T> {
            /// Whether this type supports memory-niche optimization.
            pub const IS_NICHE: bool = $niche;

            /* constructors */

            /// Creates a new `MaybeNiche` containing `value`.
            #[must_use] #[inline(always)]
            pub const fn new(value: $T) -> Self { Self(value) }

            /// Creates a new `MaybeNiche` from a primitive value.
            #[must_use] #[inline(always)]
            pub const fn from_prim(primitive: $prim) -> Option<Self> {
                // WAIT: custom attrs on expr https://github.com/rust-lang/rust/issues/54727
                // Can't use `compile` on expr or stmt, e.g.: return Some(Self(primitive));
                // so we need to leverage defined functions instead.

                // NonNiche, NonValue, NonZero
                #[crate::compile(some($($new)?))]
                const fn _new $(<const $V: $v>)? (v: $prim) -> Option<$T> { $( <$T>::$new(v) )? }
                // primitives
                #[crate::compile(none($($new)?))]
                const fn _new $(<const $V: $v>)? (v: $prim) -> Option<$T> { Some(v) }

                Some(Self(unwrap![some? _new(primitive)]))
            }

            /// Creates a new `MaybeNiche` without any checks.
            /// # Safety
            /// For niche-optimized types, callers must ensure that
            /// `value` satisfies the variant's validity constraints.
            #[must_use] #[inline(always)]
            #[cfg(all(not(base_safe_num), feature = "unsafe_niche"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_niche")))]
            pub const unsafe fn from_prim_unchecked(primitive: $prim) -> Self {
                // NonNiche, NonValue, NonZero
                #[crate::compile(some($($new)?))]
                const fn _new $(<const $V: $v>)? (v: $prim) -> $T { $crate::paste! {
                    unsafe { $( <$T>:: [< $new _unchecked >](v) )? }
                }}
                // primitives
                #[crate::compile(none($($new)?))]
                const fn _new $(<const $V: $v>)? (v: $prim) -> $T { v }

                Self(_new(primitive))
            }

            /// Creates a new `MaybeNiche` from a primitive value, converting invalid inputs
            /// into a valid but *approximate* representation.
            ///
            /// This constructor performs a **lossy conversion**, applying a best-effort
            /// fallback when the primitive violates the underlying type’s invariant:
            ///
            /// - For [`NonZero*`] types, `0` becomes the smallest valid value (`MIN`).
            /// - For [`NonValue*`] types, conversion defers to their own
            ///   [`new_lossy`](NonValueU8::new_lossy)-style semantics.
            /// - For [`NonNiche`] and primitive integers, the value is used as-is.
            #[must_use] #[inline(always)]
            pub const fn from_prim_lossy(value: $prim) -> Self {
                // NonZero converts
                #[crate::compile(all(some($($new)?), some($($non0)?)))]
                const fn _lossy $(<const $V: $v>)? (v: $prim) -> $T {
                    if v == 0 { <$T>::MIN }
                    else {
                        #[cfg(any(base_safe_num, not(feature = "unsafe_niche")))] // safe
                        { unwrap![some <$T>::new(v)] }
                        #[cfg(all(not(base_safe_num), feature = "unsafe_niche"))] // unsafe
                        { unwrap![some_guaranteed_or_ub <$T>::new(v)] }
                    }
                }
                // NonNiche, NonValue (has its own rules)
                #[crate::compile(all(some($($new)?), none($($non0)?)))]
                const fn _lossy $(<const $V: $v>)? (v: $prim) -> $T { $crate::paste! {
                    $( <$T>:: [< $new _lossy >](v) )?
                }}
                // primitives
                #[crate::compile(none($($new)?))]
                const fn _lossy $(<const $V: $v>)? (v: $prim) -> $T { v }

                Self(_lossy(value))
            }

            /* queries */

            /// Returns `true` if this type has a memory-niche optimization.
            #[must_use] #[inline(always)]
            pub const fn is_niche(self) -> bool { Self::IS_NICHE }

            /* accessors */

            /// Returns a copy of the inner value.
            #[must_use] #[inline(always)]
            pub const fn get(self) -> $T { self.0 }

            /// Returns the primitive value directly.
            #[must_use] #[inline(always)]
            pub const fn as_prim(self) -> $prim { self.0 $( . $get() )? }

            /* casts */

            /// Converts the value into a `usize`, returning an error on overflow.
            #[must_use] #[inline(always)]
            pub const fn try_to_usize(self) -> Result<usize, Overflow> {
                Cast(self.as_prim()).checked_cast_to_usize()
            }
            /// Converts the value into a `usize`, saturating at the numeric bounds.
            #[must_use] #[inline(always)]
            pub const fn to_usize_saturating(self) -> usize {
                Cast(self.as_prim()).saturating_cast_to_usize()
            }
            /// Converts the value into a `usize`, wrapping at the numeric bounds.
            #[must_use] #[inline(always)]
            pub const fn to_usize_wrapping(self) -> usize {
                Cast(self.as_prim()).wrapping_cast_to_usize()
            }
        }
    };
}
impl_maybe![];

/// A zero-cost wrapper that behaves like a niche type but stores `T` directly.
///
/// This type is useful when you want to offer API consistency between
/// niche-optimized and non-optimized versions of a type, allowing users
/// to choose based on their memory layout needs.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonNiche<T>(T);

#[rustfmt::skip]
impl<T> NonNiche<T> {
    /// Creates a new `NonNiche` with the given value.
    ///
    /// This always succeeds, unlike `NonZero*` types.
    #[must_use] #[inline(always)]
    pub const fn new(value: T) -> Option<Self> { Some(Self(value)) }

    /// Creates a new `NonNiche` without checking.
    /// # Safety
    /// This is always safe since `NonNiche` doesn't have any validity constraints.
    /// Method provided for API completion.
    #[must_use] #[inline(always)]
    pub const unsafe fn new_unchecked(value: T) -> Self { Self(value) }

    /// Creates a NonNiche, automatically converting any prohibited values.
    ///
    /// There are no prohibited values. Method provided for API completion.
    #[must_use] #[inline(always)]
    pub const fn new_lossy(value: T) -> Self { Self(value) }

    /// Extracts the inner value.
    #[must_use] #[inline(always)]
    pub const fn get(&self) -> T where T: Copy { self.0 }

    #[must_use] #[inline(always)]
    /// Extracts the inner value by value.
    pub const fn into_inner(self) -> T where T: Copy { self.0 }
}

#[rustfmt::skip]
impl<T> From<T> for NonNiche<T> { #[inline(always)] fn from(value: T) -> Self { Self(value) } }

impl<T: ConstInitCore> ConstInitCore for NonNiche<T> {
    const INIT: Self = Self(T::INIT);
}

// helper make implementations over primitives.
macro_rules! impl_non {
    () => { impl_non!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize); };
    ($($prim:ty),+) => {
        $(
            impl NonNiche<$prim> {
                /// The minimum possible value.
                pub const MIN: Self = Self(<$prim>::MIN);
                /// The maximum possible value.
                pub const MAX: Self = Self(<$prim>::MAX);
            }
        )+
    };
}
impl_non![];
