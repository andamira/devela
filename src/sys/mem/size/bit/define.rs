// devela/src/sys/mem/size/bit/define.rs
//
//! Defines [`BitSized`].
//

use crate::{ByteSized, Mem};

#[doc = crate::_tags!(mem)]
/// Type size information in bits.
#[doc = crate::_doc_meta!{location("sys/mem")}]
///
/// Indicates a size of exactly `LEN` bits for the relevant data part of this type.
///
/// E.g. a `bool` has a BitSized of 1 bit.
pub trait BitSized<const LEN: usize>: ByteSized {
    /// The bit size of this type (only the relevant data part, without padding).
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSized::BYTE_SIZE],
    const BIT_SIZE: usize = {
        let min_byte_size = Mem::bytes_from_bits(LEN);
        if min_byte_size > Self::BYTE_SIZE {
            panic!["BitSized::MIN_BYTE_SIZE > ByteSized::BYTE_SIZE"];
        }
        LEN
    };

    /// The rounded up byte size for this type.
    ///
    /// This is the minimum number of full bytes needed to represent this type.
    /// Basically `(LEN + 7) / 8`.
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSized::BYTE_SIZE],
    const MIN_BYTE_SIZE: usize = {
        let min_byte_size = Mem::bytes_from_bits(LEN);
        if min_byte_size > Self::BYTE_SIZE {
            panic!["BitSized::MIN_BYTE_SIZE > ByteSized::BYTE_SIZE"];
        }
        min_byte_size
    };

    /// Returns the bit size of this type (only the relevant data part, without padding).
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSized::BYTE_SIZE],
    #[must_use]
    fn bit_size(&self) -> usize {
        Self::BIT_SIZE
    }

    /// Returns the rounded up byte size for this type.
    ///
    /// This is the minimum number of full bytes needed to represent this type.
    /// Basically `(LEN + 7) / 8`.
    ///
    /// # Panics
    /// Panics if `MIN_BYTE_SIZE > `[`BYTE_SIZE`][ByteSized::BYTE_SIZE],
    #[must_use]
    fn min_byte_size(&self) -> usize {
        Self::MIN_BYTE_SIZE
    }
}
