// devela_base_core::num::niche::absence
//
//! Absence of niche constraints and commitments.
//!
//! This module defines building blocks for working *around* niche-optimized
//! numeric representations, either by explicitly opting out of layout
//! optimization or by abstracting over whether such optimization is used.
//!
//! - [`MaybeNiche`] represents the **absence of representation commitment**:
//!   it abstracts over primitive integers, niche-optimized types, and their
//!   non-optimized counterparts, allowing generic code to remain independent
//!   of the chosen representation.
//!
//! - [`NonNiche`] represents the **absence of niche constraints**: it mirrors
//!   the API of niche-constrained numeric types while storing values unchanged.
//!
//! These types are complementary: one selects a concrete, non-optimized
//! representation, while the other erases the distinction between optimized
//! and non-optimized forms.
//
// TOC
// - struct MaybeNiche
// - struct NonNiche
// - tests

use crate::{
    Cast, ConstInitCore, InvalidValue, NicheValueError, NonValueI8, NonValueI16, NonValueI32,
    NonValueI64, NonValueI128, NonValueIsize, NonValueU8, NonValueU16, NonValueU32, NonValueU64,
    NonValueU128, NonValueUsize, NonZero, Overflow, unwrap,
};

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// A zero-cost wrapper that abstracts over both niche and non-niche integer types.
///
/// `MaybeNiche<T>` represents the absence of a commitment to either a niche-optimized
/// or non-optimized numeric representation.
///
/// It exposes a uniform API for plain integers, `NonNiche`, `NonZero*`, and `NonValue*` variants.
/// It adds no rules of its own and relies entirely on the invariants of `T`.
///
/// This is useful when you want code that can switch between optimized and
/// non-optimized representations without changing any surrounding logic.
///
/// Practical note:
///
/// `MaybeNiche<T>` serves as an adapter for generic implementations.
/// It lets macros and monomorphized structures accept any integer-like
/// representation while remaining fully niche-agnostic.
///
/// Used in macros like `define_handle!` to build generic, niche-agnostic
/// structures without committing to a specific integer representation.
///
/// See also [`NonNiche`], which provides a concrete non-optimized representation
/// with the same public API as niche-constrained types.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaybeNiche<T: Copy>(pub T);

