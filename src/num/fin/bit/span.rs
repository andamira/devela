// devela::num::fin::bit::span
//
//! Defines [`BitSpan`].
//

use crate::{Bitwise, MismatchedBounds};

#[doc = crate::_tags!(bit data_structure)]
/// A contiguous span of bits in an integer carrier.
#[doc = crate::_doc_meta!{location("bin")}]
///
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
                let mask = Bitwise::<$T>::mask_range(start, end).0;
                let max = if width >= <$T>::BITS {
                    !0 as $T
                } else {
                    ((1 as $T) << width) - 1
                };
                Self { start, end, width, mask, max }
            }
            /// Returns a new checked bit span.
            ///
            /// # Errors
            /// Returns an error if `start > end` or if `end` exceeds the carrier width.
            pub const fn try_new(start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Bitwise::<$T>::mask_range_checked(start, end) {
                    Ok(mask) => {
                        let width = end - start + 1;
                        let max = if width >= <$T>::BITS {
                            !0 as $T
                        } else {
                            ((1 as $T) << width) - 1
                        };
                        Ok(Self { start, end, width, mask: mask.0, max })
                    }
                    Err(e) => Err(e),
                }
            }

            /// Returns whether `value` fits in this span.
            #[must_use]
            pub const fn value_fits(self, value: $T) -> bool { value <= self.max }

            /// Extracts the span value from `bits`.
            #[must_use]
            pub const fn get_value(self, bits: $T) -> $T {
                Bitwise::<$T>(bits).get_value_range(self.start, self.end).0
            }

            /// Returns `bits` with the span value replaced.
            ///
            /// The value is masked to fit the span width.
            #[must_use]
            pub const fn set_value(self, bits: $T, value: $T) -> $T {
                Bitwise::<$T>(bits).set_value_range(value, self.start, self.end).0
            }
            /// Returns `bits` with the checked span value replaced.
            ///
            /// # Errors
            /// Returns an error if the span is invalid
            /// or if `value` does not fit within the span width.
            pub const fn try_set_value(self, bits: $T, value: $T)
                -> Result<$T, MismatchedBounds> {
                match Bitwise::<$T>(bits)
                    .set_value_range_checked_strict(value, self.start, self.end)
                {
                    Ok(bits) => Ok(bits.0),
                    Err(e) => Err(e),
                }
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
mod tests {
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
