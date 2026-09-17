//
//! Integers with a number of high capacity bits removed.
//

use crate::{_impl_init, Hash, enumint, is, unwrap};

#[doc = crate::_tags!(num niche)]
/// An unsigned byte representation with `B` high capacity bits removed.
#[doc = crate::_doc_meta!{
    location("num/grain/niche", struct BittenU8),
    test_size_of(BittenU8<2> = 1|8; niche Option),
}]
/// `BittenU8<B>` represents the contiguous integer range
/// `0..=2^(8 - B) - 1`, for `B` from `0` through `8`.
///
/// `B` describes value capacity rather than storage size:
///
/// - `BittenU8<0>` represents the full `u8` domain.
/// - `BittenU8<1>` through `BittenU8<7>` remain one byte while
///   reserving the upper byte patterns as memory niches.
/// - `BittenU8<8>` represents only zero and has zero-sized storage.
///
/// The representation is intended for compact indices, handles, tags,
/// and other values whose domain needs fewer than eight bits.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[expect(
    private_bounds,
    reason = "the private bound selects the representation for each supported B"
)]
pub struct BittenU8<const B: u8>(<__BittenU8Select<B> as __BittenU8Spec>::Repr)
where
    __BittenU8Select<B>: __BittenU8Spec;

_impl_init![Self::ZERO => BittenU8<0>];
_impl_init![Self::ZERO => BittenU8<1>];
_impl_init![Self::ZERO => BittenU8<2>];
_impl_init![Self::ZERO => BittenU8<3>];
_impl_init![Self::ZERO => BittenU8<4>];
_impl_init![Self::ZERO => BittenU8<5>];
_impl_init![Self::ZERO => BittenU8<6>];
_impl_init![Self::ZERO => BittenU8<7>];
_impl_init![Self::ZERO => BittenU8<8>];

/* private representations */

enumint![__BittenU8_1, u8, 0, 127];
enumint![__BittenU8_2, u8, 0, 63];
enumint![__BittenU8_3, u8, 0, 31];
enumint![__BittenU8_4, u8, 0, 15];
enumint![__BittenU8_5, u8, 0, 7];
enumint![__BittenU8_6, u8, 0, 3];
enumint![__BittenU8_7, u8, 0, 1];

/// Representation selection
struct __BittenU8Select<const B: u8>;

trait __BittenU8Spec {
    type Repr: Clone + Copy + Eq + Hash;
}

impl __BittenU8Spec for __BittenU8Select<0> {
    type Repr = u8;
}
impl __BittenU8Spec for __BittenU8Select<1> {
    type Repr = __BittenU8_1;
}
impl __BittenU8Spec for __BittenU8Select<2> {
    type Repr = __BittenU8_2;
}
impl __BittenU8Spec for __BittenU8Select<3> {
    type Repr = __BittenU8_3;
}
impl __BittenU8Spec for __BittenU8Select<4> {
    type Repr = __BittenU8_4;
}
impl __BittenU8Spec for __BittenU8Select<5> {
    type Repr = __BittenU8_5;
}
impl __BittenU8Spec for __BittenU8Select<6> {
    type Repr = __BittenU8_6;
}
impl __BittenU8Spec for __BittenU8Select<7> {
    type Repr = __BittenU8_7;
}
impl __BittenU8Spec for __BittenU8Select<8> {
    type Repr = ();
}

/* endpoints */

#[rustfmt::skip]
impl BittenU8<0> {
    /// The number of high capacity bits removed from the `u8` domain.
    pub const BITTEN: u8 = 0;
    /// The number of remaining value-capacity bits.
    pub const BITS: u8 = 8;
    /// The number of distinct representable values.
    pub const VALUES: u16 = 256;

    /// The minimum representable value.
    pub const MIN: Self = Self(0);
    /// Zero.
    pub const ZERO: Self = Self::MIN;
    /// The maximum representable value.
    pub const MAX: Self = Self(u8::MAX);

    /// Creates a value.
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> { Some(Self(value)) }

    /// Creates a value without loss.
    #[must_use]
    pub const fn new_lossy(value: u8) -> Self { Self(value) }

    /// Creates a value without checking its validity.
    ///
    /// # Safety
    /// Every `u8` is valid for `BittenU8<0>`.
    #[must_use]
    #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_niche")))]
    pub const unsafe fn new_unchecked(value: u8) -> Self { Self(value) }

    /// Returns whether `self` and `other` represent the same value.
    #[must_use]
    pub const fn eq(self, other: Self) -> bool { self.0 == other.0 }

    /// Returns the represented value as a `u8`.
    #[must_use]
    pub const fn get(self) -> u8 { self.0 }
}

#[rustfmt::skip]
impl BittenU8<8> {
    /// The number of high capacity bits removed from the `u8` domain.
    pub const BITTEN: u8 = 8;
    /// The number of remaining value-capacity bits.
    pub const BITS: u8 = 0;
    /// The number of distinct representable values.
    pub const VALUES: u16 = 1;

    /// The minimum representable value.
    pub const MIN: Self = Self(());
    /// Zero.
    pub const ZERO: Self = Self::MIN;
    /// The maximum representable value.
    pub const MAX: Self = Self::MIN;