/// impl helper for [`MaybeNiche`].
macro_rules! impl_maybe {
    () => {
        impl_maybe!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
    };
    ($($T:ty),+) => {
        // % $is_niche, $prim, $T
        //   $(,<const V: $V>) $(,*$get)? $(,^$new)? $(,@$non0)?
        // ------------------------------------------------------
        $( impl_maybe![% false, $T, $T]; )+
        $( impl_maybe![% false, $T, NonNiche<$T>, *get, ^new]; )+
        $( impl_maybe![% true,  $T, NonZero<$T>, *get, ^new, @non0]; )+
        $crate::paste!{ $(
            impl_maybe![% true, $T, [<NonValue $T:camel>] <V>, <const V: $T>,
            *get, ^new];
        )+ }
    };
    (%
     $is_niche:literal, // IS_NICHE
     $prim:ty,
     $T:ty $(, <const $V:ident : $v:ty>)?
     $(, *$get:ident)?  // for: get_prim
     $(, ^$new:ident)?  // for: from_prim, *_unchecked
     $(, @$non0:ident)? // identifies nonzero types, for: from_prim_lossy
    ) => {
        impl $(<const $V: $v>)? ConstInitCore for MaybeNiche<$T> where $T: ConstInitCore {
            const INIT: Self = Self::new(<$T>::INIT);
        }

        impl $(<const $V: $v>)? MaybeNiche<$T> {
            /* constants */

            /// Whether this representation uses a memory niche for layout optimization.
            pub const IS_NICHE: bool = $is_niche;

            /// Whether the representable domain forms a contiguous interval.
            ///
            /// That is, whether every primitive value `v` such that
            /// `MIN <= v <= MAX` is representable.
            pub const IS_CONTIGUOUS: bool = {
                // primitives, NonNiche, NonZero
                #[crate::compile(none($(<const $V: $v>)?))]
                const fn _is_contiguous $(<const $V: $v>)? () -> bool { true }
                // NonValue
                #[crate::compile(some($(<const $V: $v>)?))]
                const fn _is_contiguous $(<const $V: $v>)? () -> bool {
                    V == <$prim>::MIN || V == <$prim>::MAX
                }
                _is_contiguous$(::<$V>)?()
            };

            /// Whether the representable domain includes negative values.
            #[allow(unused_comparisons, reason = "for unsigned types")]
            pub const HAS_NEGATIVE: bool = Self::MIN.prim() < 0;

            /// The minimum representable value.
            pub const MIN: Self = Self(<$T>::MIN);
            /// The maximum representable value.
            pub const MAX: Self = Self(<$T>::MAX);

            /// The zero value, if representable by this type.
            pub const ZERO: Option<Self> = unwrap![ok_some Self::try_from_prim(0)];

            /* constructors */

            /// Creates a new `MaybeNiche` containing `value`.
            #[must_use] #[inline(always)]
            pub const fn new(value: $T) -> Self { Self(value) }

            /// Creates a new `MaybeNiche` from a primitive value.
            /// # Errors
            /// - [`InvalidValue`] if the value violates the validity invariant of `T`.
            #[must_use] #[inline(always)]
            pub const fn try_from_prim(primitive: $prim) -> Result<Self, InvalidValue> {
                // WAIT: custom attrs on expr https://github.com/rust-lang/rust/issues/54727
                // Can't use `compile` on expr or stmt, e.g.: return Some(Self(primitive));
                // so we need to leverage defined functions instead.

                // NonNiche, NonValue, NonZero
                #[crate::compile(some($($new)?))]
                const fn _new $(<const $V: $v>)? (v: $prim) -> Option<$T> { $( <$T>::$new(v) )? }
                // primitives
                #[crate::compile(none($($new)?))]
                const fn _new $(<const $V: $v>)? (v: $prim) -> Option<$T> { Some(v) }

                Ok(Self(unwrap![some_ok_or? _new(primitive), crate::InvalidValue]))
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
            /// - For `NonZero*` types, `0` becomes the smallest valid value (`MIN`).
            /// - For `NonValue*` types, conversion defers to their own
            ///   [`new_lossy`](NonValueU8::new_lossy)-style semantics.
            /// - For `NonNiche` and primitive integers, the value is used as-is.
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

            /// Tries to create a new `MaybeNiche` from a `usize`.
            /// # Errors
            /// - [`NicheValueError::Overflow`] if the value cannot be represented by the
            ///   underlying primitive type.
            /// - [`NicheValueError::InvalidValue`] if the value violates the validity
            ///   invariant of `T`.
            #[inline(always)]
            pub const fn try_from_usize(value: usize) -> Result<Self, NicheValueError> {
                // NonNiche, NonValue, NonZero
                #[crate::compile(some($($new)?))]
                const fn _new $(<const $V: $v>)? (v: $prim) -> Option<$T> { $( <$T>::$new(v) )? }
                // primitives
                #[crate::compile(none($($new)?))]
                const fn _new $(<const $V: $v>)? (v: $prim) -> Option<$T> { Some(v) }

                let prim = $crate::paste! { Cast(value).[<checked_cast_to_ $prim>]() };
                let prim = unwrap![ok_err_map? prim, |e| NicheValueError::from_overflow(e)];
                Ok(Self(unwrap![some_ok_or? _new(prim), NicheValueError::InvalidValue]))
            }

            /// Creates a new `MaybeNiche` from a `usize`, saturating at numeric bounds.
            ///
            /// The conversion applies the following steps:
            /// 1. The `usize` value is saturated to the bounds of the primitive type.
            /// 2. If the resulting value violates the niche invariant of `T`,
            ///    a best-effort lossy conversion is applied.
            #[must_use] #[inline(always)]
            pub const fn from_usize_saturating(value: usize) -> Self {
                let prim = $crate::paste! { Cast(value).[<saturating_cast_to_ $prim>]() };
                Self::from_prim_lossy(prim)
            }

            /// Creates a new `MaybeNiche` from a `usize`, wrapping at numeric bounds.
            ///
            /// The conversion applies the following steps:
            /// 1. The `usize` value is wrapped to the primitive type.
            /// 2. If the resulting value violates the niche invariant of `T`,
            ///    a best-effort lossy conversion is applied.
            #[must_use] #[inline(always)]
            pub const fn from_usize_wrapping(value: usize) -> Self {
                let prim = $crate::paste! { Cast(value).[<wrapping_cast_to_ $prim>]() };
                Self::from_prim_lossy(prim)
            }

            /* queries */

            /// Returns `true` if this representation uses a memory niche.
            #[must_use] #[inline(always)]
            pub const fn is_niche(self) -> bool { Self::IS_NICHE }

            /// Returns `true` if the representable domain is contiguous.
            ///
            /// That is, if every primitive value `v` such that `MIN <= v <= MAX` is representable.
            #[must_use] #[inline(always)]
            pub const fn is_contiguous(self) -> bool { Self::IS_CONTIGUOUS }

            /// Returns `true` if the representable domain includes negative values.
            #[must_use] #[inline(always)]
            pub const fn has_negative(self) -> bool { Self::HAS_NEGATIVE }

            /// Returns `true` if this type can represent zero.
            #[must_use] #[inline(always)]
            pub const fn has_zero(self) -> bool { Self::ZERO.is_some() }

            /* representation access */

            /// Returns the validated (niche) representation.
            #[must_use] #[inline(always)]
            pub const fn get(self) -> $T { self.0 }

            /// Alias of [`get`][Self::get], provided for representational clarity.
            #[must_use] #[inline(always)]
            pub const fn repr(self) -> $T { self.get() }

            /* primitive access */

            /// Returns the primitive carrier value.
            #[must_use] #[inline(always)]
            pub const fn get_prim(self) -> $prim { self.0 $( . $get() )? }

            /// Alias of [`get_prim`][Self::get_prim], provided for representational clarity.
            #[must_use] #[inline(always)]
            pub const fn prim(self) -> $prim { self.get_prim() }

            /* casts */

            /// Converts the value into a `usize`, returning an error on overflow.
            ///
            /// # Errors
            /// Will return [`Overflow`] if `self` can't fit in a `usize`.
            #[inline(always)]
            pub const fn try_to_usize(self) -> Result<usize, Overflow> {
                Cast(self.get_prim()).checked_cast_to_usize()
            }
            /// Converts the value into a `usize`, saturating at the numeric bounds.
            #[must_use] #[inline(always)]
            pub const fn to_usize_saturating(self) -> usize {
                Cast(self.get_prim()).saturating_cast_to_usize()
            }
            /// Converts the value into a `usize`, wrapping at the numeric bounds.
            #[must_use] #[inline(always)]
            pub const fn to_usize_wrapping(self) -> usize {
                Cast(self.get_prim()).wrapping_cast_to_usize()
            }
        }
    };
}
impl_maybe![];

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NICHE!()]
/// A zero-cost wrapper that mirrors the shape of a niche type but stores `T` unchanged.
///
/// `NonNiche` represents the absence of niche constraints while preserving
/// API symmetry with niche-optimized numeric types.
///
/// Practical note:
///
/// `NonNiche<T>` is a concrete representation choice. It gives you a parallel,
/// non-optimized version of a type (e.g. fast vs compact) while keeping the
/// same public API and implementation surface.
///
/// Used in types like `charu` to provide a non-optimized parallel to
/// their niche-enabled counterparts.
///
/// See also [`MaybeNiche`], which abstracts over primitive, niche-optimized,
/// and non-optimized integer representations.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonNiche<T: Copy>(pub T);

