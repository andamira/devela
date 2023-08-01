// devela::conversion::primitive::split
//
//! fns to split a primitive into an array of smaller primitives.
//

/* u16 */

/// Splits a `u16` into an array of `[u8; 2]` in big-endian order.
#[inline]
#[must_use]
pub const fn u16_into_u8_be(v: u16) -> [u8; 2] {
    v.to_be_bytes()
}

/// Splits a `u16` into an array of `[u8; 2]` in little-endian order.
#[inline]
#[must_use]
pub const fn u16_into_u8_le(v: u16) -> [u8; 2] {
    v.to_le_bytes()
}

/// Splits a `u16` into an array of `[u8; 2]` in native-endian order.
#[inline]
#[must_use]
pub const fn u16_into_u8_ne(v: u16) -> [u8; 2] {
    v.to_ne_bytes()
}

/* u32 */

/// Splits a `u32` into an array of `[u16; 2]` in big-endian order.
#[inline]
#[must_use]
pub const fn u32_into_u16_be(v: u32) -> [u16; 2] {
    let v0: u16 = ((v >> 16) & u16::MAX as u32) as u16;
    let v1: u16 = (v & u16::MAX as u32) as u16;
    [v0, v1]
}

/// Splits a `u32` into an array of `[u16; 2]` in little-endian order.
#[inline]
#[must_use]
pub const fn u32_into_u16_le(v: u32) -> [u16; 2] {
    let v1: u16 = ((v >> 16) & u16::MAX as u32) as u16;
    let v0: u16 = (v & u16::MAX as u32) as u16;
    [v0, v1]
}

/// Splits a `u32` into an array of `[u16; 2]` in native-endian order.
#[inline]
#[must_use]
pub const fn u32_into_u16_ne(v: u32) -> [u16; 2] {
    if cfg!(target_endian = "big") {
        u32_into_u16_be(v)
    } else {
        u32_into_u16_le(v)
    }
}

/// Splits a `u32` into an array of `[u8; 4]` in big-endian order.
#[inline]
#[must_use]
pub const fn u32_into_u8_be(v: u32) -> [u8; 4] {
    v.to_be_bytes()
}

/// Splits a `u32` into an array of `[u8; 4]` in little-endian order.
#[inline]
#[must_use]
pub const fn u32_into_u8_le(v: u32) -> [u8; 4] {
    v.to_le_bytes()
}

/// Splits a `u32` into an array of `[u8; 4]` in native-endian order.
#[inline]
#[must_use]
pub const fn u32_into_u8_ne(v: u32) -> [u8; 4] {
    v.to_ne_bytes()
}

/* u64 */

/// Splits a `u64` into an array of `[u32; 2]` in big-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u32_be(v: u64) -> [u32; 2] {
    let v0: u32 = ((v >> 32) & u32::MAX as u64) as u32;
    let v1: u32 = (v & u32::MAX as u64) as u32;
    [v0, v1]
}

/// Splits a `u64` into an array of `[u32; 2]` in little-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u32_le(v: u64) -> [u32; 2] {
    let v1: u32 = ((v >> 32) & u32::MAX as u64) as u32;
    let v0: u32 = (v & u32::MAX as u64) as u32;
    [v0, v1]
}

/// Splits a `u64` into an array of `[u32; 2]` in native-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u32_ne(v: u64) -> [u32; 2] {
    if cfg!(target_endian = "big") {
        u64_into_u32_be(v)
    } else {
        u64_into_u32_le(v)
    }
}

/// Splits a `u64` into an array of `[u16; 4]` in big-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u16_be(v: u64) -> [u16; 4] {
    let v0: u16 = ((v >> (16 * 3)) & u16::MAX as u64) as u16;
    let v1: u16 = ((v >> (16 * 2)) & u16::MAX as u64) as u16;
    let v2: u16 = ((v >> 16) & u16::MAX as u64) as u16;
    let v3: u16 = (v & u16::MAX as u64) as u16;
    [v0, v1, v2, v3]
}

/// Splits a `u64` into an array of `[u16; 4]` in little-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u16_le(v: u64) -> [u16; 4] {
    let v3: u16 = ((v >> (16 * 3)) & u16::MAX as u64) as u16;
    let v2: u16 = ((v >> (16 * 2)) & u16::MAX as u64) as u16;
    let v1: u16 = ((v >> 16) & u16::MAX as u64) as u16;
    let v0: u16 = (v & u16::MAX as u64) as u16;
    [v0, v1, v2, v3]
}

/// Splits a `u64` into an array of `[u16; 4]` in native-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u16_ne(v: u64) -> [u16; 4] {
    if cfg!(target_endian = "big") {
        u64_into_u16_be(v)
    } else {
        u64_into_u16_le(v)
    }
}

/// Splits a `u64` into an array of `[u8; 8]` in big-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u8_be(v: u64) -> [u8; 8] {
    v.to_be_bytes()
}

/// Splits a `u64` into an array of `[u8; 8]` in little-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u8_le(v: u64) -> [u8; 8] {
    v.to_le_bytes()
}

