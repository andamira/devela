// devela_base_core::num::grain::cast::join
//
//! Construct an unsigned primitive by joining an array of smaller unsigned primitives.
//

use crate::Cast;

#[rustfmt::skip]
impl Cast<u16> {
    #[must_use]
    #[inline(always)]
    /// Constructs a `u16` from an array of `[u8; 2]` in big-endian order.
    pub const fn from_u8_be(v: [u8; 2]) -> u16 { u16::from_be_bytes(v) }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u16` from an array of `[u8; 2]` in little-endian order.
    pub const fn from_u8_le(v: [u8; 2]) -> u16 { u16::from_le_bytes(v) }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u16` from an array of `[u8; 2]` in native-endian order.
    pub const fn from_u8_ne(v: [u8; 2]) -> u16 { u16::from_ne_bytes(v) }
}

#[rustfmt::skip]
impl Cast<u32> {
    #[must_use]
    /// Constructs a `u32` from an array of `[u16; 2]` in big-endian order.
    pub const fn from_u16_be(v: [u16; 2]) -> u32 {
        ((v[0] as u32) << 16) | (v[1] as u32)
    }

    #[must_use]
    /// Constructs a `u32` from an array of `[u16; 2]` in little-endian order.
    pub const fn from_u16_le(v: [u16; 2]) -> u32 {
        ((v[1] as u32) << 16) | (v[0] as u32)
    }

    #[must_use]
    /// Constructs a `u32` from an array of `[u16; 2]` in native-endian order.
    pub const fn from_u16_ne(v: [u16; 2]) -> u32 {
        if cfg!(target_endian = "big") {
            Cast::<u32>::from_u16_be(v)
        } else {
            Cast::<u32>::from_u16_le(v)
        }
    }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u32` from an array of `[u8; 4]` in big-endian order.
    pub const fn from_u8_be(v: [u8; 4]) -> u32 { u32::from_be_bytes(v) }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u32` from an array of `[u8; 4]` in little-endian order.
    pub const fn from_u8_le(v: [u8; 4]) -> u32 { u32::from_le_bytes(v) }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u32` from an array of `[u8; 4]` in native-endian order.
    pub const fn from_u8_ne(v: [u8; 4]) -> u32 { u32::from_ne_bytes(v) }
}

#[rustfmt::skip]
impl Cast<u64> {
    #[must_use]
    /// Constructs a `u64` from an array of `[u32; 2]` in big-endian order.
    pub const fn from_u32_be(v: [u32; 2]) -> u64 {
        ((v[0] as u64) << 32) | (v[1] as u64)
    }

    #[must_use]
    /// Constructs a `u64` from an array of `[u32; 2]` in little-endian order.
    pub const fn from_u32_le(v: [u32; 2]) -> u64 {
        ((v[1] as u64) << 32) | (v[0] as u64)
    }

    #[must_use]
    /// Constructs a `u64` from an array of `[u32; 2]` in native-endian order.
    pub const fn from_u32_ne(v: [u32; 2]) -> u64 {
        if cfg!(target_endian = "big") {
            Cast::<u64>::from_u32_be(v)
        } else {
            Cast::<u64>::from_u32_le(v)
        }
    }

    #[must_use]
    /// Constructs a `u64` from an array of `[u16; 4]` in big-endian order.
    pub const fn from_u16_be(v: [u16; 4]) -> u64 {
        ((v[0] as u64) << (16 * 3))
            | ((v[1] as u64) << (16 * 2))
            | ((v[2] as u64) << 16)
            | (v[3] as u64)
    }

    #[must_use]
    /// Constructs a `u64` from an array of `[u16; 4]` in little-endian order.
    pub const fn from_u16_le(v: [u16; 4]) -> u64 {
        ((v[3] as u64) << (16 * 3))
            | ((v[2] as u64) << (16 * 2))
            | ((v[1] as u64) << 16)
            | (v[0] as u64)
    }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u64` from an array of `[u16; 4]` in native-endian order.
    pub const fn from_u16_ne(v: [u16; 4]) -> u64 {
        if cfg!(target_endian = "big") {
            Cast::<u64>::from_u16_be(v)
        } else {
            Cast::<u64>::from_u16_le(v)
        }
    }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u64` from an array of `[u8; 8]` in big-endian order.
    pub const fn from_u8_be(v: [u8; 8]) -> u64 { u64::from_be_bytes(v) }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u64` from an array of `[u8; 8]` in little-endian order.
    pub const fn from_u8_le(v: [u8; 8]) -> u64 { u64::from_le_bytes(v) }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u64` from an array of `[u8; 8]` in native-endian order.
    pub const fn from_u8_ne(v: [u8; 8]) -> u64 { u64::from_ne_bytes(v) }
}

#[rustfmt::skip]
impl Cast<u128> {
    #[must_use]
    /// Constructs a `u128` from an array of `[u64; 2]` in big-endian order.
    pub const fn from_u64_be(v: [u64; 2]) -> u128 {
        ((v[0] as u128) << 64) | (v[1] as u128)
    }

    #[must_use]
    /// Constructs a `u128` from an array of `[u64; 2]` in little-endian order.
    pub const fn from_u64_le(v: [u64; 2]) -> u128 {
        ((v[1] as u128) << 64) | (v[0] as u128)
    }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u128` from an array of `[u64; 2]` in native-endian order.
    pub const fn from_u64_ne(v: [u64; 2]) -> u128 {
        if cfg!(target_endian = "big") {
            Cast::<u128>::from_u64_be(v)
        } else {
            Cast::<u128>::from_u64_le(v)
        }
    }

