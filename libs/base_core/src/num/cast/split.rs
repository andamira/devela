// devela_base_core::data::conversion::cast::split
//
//! fns to split a primitive into an array of smaller primitives.
//

use crate::Cast;

#[rustfmt::skip]
impl Cast<u16> {
    #[must_use]
    #[inline(always)]
    /// Splits a `u16` into an array of `[u8; 2]` in big-endian order.
    pub const fn into_u8_be(self) -> [u8; 2] { self.0.to_be_bytes() }

    #[must_use]
    #[inline(always)]
    /// Splits a `u16` into an array of `[u8; 2]` in little-endian order.
    pub const fn into_u8_le(self) -> [u8; 2] { self.0.to_le_bytes() }

    #[must_use]
    #[inline(always)]
    /// Splits a `u16` into an array of `[u8; 2]` in native-endian order.
    pub const fn into_u8_ne(self) -> [u8; 2] { self.0.to_ne_bytes() }
}

#[rustfmt::skip]
impl Cast<u32> {
    #[must_use]
    /// Splits a `u32` into an array of `[u16; 2]` in big-endian order.
    pub const fn into_u16_be(self) -> [u16; 2] {
        let v0: u16 = ((self.0 >> 16) & u16::MAX as u32) as u16;
        let v1: u16 = (self.0 & u16::MAX as u32) as u16;
        [v0, v1]
    }

    #[must_use]
    /// Splits a `u32` into an array of `[u16; 2]` in little-endian order.
    pub const fn into_u16_le(self) -> [u16; 2] {
        let v1: u16 = ((self.0 >> 16) & u16::MAX as u32) as u16;
        let v0: u16 = (self.0 & u16::MAX as u32) as u16;
        [v0, v1]
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u32` into an array of `[u16; 2]` in native-endian order.
    pub const fn into_u16_ne(self) -> [u16; 2] {
        if cfg!(target_endian = "big") {
            Cast::<u32>::into_u16_be(self)
        } else {
            Cast::<u32>::into_u16_le(self)
        }
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u32` into an array of `[u8; 4]` in big-endian order.
    pub const fn into_u8_be(self) -> [u8; 4] { self.0.to_be_bytes() }

    #[must_use]
    #[inline(always)]
    /// Splits a `u32` into an array of `[u8; 4]` in little-endian order.
    pub const fn into_u8_le(self) -> [u8; 4] { self.0.to_le_bytes() }

    #[must_use]
    #[inline(always)]
    /// Splits a `u32` into an array of `[u8; 4]` in native-endian order.
    pub const fn into_u8_ne(self) -> [u8; 4] { self.0.to_ne_bytes() }
}

#[rustfmt::skip]
impl Cast<u64> {
    #[must_use]
    /// Splits a `u64` into an array of `[u32; 2]` in big-endian order.
    pub const fn into_u32_be(self) -> [u32; 2] {
        let v0: u32 = ((self.0 >> 32) & u32::MAX as u64) as u32;
        let v1: u32 = (self.0 & u32::MAX as u64) as u32;
        [v0, v1]
    }