#[rustfmt::skip]
impl<T: Copy> NonNiche<T> {
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
    #[cfg(all(not(base_safe_num), feature = "unsafe_niche"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_niche")))]
    pub const unsafe fn new_unchecked(value: T) -> Self { Self(value) }

    /// Creates a NonNiche, automatically converting any prohibited values.
    ///
    /// There are no prohibited values. Method provided for API completion.
    #[must_use] #[inline(always)]
    pub const fn new_lossy(value: T) -> Self { Self(value) }

    /// Extracts the inner value.
    #[must_use] #[inline(always)]
    pub const fn get(self) -> T { self.0 }
}

#[rustfmt::skip]
impl<T: Copy> From<T> for NonNiche<T> {
    #[inline(always)]
    fn from(value: T) -> Self { Self(value) }
}

impl<T: Copy + ConstInitCore> ConstInitCore for NonNiche<T> {
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

#[cfg(test)]
mod tests {
    use super::{MaybeNiche, NonNiche, NonValueU8, NonZero};

    #[test]
    fn maybe_niche() {
        let u = MaybeNiche(3_u8);
        let nn = MaybeNiche(NonNiche::<u8>::new(3).unwrap());
        let n0 = MaybeNiche(NonZero::<u8>::new(3).unwrap());
        let nv0 = MaybeNiche(NonValueU8::<0>::new(3).unwrap());
        let nv1 = MaybeNiche(NonValueU8::<1>::new(3).unwrap());

        // u8
        assert_eq![u.is_contiguous(), true];
        assert_eq![u.is_niche(), false];
        assert_eq![u.has_zero(), true];
        // NonNiche
        assert_eq![nn.is_contiguous(), true];
        assert_eq![nn.is_niche(), false];
        assert_eq![nn.has_zero(), true];
        // NonZero
        assert_eq![n0.is_contiguous(), true];
        assert_eq![n0.is_niche(), true];
        assert_eq![n0.has_zero(), false];
        // NonValue::<0>
        assert_eq![nv0.is_contiguous(), true];
        assert_eq![nv0.is_niche(), true];
        assert_eq![nv0.has_zero(), false];
        // NonValue::<1>
        assert_eq![nv1.is_contiguous(), false];
        assert_eq![nv1.is_niche(), true];
        assert_eq![nv1.has_zero(), true];
    }
}
