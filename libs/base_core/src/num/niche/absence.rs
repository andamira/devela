// devela_base_core::num::niche::absence
//
//! Defines [`MaybeNiche`] and [`NonNiche`].
//
// TOC
// - struct MaybeNiche
// - struct NonNiche

use crate::{
    NonValueI8, NonValueI16, NonValueI32, NonValueI64, NonValueI128, NonValueIsize, NonValueU8,
    NonValueU16, NonValueU32, NonValueU64, NonValueU128, NonValueUsize, NonZero,
};

/// A zero-cost wrapper over both niche and non-niche integer primitives.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaybeNiche<T>(T);

/// impl helper for [`MaybeNiche`].
macro_rules! impl_maybe {
    () => {
        impl_maybe!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    };
    ($($T:ty),+) => {
        $( impl_maybe![%false, $T, $T]; )+
        $( impl_maybe![%false, $T, NonNiche<$T>]; )+
        $( impl_maybe![%true, $T, NonZero<$T>]; )+
        $crate::paste!{ $(
            impl_maybe![%true, $T, [<NonValue $T:camel>] <V>, <const V: $T>];
        )+ }
    };
    (% $niche:literal, $prim:ty, $T:ty $(, <const $V:ident : $v:ty>)? ) => {
        impl $(<const $V: $v>)? MaybeNiche<$T> {
            /// Indicates whether this type has a memory-niche optimization.
            pub const IS_NICHE: bool = $niche;

            #[inline(always)]
            /// Creates a new `MaybeNiche` containing `value`.
            pub const fn new(value: $T) -> Self { Self(value) }

            /// Creates a new `MaybeNiche` without any checks.
            /// # Safety
            /// For niche-optimized types, callers must ensure that
            /// `value` satisfies the variant’s validity constraints.
            #[must_use] #[inline(always)]
            pub const unsafe fn new_unchecked(value: $T) -> Self { Self(value) }

            #[must_use] #[inline(always)]
            /// Returns a copy of the inner value.
            pub const fn get(&self) -> $T { self.0 }

            #[must_use] #[inline(always)]
            /// Consumes the wrapper and returns the inner value by value.
            pub const fn into_inner(self) -> $T { self.0 }

            #[must_use] #[inline(always)]
            /// Returns `true` if this type has a memory-niche optimization.
            pub const fn is_niche(&self) -> bool { Self::IS_NICHE }
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
    #[must_use] #[inline(always)]
    pub const unsafe fn new_unchecked(value: T) -> Self { Self(value) }

    /// Extracts the inner value.
    #[must_use] #[inline(always)]
    pub const fn get(&self) -> T where T: Copy { self.0 }

    #[must_use] #[inline(always)]
    /// Extracts the inner value by value.
    pub const fn into_inner(self) -> T where T: Copy { self.0 }
}

#[rustfmt::skip]
impl<T> From<T> for NonNiche<T> { #[inline(always)] fn from(value: T) -> Self { Self(value) } }

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