    #[must_use]
    /// Splits a `u64` into an array of `[u32; 2]` in little-endian order.
    pub const fn into_u32_le(self) -> [u32; 2] {
        let v1: u32 = ((self.0 >> 32) & u32::MAX as u64) as u32;
        let v0: u32 = (self.0 & u32::MAX as u64) as u32;
        [v0, v1]
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u64` into an array of `[u32; 2]` in native-endian order.
    pub const fn into_u32_ne(self) -> [u32; 2] {
        if cfg!(target_endian = "big") {
            Cast::<u64>::into_u32_be(self)
        } else {
            Cast::<u64>::into_u32_le(self)
        }
    }

    #[must_use]
    /// Splits a `u64` into an array of `[u16; 4]` in big-endian order.
    pub const fn into_u16_be(self) -> [u16; 4] {
        let v0: u16 = ((self.0 >> (16 * 3)) & u16::MAX as u64) as u16;
        let v1: u16 = ((self.0 >> (16 * 2)) & u16::MAX as u64) as u16;
        let v2: u16 = ((self.0 >> 16) & u16::MAX as u64) as u16;
        let v3: u16 = (self.0 & u16::MAX as u64) as u16;
        [v0, v1, v2, v3]
    }

    #[must_use]
    /// Splits a `u64` into an array of `[u16; 4]` in little-endian order.
    pub const fn into_u16_le(self) -> [u16; 4] {
        let v3: u16 = ((self.0 >> (16 * 3)) & u16::MAX as u64) as u16;
        let v2: u16 = ((self.0 >> (16 * 2)) & u16::MAX as u64) as u16;
        let v1: u16 = ((self.0 >> 16) & u16::MAX as u64) as u16;
        let v0: u16 = (self.0 & u16::MAX as u64) as u16;
        [v0, v1, v2, v3]
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u64` into an array of `[u16; 4]` in native-endian order.
    pub const fn into_u16_ne(self) -> [u16; 4] {
        if cfg!(target_endian = "big") {
            Cast::<u64>::into_u16_be(self)
        } else {
            Cast::<u64>::into_u16_le(self)
        }
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u64` into an array of `[u8; 8]` in big-endian order.
    pub const fn into_u8_be(self) -> [u8; 8] { self.0.to_be_bytes() }

    #[must_use]
    #[inline(always)]
    /// Splits a `u64` into an array of `[u8; 8]` in little-endian order.
    pub const fn into_u8_le(self) -> [u8; 8] { self.0.to_le_bytes() }

    #[must_use]
    #[inline(always)]
    /// Splits a `u64` into an array of `[u8; 8]` in native-endian order.
    pub const fn into_u8_ne(self) -> [u8; 8] {
        if cfg!(target_endian = "big") {
            Cast::<u64>::into_u8_be(self)
        } else {
            Cast::<u64>::into_u8_le(self)
        }
    }
}

#[rustfmt::skip]
impl Cast<u128> {
    #[must_use]
    /// Splits a `u128` into an array of `[u64; 2]` in big-endian order.
    pub const fn into_u64_be(self) -> [u64; 2] {
        let v0: u64 = (self.0 >> 64) as u64;
        let v1: u64 = (self.0 & u64::MAX as u128) as u64;
        [v0, v1]
    }

    #[must_use]
    /// Splits a `u128` into an array of `[u64; 2]` in little-endian order.
    pub const fn into_u64_le(self) -> [u64; 2] {
        let v1: u64 = (self.0 >> 64) as u64;
        let v0: u64 = (self.0 & u64::MAX as u128) as u64;
        [v0, v1]
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u128` into an array of `[u64; 2]` in native-endian order.
    pub const fn into_u64_ne(self) -> [u64; 2] {
        if cfg!(target_endian = "big") {
            Cast::<u128>::into_u64_be(self)
        } else {
            Cast::<u128>::into_u64_le(self)
        }
    }

    #[must_use]
    /// Splits a `u128` into an array of `[u32; 4]` in big-endian order.
    pub const fn into_u32_be(self) -> [u32; 4] {
        let v0: u32 = (self.0 >> (32 * 3)) as u32;
        let v1: u32 = ((self.0 >> (32 * 2)) & u32::MAX as u128) as u32;
        let v2: u32 = ((self.0 >> 32) & u32::MAX as u128) as u32;
        let v3: u32 = (self.0 & u32::MAX as u128) as u32;
        [v0, v1, v2, v3]
    }

    #[must_use]
    /// Splits a `u128` into an array of `[u32; 4]` in little-endian order.
    pub const fn into_u32_le(self) -> [u32; 4] {
        let v3: u32 = (self.0 >> (32 * 3)) as u32;
        let v2: u32 = ((self.0 >> (32 * 2)) & u32::MAX as u128) as u32;
        let v1: u32 = ((self.0 >> 32) & u32::MAX as u128) as u32;
        let v0: u32 = (self.0 & u32::MAX as u128) as u32;
        [v0, v1, v2, v3]
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u128` into an array of `[u32; 4]` in native-endian order.
    pub const fn into_u32_ne(self) -> [u32; 4] {
        if cfg!(target_endian = "big") {
            Cast::<u128>::into_u32_be(self)
        } else {
            Cast::<u128>::into_u32_le(self)
        }
    }

    #[must_use]
    /// Splits a `u128` into an array of `[u16; 8]` in big-endian order.
    pub const fn into_u16_be(self) -> [u16; 8] {
        let v0: u16 = (self.0 >> (16 * 7)) as u16;
        let v1: u16 = ((self.0 >> (16 * 6)) & u16::MAX as u128) as u16;
        let v2: u16 = ((self.0 >> (16 * 5)) & u16::MAX as u128) as u16;
        let v3: u16 = ((self.0 >> (16 * 4)) & u16::MAX as u128) as u16;
        let v4: u16 = ((self.0 >> (16 * 3)) & u16::MAX as u128) as u16;
        let v5: u16 = ((self.0 >> (16 * 2)) & u16::MAX as u128) as u16;
        let v6: u16 = ((self.0 >> 16) & u16::MAX as u128) as u16;
        let v7: u16 = (self.0 & u16::MAX as u128) as u16;
        [v0, v1, v2, v3, v4, v5, v6, v7]
    }

    #[must_use]
    /// Splits a `u128` into an array of `[u16; 8]` in little-endian order.
    pub const fn into_u16_le(self) -> [u16; 8] {
        let v7: u16 = (self.0 >> (16 * 7)) as u16;
        let v6: u16 = ((self.0 >> (16 * 6)) & u16::MAX as u128) as u16;
        let v5: u16 = ((self.0 >> (16 * 5)) & u16::MAX as u128) as u16;
        let v4: u16 = ((self.0 >> (16 * 4)) & u16::MAX as u128) as u16;
        let v3: u16 = ((self.0 >> (16 * 3)) & u16::MAX as u128) as u16;
        let v2: u16 = ((self.0 >> (16 * 2)) & u16::MAX as u128) as u16;
        let v1: u16 = ((self.0 >> 16) & u16::MAX as u128) as u16;
        let v0: u16 = (self.0 & u16::MAX as u128) as u16;
        [v0, v1, v2, v3, v4, v5, v6, v7]
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u128` into an array of `[u16; 8]` in native-endian order.
    pub const fn into_u16_ne(self) -> [u16; 8] {
        if cfg!(target_endian = "big") {
            Cast::<u128>::into_u16_be(self)
        } else {
            Cast::<u128>::into_u16_le(self)
        }
    }

    #[must_use]
    #[inline(always)]
    /// Splits a `u128` into an array of `[u8; 16]` in big-endian order.
    pub const fn into_u8_be(self) -> [u8; 16] { self.0.to_be_bytes() }

    #[must_use]
    #[inline(always)]
    /// Splits a `u128` into an array of `[u8; 16]` in little-endian order.
    pub const fn into_u8_le(self) -> [u8; 16] { self.0.to_le_bytes() }

    #[must_use]
    #[inline(always)]
    /// Splits a `u128` into an array of `[u8; 16]` in native-endian order.
    pub const fn into_u8_ne(self) -> [u8; 16] { self.0.to_ne_bytes() }
}
