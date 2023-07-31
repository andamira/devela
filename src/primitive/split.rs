// devela::primitive::split
//
//! fns to split a primitive into an array of smaller primitives.
//

/* u16 */

/// Splits a `u16` into an array of `[u8; 2]` in big-endian order.
// ```asm-x86-64
// mov eax, edi
// rol ax, 8
// ret
// ```
#[inline]
#[must_use]
pub const fn u16_into_u8_be(v: u16) -> [u8; 2] {
    v.to_be_bytes()
}

/// Splits a `u16` into an array of `[u8; 2]` in little-endian order.
// ```asm-x86-64
// mov eax, edi
// ret
// ```
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
// ```asm-x86-64
// move eax, edi
// rol eax, 16
// ret
// ```
#[inline]
#[must_use]
pub const fn u32_into_u16_be(v: u32) -> [u16; 2] {
    let v0: u16 = ((v >> 16) & u16::MAX as u32) as u16;
    let v1: u16 = (v & u16::MAX as u32) as u16;
    [v0, v1]
}

/// Splits a `u32` into an array of `[u16; 2]` in little-endian order.
// ```asm-x86-64
// mov eax, edi
// ret
// ```
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
// ```asm-x86-64
// mov eax, edi
// bswap eax
// ret
// ```
#[inline]
#[must_use]
pub const fn u32_into_u8_be(v: u32) -> [u8; 4] {
    v.to_be_bytes()
}

/// Splits a `u32` into an array of `[u8; 4]` in little-endian order.
// ```asm-x86-64
// mov eax, edi
// ret
// ```
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
// ```asm-x86-64
// mov rax, rdi
// rol rax, 32
// ret
// ```
#[inline]
#[must_use]
pub const fn u64_into_u32_be(v: u64) -> [u32; 2] {
    let v0: u32 = ((v >> 32) & u32::MAX as u64) as u32;
    let v1: u32 = (v & u32::MAX as u64) as u32;
    [v0, v1]
}

/// Splits a `u64` into an array of `[u32; 2]` in little-endian order.
// ```asm-x86-64
// mov rax, rdi
// ret
// ```
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
// ```asm-x86-64
// mov rcx, rdi
// shr rcx, 48
// mov rdx, rdi
// shl rdx, 48
// mov eax, edi
// and eax, -65536
// shl rax, 16
// or rax, rdx
// shr rdi, 16
// and edi, -65536
// or rax, rdi
// or rax, rcx
// ret
// ```
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
// ```asm-x86-64
// mov rax, rdi
// ret
// ```
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
// ```asm-x86-64
// mov rax, rdi
// bswap rax
// ```
#[inline]
#[must_use]
pub const fn u64_into_u8_be(v: u64) -> [u8; 8] {
    v.to_be_bytes()
}

/// Splits a `u64` into an array of `[u8; 8]` in little-endian order.
// ```asm-x86-64
// mov rax, rdi
// ret
// ```
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
// ```asm-x86-64
// mov rax, rdi
// mov qword ptr [rdi], rdx
// mov qword ptr [rdi + 8], rsi
// ret
// ```
#[inline]
#[must_use]
pub const fn u128_into_u64_be(v: u128) -> [u64; 2] {
    let v0: u64 = (v >> 64) as u64;
    let v1: u64 = (v & u64::MAX as u128) as u64;
    [v0, v1]
}

/// Splits a `u128` into an array of `[u64; 2]` in little-endian order.
// ```asm-x86-64
// mov rax, rdi
// mov qword ptr [rdi], rsi
// mov qword ptr [rdi + 8], rdx
// ret
// ```
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
// ```asm-x86-64
// mov rax, rdi
// movd xmm0, edx
// shr rdx, 32
// movd xmm1, esi
// shr rsi, 32
// movd xmm2, esi
// punpckldq xmm2, xmm1
// movd xmm1, edx
// punpckldq xmm1, xmm0
// punpcklqdq xmm1, xmm2
// movdqu xmmword ptr [rdi], xmm1
// ret
// ```
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
// ```asm-x86-64
// mov rax, rdi
// mov qword ptr [rdi + 8], rdx
// mov qword ptr [rdi], rsi
// ret
// ```
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
// ```asm-x86-64
// mov rax, rdi
// mov rcx, rdx
// mov rdi, rdx
// movd xmm0, edx
// shr rdx, 48
// shr rcx, 32
// shr rdi, 16
// mov r8, rsi
// mov r9, rsi
// movd xmm1, esi
// shr rsi, 48
// shr r8, 32
// shr r9, 16
// movd xmm2, r9d
// punpcklwd xmm2, xmm1
// movd xmm1, r8d
// movd xmm3, esi
// punpcklwd xmm3, xmm1
// punpckldq xmm3, xmm2
// movd xmm1, edi
// punpcklwd xmm1, xmm0
// movd xmm0, ecx
// movd xmm2, edx
// punpcklwd xmm2, xmm0
// punpckldq xmm2, xmm1
// punpcklqdq xmm2, xmm3
// movdqu xmmword ptr [rax], xmm2
// ret
// ```
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
// ```asm-x86-64
// mov rax, rdi
// mov qword ptr [rdi + 8], rdx
// mov qword ptr [rdi], rsi
// ret
// ```
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
// ```asm-x86-64
// bswap rdx
// bswap rsi
// mov rax, rdi
// mov qword ptr [rdi + 8], rsi
// mov qword ptr [rdi], rdx
// ret
// ```
#[inline]
#[must_use]
pub const fn u128_into_u8_be(v: u128) -> [u8; 16] {
    v.to_be_bytes()
}

/// Splits a `u128` into an array of `[u8; 16]` in little-endian order.
// ```asm-x86-64
// mov rax, rdi
// mov qword ptr [rdi + 8], rdx
// mov qword ptr [rdi], rsi
// ret
// ```
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