/// Splits a `u64` into an array of `[u8; 8]` in native-endian order.
#[inline]
#[must_use]
pub const fn u64_into_u8_ne(v: u64) -> [u8; 8] {
    if cfg!(target_endian = "big") {
        u64_into_u8_be(v)
    } else {
        u64_into_u8_le(v)
    }
}

/* u128 */

/// Splits a `u128` into an array of `[u64; 2]` in big-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u64_be(v: u128) -> [u64; 2] {
    let v0: u64 = (v >> 64) as u64;
    let v1: u64 = (v & u64::MAX as u128) as u64;
    [v0, v1]
}

/// Splits a `u128` into an array of `[u64; 2]` in little-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u64_le(v: u128) -> [u64; 2] {
    let v1: u64 = (v >> 64) as u64;
    let v0: u64 = (v & u64::MAX as u128) as u64;
    [v0, v1]
}

/// Splits a `u128` into an array of `[u64; 2]` in native-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u64_ne(v: u128) -> [u64; 2] {
    if cfg!(target_endian = "big") {
        u128_into_u64_be(v)
    } else {
        u128_into_u64_le(v)
    }
}

/// Splits a `u128` into an array of `[u32; 4]` in big-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u32_be(v: u128) -> [u32; 4] {
    let v0: u32 = (v >> (32 * 3)) as u32;
    let v1: u32 = ((v >> (32 * 2)) & u32::MAX as u128) as u32;
    let v2: u32 = ((v >> 32) & u32::MAX as u128) as u32;
    let v3: u32 = (v & u32::MAX as u128) as u32;
    [v0, v1, v2, v3]
}

/// Splits a `u128` into an array of `[u32; 4]` in little-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u32_le(v: u128) -> [u32; 4] {
    let v3: u32 = (v >> (32 * 3)) as u32;
    let v2: u32 = ((v >> (32 * 2)) & u32::MAX as u128) as u32;
    let v1: u32 = ((v >> 32) & u32::MAX as u128) as u32;
    let v0: u32 = (v & u32::MAX as u128) as u32;
    [v0, v1, v2, v3]
}

/// Splits a `u128` into an array of `[u32; 4]` in native-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u32_ne(v: u128) -> [u32; 4] {
    if cfg!(target_endian = "big") {
        u128_into_u32_be(v)
    } else {
        u128_into_u32_le(v)
    }
}

/// Splits a `u128` into an array of `[u16; 8]` in big-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u16_be(v: u128) -> [u16; 8] {
    let v0: u16 = (v >> (16 * 7)) as u16;
    let v1: u16 = ((v >> (16 * 6)) & u16::MAX as u128) as u16;
    let v2: u16 = ((v >> (16 * 5)) & u16::MAX as u128) as u16;
    let v3: u16 = ((v >> (16 * 4)) & u16::MAX as u128) as u16;
    let v4: u16 = ((v >> (16 * 3)) & u16::MAX as u128) as u16;
    let v5: u16 = ((v >> (16 * 2)) & u16::MAX as u128) as u16;
    let v6: u16 = ((v >> 16) & u16::MAX as u128) as u16;
    let v7: u16 = (v & u16::MAX as u128) as u16;
    [v0, v1, v2, v3, v4, v5, v6, v7]
}

/// Splits a `u128` into an array of `[u16; 8]` in little-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u16_le(v: u128) -> [u16; 8] {
    let v7: u16 = (v >> (16 * 7)) as u16;
    let v6: u16 = ((v >> (16 * 6)) & u16::MAX as u128) as u16;
    let v5: u16 = ((v >> (16 * 5)) & u16::MAX as u128) as u16;
    let v4: u16 = ((v >> (16 * 4)) & u16::MAX as u128) as u16;
    let v3: u16 = ((v >> (16 * 3)) & u16::MAX as u128) as u16;
    let v2: u16 = ((v >> (16 * 2)) & u16::MAX as u128) as u16;
    let v1: u16 = ((v >> 16) & u16::MAX as u128) as u16;
    let v0: u16 = (v & u16::MAX as u128) as u16;
    [v0, v1, v2, v3, v4, v5, v6, v7]
}

/// Splits a `u128` into an array of `[u16; 8]` in native-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u16_ne(v: u128) -> [u16; 8] {
    if cfg!(target_endian = "big") {
        u128_into_u16_be(v)
    } else {
        u128_into_u16_le(v)
    }
}

/// Splits a `u128` into an array of `[u8; 16]` in big-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u8_be(v: u128) -> [u8; 16] {
    v.to_be_bytes()
}

/// Splits a `u128` into an array of `[u8; 16]` in little-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u8_le(v: u128) -> [u8; 16] {
    v.to_le_bytes()
}

/// Splits a `u128` into an array of `[u8; 16]` in native-endian order.
#[inline]
#[must_use]
pub const fn u128_into_u8_ne(v: u128) -> [u8; 16] {
    v.to_ne_bytes()
}
