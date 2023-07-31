// devela::primitive::join
//
//! construct an unsigned primitive by joining an array of smaller unsigned primitives.
//

/* u16 */

/// Constructs a `u16` from an array of `[u8; 2]` in little-endian order.
// ```asm-x86-64
// move eax, edi
// ret
// ```
#[inline]
#[must_use]
pub const fn u16_from_u8_le(v: [u8; 2]) -> u16 {
    u16::from_le_bytes(v)
}

/* u32 */

/// Constructs a `u32` from an array of `[u16; 2]` in little-endian order.
// ```asm-x86-64
// move eax, edi
// ret
// ```
#[inline]
#[must_use]
pub const fn u32_from_u16_le(v: [u16; 2]) -> u32 {
    ((v[1] as u32) << 16) | (v[0] as u32)
}

/// Constructs a `u32` from an array of `[u8; 4]` in little-endian order.
// ```asm-x86-64
// move eax, edi
// ret
// ```
#[inline]
#[must_use]
pub const fn u32_from_u8_le(v: [u8; 4]) -> u32 {
    u32::from_le_bytes(v)
}

/* u64 */

/// Constructs a `u64` from an array of `[u32; 2]` in little-endian order.
// ```asm-x86-64
// move rax, rdi
// ret
// ```
#[inline]
#[must_use]
pub const fn u64_from_u32_le(v: [u32; 2]) -> u64 {
    ((v[1] as u64) << 32) | (v[0] as u64)
}

/// Constructs a `u64` from an array of `[u16; 4]` in little-endian order.
// ```asm-x86-64
// move rax, rdi
// ret
// ```
#[inline]
#[must_use]
pub const fn u64_from_u16_le(v: [u16; 4]) -> u64 {
    ((v[3] as u64) << (16 * 3))
        | ((v[2] as u64) << (16 * 2))
        | ((v[1] as u64) << 16)
        | (v[0] as u64)
}

/// Constructs a `u64` from an array of `[u8; 8]` in little-endian order.
// ```asm-x86-64
// move rax, rdi
// ret
// ```
#[inline]
#[must_use]
pub const fn u64_from_u8_le(v: [u8; 8]) -> u64 {
    u64::from_le_bytes(v)
}

/* u128 */

/// Constructs a `u128` from an array of `[u64; 2]` in little-endian order.
// ```asm-x86-64
// mov rax, qword ptr [rdi]
// mov rdx, qword ptr [rdi + 8]
// ret
// ```
#[inline]
#[must_use]
pub const fn u128_from_u64_le(v: [u64; 2]) -> u128 {
    ((v[1] as u128) << 64) | (v[0] as u128)
}

/// Constructs a `u128` from an array of `[u32; 4]` in little-endian order.
// ```asm-x86-64
// mov rax, qword ptr [rdi]
// mov rdx, qword ptr [rdi + 8]
// ret
// ```
#[inline]
#[must_use]
pub const fn u128_from_u32_le(v: [u32; 4]) -> u128 {
    ((v[3] as u128) << (32 * 3))
        | ((v[2] as u128) << (32 * 2))
        | ((v[1] as u128) << 32)
        | (v[0] as u128)
}

/// Constructs a `u128` from an array of `[u16; 8]` in little-endian order.
// ```asm-x86-64
// mov rax, qword ptr [rdi]
// mov rdx, qword ptr [rdi + 8]
// ret
// ```
#[inline]
#[must_use]
pub const fn u128_from_u16_le(v: [u16; 8]) -> u128 {
    ((v[7] as u128) << (16 * 7))
        | ((v[6] as u128) << (16 * 6))
        | ((v[5] as u128) << (16 * 5))
        | ((v[4] as u128) << (16 * 4))
        | ((v[3] as u128) << (16 * 3))
        | ((v[2] as u128) << (16 * 2))
        | ((v[1] as u128) << 16)
        | (v[0] as u128)
}

/// Constructs a `u128` from an array of `[u8; 16]` in little-endian order.
// ```asm-x86-64
// mov rax, qword ptr [rdi]
// mov rdx, qword ptr [rdi + 8]
// ret
// ```
#[inline]
#[must_use]
pub const fn u128_from_u8_le(v: [u8; 16]) -> u128 {
    u128::from_le_bytes(v)
}