    /// Creates the zero value, rejecting every non-zero input.
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> { is![value == 0, Some(Self(())), None] }

    /// Creates the only representable value, zero.
    #[must_use]
    pub const fn new_lossy(_value: u8) -> Self { Self::ZERO }

    /// Creates the zero value without checking the input.
    ///
    /// # Safety
    /// `value` must be zero.
    #[must_use]
    #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_niche")))]
    pub const unsafe fn new_unchecked(value: u8) -> Self {
        unwrap![some_guaranteed_or_ub Self::new(value)]
    }
    /// Returns whether `self` and `other` represent the same value.
    #[must_use]
    pub const fn eq(self, other: Self) -> bool { let _ = other; true }

    /// Returns zero.
    #[must_use]
    pub const fn get(self) -> u8 { 0 }
}

/// Generates the enum-backed representations.
macro_rules! impl_bitten_u8 {
    ($B:literal, $Repr:ident, $bits:literal, $values:literal, $max:literal) => {
        impl BittenU8<$B> {
            /// The number of high capacity bits removed from the `u8` domain.
            pub const BITTEN: u8 = $B;

            /// The number of remaining value-capacity bits.
            pub const BITS: u8 = $bits;

            /// The number of distinct representable values.
            pub const VALUES: u16 = $values;

            /// The minimum representable value.
            pub const MIN: Self = unwrap![some Self::new(0)];

            /// Zero.
            pub const ZERO: Self = Self::MIN;

            /// The maximum representable value.
            pub const MAX: Self = unwrap![some Self::new($max)];

            /// Creates a value if `value` is representable.
            #[must_use]
            pub const fn new(value: u8) -> Option<Self> {
                unwrap![=some_map $Repr::new(value), |repr| Self(repr)]
            }
            /// Creates a value, clamping values above [`MAX`](Self::MAX).
            #[must_use]
            pub const fn new_lossy(value: u8) -> Self {
                unwrap![some_or Self::new(value), Self::MAX]
            }
            /// Creates a value without checking its validity.
            ///
            /// # Safety
            /// `value` must be at most [`MAX`](Self::MAX).
            #[must_use]
            #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_niche")))]
            pub const unsafe fn new_unchecked(value: u8) -> Self {
                unwrap![some_guaranteed_or_ub Self::new(value)]
            }

            /// Returns whether `self` and `other` represent the same value.
            #[must_use]
            pub const fn eq(self, other: Self) -> bool { self.0.eq(other.0) }

            /// Returns the represented value as a `u8`.
            #[must_use]
            pub const fn get(self) -> u8 { self.0.get() }
        }
    };
}
impl_bitten_u8!(1, __BittenU8_1, 7, 128, 127);
impl_bitten_u8!(2, __BittenU8_2, 6, 64, 63);
impl_bitten_u8!(3, __BittenU8_3, 5, 32, 31);
impl_bitten_u8!(4, __BittenU8_4, 4, 16, 15);
impl_bitten_u8!(5, __BittenU8_5, 3, 8, 7);
impl_bitten_u8!(6, __BittenU8_6, 2, 4, 3);
impl_bitten_u8!(7, __BittenU8_7, 1, 2, 1);

#[cfg(test)]
mod _test {
    use super::*;

    const B2: BittenU8<2> = unwrap![some BittenU8::<2>::new(42)];
    const B3: BittenU8<3> = unwrap![some BittenU8::<3>::new(17)];

    #[test]
    fn bitten_u8_const() {
        assert_eq!(B2.get(), 42);
        assert_eq!(B3.get(), 17);
        assert!(BittenU8::<2>::new(63).is_some());
        assert!(BittenU8::<2>::new(64).is_none());
        assert!(BittenU8::<3>::new(31).is_some());
        assert!(BittenU8::<3>::new(32).is_none());
    }
    #[test]
    fn bitten_u8_endpoints() {
        assert_eq!(BittenU8::<0>::new(255).unwrap().get(), 255);
        assert_eq!(BittenU8::<8>::new(0).unwrap().get(), 0);
        assert!(BittenU8::<8>::new(1).is_none());
    }
    #[test]
    fn bitten_u8_layout() {
        assert_eq!(size_of::<BittenU8<0>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<0>>>(), 2);
        assert_eq!(size_of::<BittenU8<1>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<1>>>(), 1);
        assert_eq!(size_of::<BittenU8<2>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<2>>>(), 1);
        assert_eq!(size_of::<BittenU8<3>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<3>>>(), 1);
        assert_eq!(size_of::<BittenU8<4>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<4>>>(), 1);
        assert_eq!(size_of::<BittenU8<5>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<5>>>(), 1);
        assert_eq!(size_of::<BittenU8<6>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<6>>>(), 1);
        assert_eq!(size_of::<BittenU8<7>>(), 1);
        assert_eq!(size_of::<Option<BittenU8<7>>>(), 1);
        assert_eq!(size_of::<BittenU8<8>>(), 0);
        assert_eq!(size_of::<Option<BittenU8<8>>>(), 1);
    }
}
