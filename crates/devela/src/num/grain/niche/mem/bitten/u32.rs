//
//! Defines [`BittenU32`].
//

use crate::{_BittenBytes, _BittenSpec, _impl_init, BittenU8, is, unwrap};

#[doc = crate::_tags!(num niche)]
/// An unsigned 32-bit representation with `B` high capacity bits removed.
#[doc = crate::_doc_meta!{
    location("num/grain/niche", struct BittenU32),
    test_size_of(BittenU32<11> = 3|24; niche Option),
}]
/// `BittenU32<B>` represents the contiguous integer range
/// `0..=2^(32 - B) - 1`, for `B` from `0` through `32`.
///
/// Complete removed bytes reduce the physical size, and a partially used
/// upper byte provides niches when the remaining capacity is not byte-aligned.
///
/// For example:
/// - `BittenU32<11>` retains 21 value bits in three bytes, with a niche.
/// - `BittenU32<16>` retains 16 value bits in two bytes, without a niche.
/// - `BittenU32<25>` retains 7 value bits in one byte, with a niche.
/// - `BittenU32<32>` represents only zero, occupying zero bytes.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[expect(
    private_bounds,
    reason = "the private bound selects the representation for each supported B"
)]
pub struct BittenU32<const B: u8>(<_BittenU32Select<B> as _BittenSpec>::Repr)
where
    _BittenU32Select<B>: _BittenSpec;

pub(crate) struct _BittenU32Select<const B: u8>;

/// Value-capacity metadata shared by every supported `B`.
#[expect(
    private_bounds,
    reason = "the private _BittenSpec bound restricts B to supported capacities"
)]
impl<const B: u8> BittenU32<B>
where
    _BittenU32Select<B>: _BittenSpec,
{
    /// The number of high capacity bits removed from the `u8` domain.
    pub const BITTEN: u8 = B;
    /// The number of remaining value-capacity bits.
    pub const BITS: u8 = 32 - B;
}

macro_rules! _impl_bitten_u32_common {
    ($B:literal) => {
        // checked_shr also handles the zero-capacity endpoint (B = 32).
        const MAX_PRIM: u32 = unwrap![some_or u32::MAX.checked_shr($B as u32), 0];

        /// The minimum representable value.
        pub const MIN: Self = unwrap![some Self::new(0)];

        /// Zero.
        pub const ZERO: Self = Self::MIN;

        /// The maximum representable value.
        pub const MAX: Self = unwrap![some Self::new(Self::MAX_PRIM)];

        /// Creates a value, clamping values above [`MAX`](Self::MAX).
        #[must_use]
        pub const fn new_lossy(value: u32) -> Self {
            unwrap![some_or Self::new(value), Self::MAX]
        }

        /// Creates a value without checking its validity.
        ///
        /// # Safety
        /// `value` must be at most [`MAX`](Self::MAX).
        #[must_use]
        #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_niche")))]
        pub const unsafe fn new_unchecked(value: u32) -> Self {
            unwrap![some_guaranteed_or_ub Self::new(value)]
        }

        /// Returns whether `self` and `other` represent the same value.
        #[must_use]
        pub const fn eq(self, other: Self) -> bool {
            self.get() == other.get()
        }
    };
}

macro_rules! impl_bitten_u32 {
    ($($B:literal => $N:literal, $R:literal),+ $(,)?) => {
        $(
            impl _BittenSpec for _BittenU32Select<$B> {
                type Repr = _BittenBytes<$N, BittenU8<$R>>;
            }

            _impl_init![Self::ZERO => BittenU32<$B>];

            impl BittenU32<$B> {
                _impl_bitten_u32_common![$B];

                /// Creates a value if `value` is representable.
                #[must_use]
                pub const fn new(value: u32) -> Option<Self> {
                    is! { value & !Self::MAX_PRIM != 0, return None }
                    let bytes = value.to_le_bytes();
                    let mut low = [0_u8; $N];
                    let mut i = 0;
                    while i != $N {
                        low[i] = bytes[i];
                        i += 1;
                    }
                    let top = if $N < 4 { bytes[$N] } else { 0 };
                    match BittenU8::<$R>::new(top) {
                        Some(high) => Some(Self(_BittenBytes::new(low, high))),
                        None => None,
                    }
                }

                /// Returns the represented value as a `u32`.
                #[must_use]
                pub const fn get(self) -> u32 {
                    let mut bytes = [0_u8; 4];
                    let mut i = 0;
                    while i != $N {
                        bytes[i] = self.0.low[i];
                        i += 1;
                    }
                    is! { $N < 4, bytes[$N] = self.0.high.get() }
                    u32::from_le_bytes(bytes)
                }
            }
        )+
    };
}

impl_bitten_u32![
     0 => 4, 8,
     1 => 3, 1,
     2 => 3, 2,
     3 => 3, 3,
     4 => 3, 4,
     5 => 3, 5,
     6 => 3, 6,
     7 => 3, 7,

     8 => 3, 8,
     9 => 2, 1,
    10 => 2, 2,
    11 => 2, 3,
    12 => 2, 4,
    13 => 2, 5,
    14 => 2, 6,
    15 => 2, 7,
    16 => 2, 8,

    17 => 1, 1,
    18 => 1, 2,
    19 => 1, 3,
    20 => 1, 4,
    21 => 1, 5,
    22 => 1, 6,
    23 => 1, 7,
    24 => 1, 8,

    25 => 0, 1,
    26 => 0, 2,
    27 => 0, 3,
    28 => 0, 4,
    29 => 0, 5,
    30 => 0, 6,
    31 => 0, 7,
    32 => 0, 8,
];
