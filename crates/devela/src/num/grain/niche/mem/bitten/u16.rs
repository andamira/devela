//
//!
//

use crate::{_BittenBytes, _BittenSpec, _impl_init, BittenU8, is, unwrap};

#[doc = crate::_tags!(num niche)]
/// An unsigned 16-bit representation with `B` high capacity bits removed.
#[doc = crate::_doc_meta!{
    location("num/grain/niche", struct BittenU16),
    test_size_of(BittenU16<3> = 2|16; niche Option),
}]
/// `BittenU16<B>` represents the contiguous integer range
/// `0..=2^(16 - B) - 1`, for `B` from `0` through `16`.
///
/// Whole removed bytes reduce the storage size. When the remaining capacity
/// is not byte-aligned, the most significant stored byte is a [`BittenU8`]
/// whose unused upper patterns provide a memory niche.
///
/// For example:
/// - `BittenU16<3>` has 13 value bits and occupies two bytes.
/// - `BittenU16<8>` has 8 value bits and occupies one byte.
/// - `BittenU16<9>` has 7 value bits, occupies one byte, and has a niche.
/// - `BittenU16<16>` represents only zero and has zero-sized storage.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[expect(
    private_bounds,
    reason = "the private bound selects the representation for each supported B"
)]
pub struct BittenU16<const B: u8>(<_BittenU16Select<B> as _BittenSpec>::Repr)
where
    _BittenU16Select<B>: _BittenSpec;

pub(crate) struct _BittenU16Select<const B: u8>;

macro_rules! _impl_bitten_u16_common {
    ($B:literal) => {
        /// The number of high capacity bits removed from the `u16` domain.
        pub const BITTEN: u8 = $B;

        /// The number of remaining value-capacity bits.
        pub const BITS: u8 = 16 - $B;

        const MAX_PRIM: u16 = ((1_u32 << (16_u32 - $B as u32)) - 1) as u16;

        /// The minimum representable value.
        pub const MIN: Self = unwrap![some Self::new(0)];

        /// Zero.
        pub const ZERO: Self = Self::MIN;

        /// The maximum representable value.
        pub const MAX: Self = unwrap![some Self::new(Self::MAX_PRIM)];

        /// Creates a value, clamping values above [`MAX`](Self::MAX).
        #[must_use]
        pub const fn new_lossy(value: u16) -> Self {
            unwrap![some_or Self::new(value), Self::MAX]
        }
        /// Creates a value without checking its validity.
        ///
        /// # Safety
        /// `value` must be at most [`MAX`](Self::MAX).
        #[must_use]
        #[cfg(all(not(feature = "safe_num"), feature = "unsafe_niche"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_niche")))]
        pub const unsafe fn new_unchecked(value: u16) -> Self {
            unwrap![some_guaranteed_or_ub Self::new(value)]
        }

        /// Returns whether `self` and `other` represent the same value.
        #[must_use]
        pub const fn eq(self, other: Self) -> bool {
            self.get() == other.get()
        }
    };
}

macro_rules! impl_bitten_u16 {
    ($($B:literal => $N:literal, $R:literal),+ $(,)?) => {
        $(
            impl _BittenSpec for _BittenU16Select<$B> {
                type Repr = _BittenBytes<$N, BittenU8<$R>>;
            }

            _impl_init![Self::ZERO => BittenU16<$B>];

            impl BittenU16<$B> {
                _impl_bitten_u16_common![$B];

                /// Creates a value if `value` is representable.
                #[must_use]
                pub const fn new(value: u16) -> Option<Self> {
                    is! { value > Self::MAX_PRIM, return None }
                    is! { value & !Self::MAX_PRIM != 0, return None }
                    let bytes = value.to_le_bytes();
                    let mut low = [0_u8; $N];
                    let mut i = 0;
                    while i != $N {
                        low[i] = bytes[i];
                        i += 1;
                    }
                    let top = if $N < 2 { bytes[$N] } else { 0 };
                    match BittenU8::<$R>::new(top) {
                        Some(high) => Some(Self(_BittenBytes::new(low, high))),
                        None => None,
                    }
                }

                /// Returns the represented value as a `u16`.
                #[must_use]
                pub const fn get(self) -> u16 {
                    let mut bytes = [0_u8; 2];
                    let mut i = 0;
                    while i != $N {
                        bytes[i] = self.0.low[i];
                        i += 1;
                    }
                    is! { $N < 2, bytes[$N] = self.0.high.get() }
                    u16::from_le_bytes(bytes)
                }
            }
        )+
    };
}
impl_bitten_u16![
     0 => 2, 8,

     1 => 1, 1,
     2 => 1, 2,
     3 => 1, 3,
     4 => 1, 4,
     5 => 1, 5,
     6 => 1, 6,
     7 => 1, 7,
     8 => 1, 8,

     9 => 0, 1,
    10 => 0, 2,
    11 => 0, 3,
    12 => 0, 4,
    13 => 0, 5,
    14 => 0, 6,
    15 => 0, 7,
    16 => 0, 8,
];