    #[must_use]
    /// Constructs a `u128` from an array of `[u32; 4]` in big-endian order.
    pub const fn from_u32_be(v: [u32; 4]) -> u128 {
        ((v[0] as u128) << (32 * 3))
            | ((v[1] as u128) << (32 * 2))
            | ((v[2] as u128) << 32)
            | (v[3] as u128)
    }

    #[must_use]
    /// Constructs a `u128` from an array of `[u32; 4]` in little-endian order.
    pub const fn from_u32_le(v: [u32; 4]) -> u128 {
        ((v[3] as u128) << (32 * 3))
            | ((v[2] as u128) << (32 * 2))
            | ((v[1] as u128) << 32)
            | (v[0] as u128)
    }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u128` from an array of `[u32; 4]` in native-endian order.
    pub const fn from_u32_ne(v: [u32; 4]) -> u128 {
        if cfg!(target_endian = "big") {
            Cast::<u128>::from_u32_be(v)
        } else {
            Cast::<u128>::from_u32_le(v)
        }
    }

    #[must_use]
    /// Constructs a `u128` from an array of `[u16; 8]` in big-endian order.
    pub const fn from_u16_be(v: [u16; 8]) -> u128 {
        ((v[0] as u128) << (16 * 7))
            | ((v[1] as u128) << (16 * 6))
            | ((v[2] as u128) << (16 * 5))
            | ((v[3] as u128) << (16 * 4))
            | ((v[4] as u128) << (16 * 3))
            | ((v[5] as u128) << (16 * 2))
            | ((v[6] as u128) << 16)
            | (v[7] as u128)
    }

    #[must_use]
    /// Constructs a `u128` from an array of `[u16; 8]` in little-endian order.
    pub const fn from_u16_le(v: [u16; 8]) -> u128 {
        ((v[7] as u128) << (16 * 7))
            | ((v[6] as u128) << (16 * 6))
            | ((v[5] as u128) << (16 * 5))
            | ((v[4] as u128) << (16 * 4))
            | ((v[3] as u128) << (16 * 3))
            | ((v[2] as u128) << (16 * 2))
            | ((v[1] as u128) << 16)
            | (v[0] as u128)
    }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u128` from an array of `[u16; 8]` in native-endian order.
    pub const fn from_u16_ne(v: [u16; 8]) -> u128 {
        if cfg!(target_endian = "big") {
            Cast::<u128>::from_u16_be(v)
        } else {
            Cast::<u128>::from_u16_le(v)
        }
    }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u128` from an array of `[u8; 16]` in big-endian order.
    pub const fn from_u8_be(v: [u8; 16]) -> u128 { u128::from_be_bytes(v) }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u128` from an array of `[u8; 16]` in little-endian order.
    pub const fn from_u8_le(v: [u8; 16]) -> u128 { u128::from_le_bytes(v) }

    #[must_use]
    #[inline(always)]
    /// Constructs a `u128` from an array of `[u8; 16]` in native-endian order.
    pub const fn from_u8_ne(v: [u8; 16]) -> u128 { u128::from_ne_bytes(v) }
}
