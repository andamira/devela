// devela::num::niche::asm_validation
//
//! Assembly Optimization Validation.
//!
//! Contains test functions with forced codegen to verify compiler optimizations.
//! Normally commented out, only uncommented for asm inspection via:
//! ```sh
//! # cargo A_x64l -q --lib --everything > asm_safe.s
//! cargo A_x64l -q --lib --everything --features unsafe_niche > asm_unsafe.s
//! ```

#![allow(improper_ctypes_definitions, missing_docs)]

// new() uses 3 distinct patterns (all brancheless, ~1 instruction):
// - NOT (unsigned MAX)  1
// - LEA (signed MIN)    1
// - XOR (all others)    1-2
//
// new_lossy() uses 3 distinct patterns (all branchless, 5 instructions)
// - NOT+CMOV     (unsigned MAX)   5
// - LEA+NEG+CMOV (signed MIN)     5
// - XOR+CMOV     (all others)     5

/* new() */

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_u32_max(value: u32) -> Option<NonValueU32<{ u32::MAX }>> {
    NonValueU32::new(value)
    // mov eax, edi
    // not eax
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_u32_min(value: u32) -> Option<NonValueU32<{ u32::MIN }>> {
    NonValueU32::new(value)
    // mov eax, edi
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_u32_mid(value: u32) -> Option<NonValueU32<50>> {
    NonValueU32::new(value)
    // mov eax, edi
    // xor eax, 50
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_i32_max(value: i32) -> Option<NonValueI32<{ i32::MAX }>> {
    NonValueI32::new(value)
    // mov eax, edi
    // xor eax, 2147483647
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_i32_min(value: i32) -> Option<NonValueI32<{ i32::MIN }>> {
    NonValueI32::new(value)
    // lea eax, [rdi - 2147483648]
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_i32_mid(value: i32) -> Option<NonValueI32<50>> {
    NonValueI32::new(value)
    // mov eax, edi
    // xor eax, 50
    // ret
}

/* new_lossy() (`unsafe_niche` activated) */

#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_u32_max_lossy(value: u32) -> NonValueU32<{ u32::MAX }> {
    NonValueU32::new_lossy(value)
    // cmp edi, -1
    // not edi
    // mov eax, 1
    // cmovne eax, edi
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_u32_min_lossy(value: u32) -> NonValueU32<{ u32::MIN }> {
    NonValueU32::new_lossy(value)
    // mov eax, edi
    // cmp edi, 1
    // adc eax, 0
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_u32_mid_lossy(value: u32) -> NonValueU32<50> {
    NonValueU32::new_lossy(value)
    // mov ecx, edi
    // xor ecx, 50
    // cmp edi, 50
    // mov eax, 3
    // cmovne eax, ecx
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_i32_max_lossy(value: i32) -> NonValueI32<{ i32::MAX }> {
    NonValueI32::new_lossy(value)
    // mov ecx, edi
    // xor ecx, 2147483647
    // cmp edi, 2147483647
    // mov eax, 1
    // cmovne eax, ecx
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_i32_min_lossy(value: i32) -> NonValueI32<{ i32::MIN }> {
    NonValueI32::new_lossy(value)
    // lea ecx, [rdi - 2147483648]
    // neg edi
    // mov eax, 1
    // cmovno eax, ecx
    // ret
}
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn inspect_nonvalue_i32_mid_lossy(value: i32) -> NonValueI32<50> {
    NonValueI32::new_lossy(value)
    // mov ecx, edi
    // xor ecx, 50
    // cmp edi, 50
    // mov eax, 3
    // cmovne eax, ecx
    // ret
}
