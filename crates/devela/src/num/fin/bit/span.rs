//
//! Defines [`BitSpan`].
//

use crate::{MismatchedBounds, MismatchedCapacity, bitwise, is};

#[doc = crate::_tags!(bit data_structure)]
/// A contiguous span of bits in an integer carrier.
#[doc = crate::_doc_meta!{
    location("num/fin", struct BitSpan),
    test_size_of(BitSpan<u8> = 16|128; niche !Option),
    test_size_of(BitSpan<u32> = 20|160; niche !Option),
}]
/// Stores the bounds and derived masks for a packed field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitSpan<T> {
    /// First bit in the span.
    pub start: u32,
    /// Last bit in the span.
    pub end: u32,
    /// Number of bits in the span.
    pub width: u32,
    /// Shifted mask covering the span.
    pub mask: T,
    /// Maximum value that fits in the span.
    pub max: T,
}
impl<T> BitSpan<T> {
    /// Returns a new bit span from precomputed metadata.
    ///
    /// # Panics
    /// Panics if `start > end`.
    #[must_use]
    pub const fn from_parts(start: u32, end: u32, mask: T, max: T) -> Self {
        assert!(start <= end, "BitSpan: start must be <= end");
        Self { start, end, width: end - start + 1, mask, max }
    }
}
impl<T: Copy> BitSpan<T> {
    /// Returns whether the span occupies exactly one bit.
    #[must_use]
    pub const fn is_single(self) -> bool {
        self.width == 1
    }
    /// Returns whether `bit` is inside the span.
    #[must_use]
    pub const fn contains_bit(self, bit: u32) -> bool {
        self.start <= bit && bit <= self.end
    }
    /// Returns whether `other` is fully inside `self`.
    #[must_use]
    pub const fn contains_span(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }
    /// Returns whether both spans share any bits.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
    /// Returns whether both spans share no bits.
    #[must_use]
    pub const fn is_disjoint(self, other: Self) -> bool {
        !self.overlaps(other)
    }
    /// Returns whether the span fits in a carrier with `bits` bits.
    #[must_use]
    pub const fn fits_in(self, bits: u32) -> bool {
        self.start <= self.end && self.end < bits
    }
}

// methods for unsigned integer primitives
macro_rules! impl_bit_span {
    ($($T:ty),+ $(,)?) => { $(
        impl BitSpan<$T> {
            /// Returns a new bit span.
            ///
            /// # Panics
            /// Panics if `start > end` or if `end` exceeds the carrier width.
            #[must_use]
            pub const fn new(start: u32, end: u32) -> Self {
                assert!(start <= end, "BitSpan: start must be <= end");
                assert!(end < <$T>::BITS, "BitSpan: end exceeds carrier width");
                let width = end - start + 1;
                let mask = bitwise![mask $T; (start)..=(end)];
                let max = is![width >= <$T>::BITS, !0 as $T, ((1 as $T) << width) - 1];
                Self { start, end, width, mask, max }
            }
            /// Returns a new checked bit span.
            ///
            /// # Errors
            /// Returns an error if `start > end` or if `end` exceeds the carrier width.
            pub const fn try_new(start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                if start >= <$T>::BITS {
                    Err(MismatchedBounds::IndexOutOfBounds(Some(start as usize)))
                } else if end >= <$T>::BITS {
                    Err(MismatchedBounds::IndexOutOfBounds(Some(end as usize)))
                } else if start > end {
                    Err(MismatchedBounds::MismatchedIndices)
                } else {
                    Ok(Self::new(start, end))
                }
            }

            /// Returns whether `value` fits in this span.
            #[must_use]
            pub const fn value_fits(self, value: $T) -> bool { value <= self.max }

            /// Extracts the span value from `bits`.
            #[must_use]
            pub const fn get_value(self, bits: $T) -> $T {
                bitwise![extract bits; (self.start)..=(self.end)]
            }

            /// Returns `bits` with the span value replaced.
            ///
            /// The value is masked to fit the span width.
            #[must_use]
            pub const fn set_value(self, bits: $T, value: $T) -> $T {
                bitwise![insert bits, value; (self.start)..=(self.end)]
            }
            /// Returns `bits` with the checked span value replaced.
            ///
            /// # Errors
            /// Returns an error if the span is invalid
            /// or if `value` does not fit within the span width.
            pub const fn try_set_value(self, bits: $T, value: $T)
                -> Result<$T, MismatchedBounds> {
                is! { let Err(err) = Self::try_new(self.start, self.end), return Err(err) }
                let width = self.end - self.start + 1;
                if width < <$T>::BITS {
                    let limit = (1 as $T) << width;
                    if value >= limit {
                        let err = MismatchedCapacity::too_large(value as usize, limit as usize);
                        return Err(MismatchedBounds::from_mismatched_capacity(err));
                    }
                }
                Ok(bitwise![insert bits, value; (self.start)..=(self.end)])
            }
            /// Clears the span bits in `bits`.
            #[must_use]
            pub const fn clear_in(self, bits: $T) -> $T { bits & !self.mask }

            /// Returns whether every span bit is set in `bits`.
            #[must_use]
            pub const fn is_full_in(self, bits: $T) -> bool { bits & self.mask == self.mask }

            /// Returns whether no span bit is set in `bits`.
            #[must_use]
            pub const fn is_zero_in(self, bits: $T) -> bool { bits & self.mask == 0 }
        }
    )+ };
}
impl_bit_span![u8, u16, u32, u64, u128, usize];

#[cfg(test)]
mod _test {
    use super::BitSpan;

    #[test]
    fn bit_span_metadata() {
        let s = BitSpan::<u16>::new(3, 6);

        assert_eq!(s.start, 3);
        assert_eq!(s.end, 6);
        assert_eq!(s.width, 4);
        assert_eq!(s.mask, 0b0000_0000_0111_1000);
        assert_eq!(s.max, 0b1111);

        assert!(s.contains_bit(3));
        assert!(s.contains_bit(6));
        assert!(!s.contains_bit(7));
        assert!(s.value_fits(15));
        assert!(!s.value_fits(16));
    }

    #[test]
    fn bit_span_value_ops() {
        let s = BitSpan::<u16>::new(3, 6);

        let bits = s.set_value(0, 0b1010);
        assert_eq!(bits, 0b0101_0000);
        assert_eq!(s.get_value(bits), 0b1010);

        assert!(s.try_set_value(0, 0b1111).is_ok());
        assert!(s.try_set_value(0, 0b1_0000).is_err());

        assert_eq!(s.clear_in(bits), 0);
    }
}
