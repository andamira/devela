// devela::sys::arch::namespace
//
//! Defines the [`Arch`] namespace.
//
// TOC
// - impl Arch blocks
// - macro helpers
//   - impl_arch
//   - arch_fn

#![allow(clippy::too_many_arguments)]

#[cfg(feature = "dep_safe_arch")]
use crate::_dep::safe_arch::*;

#[doc = crate::TAG_NAMESPACE!()]
/// Arch-related functionality.
///
/// ---
/// Implementations that depend on: `dep_safe_arch`, (`x86` or `x86_64`)
/// and the respective target feature:
/// - [none](#functions-not-requiring-any-target-feature).
/// - [`adx`](#functions-requiring-the-adx-target-feature).
/// - [`aes`](#functions-requiring-the-aes-target-feature).
/// - [`avx`](#functions-requiring-the-avx-target-feature).
/// - [`avx2`](#functions-requiring-the-avx2-target-feature).
/// - [`bmi1`](#functions-requiring-the-bmi1-target-feature).
/// - [`bmi2`](#functions-requiring-the-bmi2-target-feature).
/// - [`fma`](#functions-requiring-the-fma-target-feature).
/// - [`lzcnt`](#functions-requiring-the-lzcnt-target-feature).
/// - [`pclmulqdq`](#functions-requiring-the-pclmulqdq-target-feature).
/// - [`popcnt`](#functions-requiring-the-popcnt-target-feature).
/// - [`rdrand`](#functions-requiring-the-rdrand-target-feature).
/// - [`rdseed`](#functions-requiring-the-rdseed-target-feature).
/// - [`sse`](#functions-requiring-the-sse-target-feature)
///   ([generic](#generic-functions-requiring-the-sse-target-feature)).
/// - [`sse2`](#functions-requiring-the-sse2-target-feature).
/// - [`sse3`](#functions-requiring-the-sse3-target-feature).
/// - [`sse4.1`](#functions-requiring-the-sse41-target-feature).
/// - [`sse4.2`](#functions-requiring-the-sse42-target-feature).
/// - [`ssse3`](#functions-requiring-the-ssse3-target-feature).
pub struct Arch;

impl_arch! {
    #[doc = "# Functions not requiring any target feature.\n\n---"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64";
    arch_fn! {
        "Swap the bytes of the given 32-bit value.",
        byte_swap_i32(i: i32) -> i32;
        "Swap the bytes of the given 64-bit value.",
        byte_swap_i64(i: i64) -> i64;
        "Reads the CPU’s timestamp counter value.",
        read_timestamp_counter() -> u64;
        "Reads the CPU’s timestamp counter value and store the processor signature.",
        read_timestamp_counter_p(aux: &mut u32) -> u64;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `adx` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/Intel_ADX>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "adx";
    arch_fn! {
        "Add two `u32` with a carry value.",
        add_carry_u32(c_in: u8, a: u32, b: u32, out: &mut u32) -> u8;
        "Add two `u64` with a carry value.",
        add_carry_u64(c_in: u8, a: u64, b: u64, out: &mut u64) -> u8;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `aes` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/AES_instruction_set>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "aes";
    arch_fn! {
        "Perform the last round of an AES decryption flow on `a` using the `round_key`.",
        aes_decrypt_last_m128i(a: m128i, round_key: m128i) -> m128i;
        "Perform one round of an AES decryption flow on `a` using the `round_key`.",
        aes_decrypt_m128i(a: m128i, round_key: m128i) -> m128i;
        "Perform the last round of an AES encryption flow on `a` using the `round_key`.",
        aes_encrypt_last_m128i(a: m128i, round_key: m128i) -> m128i;
        "Perform one round of an AES encryption flow on `a` using the `round_key`.",
        aes_encrypt_m128i(a: m128i, round_key: m128i) -> m128i;
        "Perform the InvMixColumns transform on `a`.",
        aes_inv_mix_columns_m128i(a: m128i) -> m128i;
        "Assist in expanding an AES cipher key.",
        aes_key_gen_assist_m128i<const IMM: i32>(a: m128i) -> m128i;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `avx` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/Advanced_Vector_Extensions>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "avx";
    arch_fn! {
        "Add adjacent `f32` lanes.",
        add_horizontal_m256(a: m256, b: m256) -> m256;
        "Add adjacent `f64` lanes.",
        add_horizontal_m256d(a: m256d, b: m256d) -> m256d;
        "Lanewise `a + b` with `f32` lanes.",
        add_m256(a: m256, b: m256) -> m256;
        "Lanewise `a + b` with `f64` lanes.",
        add_m256d(a: m256d, b: m256d) -> m256d;
        "Alternately, from the top, add `f32` then sub `f32`.",
        addsub_m256(a: m256, b: m256) -> m256;
        "Alternately, from the top, add `f64` then sub `f64`.",
        addsub_m256d(a: m256d, b: m256d) -> m256d;
        "Bitwise `a &amp; b`.",
        bitand_m256(a: m256, b: m256) -> m256;
        "Bitwise `a &amp; b`.",
        bitand_m256d(a: m256d, b: m256d) -> m256d;
        "Bitwise `(!a) &amp; b`.",
        bitandnot_m256(a: m256, b: m256) -> m256;
        "Bitwise `(!a) &amp; b`.",
        bitandnot_m256d(a: m256d, b: m256d) -> m256d;
        "Bitwise `a | b`.",
        bitor_m256(a: m256, b: m256) -> m256;
        "Bitwise `a | b`.",
        bitor_m256d(a: m256d, b: m256d) -> m256d;
        "Bitwise `a ^ b`.",
        bitxor_m256(a: m256, b: m256) -> m256;
        "Bitwise `a ^ b`.",
        bitxor_m256d(a: m256d, b: m256d) -> m256d;
        "Blends the `f32` lanes according to the immediate mask.",
        blend_m256<const IMM: i32>(a: m256, b: m256) -> m256;
        "Blends the `f64` lanes according to the immediate mask.",
        blend_m256d<const IMM: i32>(a: m256d, b: m256d) -> m256d;
        "Blend the lanes according to a runtime varying mask.",
        blend_varying_m256(a: m256, b: m256, mask: m256) -> m256;
        "Blend the lanes according to a runtime varying mask.",
        blend_varying_m256d(a: m256d, b: m256d, mask: m256d) -> m256d;
        "Bit-preserving cast to `m128` from `m256`.",
        cast_to_m128_from_m256(a: m256) -> m128;
        "Bit-preserving cast to `m128d` from `m256d`.",
        cast_to_m128d_from_m256d(a: m256d) -> m128d;
        "Bit-preserving cast to `m128i` from `m256i`.",
        cast_to_m128i_from_m256i(a: m256i) -> m128i;
        "Bit-preserving cast to `m256` from `m256d`.",
        cast_to_m256_from_m256d(a: m256d) -> m256;
        "Bit-preserving cast to `m256` from `m256i`.",
        cast_to_m256_from_m256i(a: m256i) -> m256;
        "Bit-preserving cast to `m256i` from `m256`.",
        cast_to_m256d_from_m256(a: m256) -> m256d;
        "Bit-preserving cast to `m256d` from `m256i`.",
        cast_to_m256d_from_m256i(a: m256i) -> m256d;
        "Bit-preserving cast to `m256i` from `m256`.",
        cast_to_m256i_from_m256(a: m256) -> m256i;
        "Bit-preserving cast to `m256i` from `m256d`.",
        cast_to_m256i_from_m256d(a: m256d) -> m256i;
        "Round `f32` lanes towards positive infinity.",
        ceil_m256(a: m256) -> m256;
        "Round `f64` lanes towards positive infinity.",
        ceil_m256d(a: m256d) -> m256d;
        "Compare `f32` lanes according to the operation specified, mask output.",
        cmp_op_mask_m128<const OP: i32>(a: m128, b: m128) -> m128;
        "Compare `f32` lanes according to the operation specified, mask output.",
        cmp_op_mask_m128_s<const OP: i32>(a: m128, b: m128) -> m128;
        "Compare `f64` lanes according to the operation specified, mask output.",
        cmp_op_mask_m128d<const OP: i32>(a: m128d, b: m128d) -> m128d;
        "Compare `f64` lanes according to the operation specified, mask output.",
        cmp_op_mask_m128d_s<const OP: i32>(a: m128d, b: m128d) -> m128d;
        "Compare `f32` lanes according to the operation specified, mask output.",
        cmp_op_mask_m256<const OP: i32>(a: m256, b: m256) -> m256;
        "Compare `f64` lanes according to the operation specified, mask output.",
        cmp_op_mask_m256d<const OP: i32>(a: m256d, b: m256d) -> m256d;
        "Convert the lowest `f32` lane to a single `f32`.",
        convert_to_f32_from_m256_s(a: m256) -> f32;
        "Convert the lowest `f64` lane to a single `f64`.",
        convert_to_f64_from_m256d_s(a: m256d) -> f64;
        "Convert the lowest `i32` lane to a single `i32`.",
        convert_to_i32_from_m256i_s(a: m256i) -> i32;
        "Convert `f64` lanes to be `i32` lanes.",
        convert_to_i32_m128i_from_m256d(a: m256d) -> m128i;
        "Convert `f32` lanes to be `i32` lanes.",
        convert_to_i32_m256i_from_m256(a: m256) -> m256i;
        "Convert `f64` lanes to be `f32` lanes.",
        convert_to_m128_from_m256d(a: m256d) -> m128;
        "Convert `i32` lanes to be `f32` lanes.",
        convert_to_m256_from_i32_m256i(a: m256i) -> m256;
        "Convert `i32` lanes to be `f64` lanes.",
        convert_to_m256d_from_i32_m128i(a: m128i) -> m256d;
        "Convert `f32` lanes to be `f64` lanes.",
        convert_to_m256d_from_m128(a: m128) -> m256d;
        "Convert `f64` lanes to `i32` lanes with truncation.",
        convert_truncate_to_i32_m128i_from_m256d(a: m256d) -> m128i;
        "Convert `f32` lanes to `i32` lanes with truncation.",
        convert_truncate_to_i32_m256i_from_m256(a: m256) -> m256i;
        "Lanewise `a / b` with `f32`.",
        div_m256(a: m256, b: m256) -> m256;
        "Lanewise `a / b` with `f64`.",
        div_m256d(a: m256d, b: m256d) -> m256d;
        "This works like `dot_product_m128`, but twice as wide.",
        dot_product_m256<const IMM: i32>(a: m256, b: m256) -> m256;
        "Duplicate the even-indexed lanes to the odd lanes.",
        duplicate_even_lanes_m256(a: m256) -> m256;
        "Duplicate the odd-indexed lanes to the even lanes.",
        duplicate_odd_lanes_m256(a: m256) -> m256;
        "Duplicate the odd-indexed lanes to the even lanes.",
        duplicate_odd_lanes_m256d(a: m256d) -> m256d;
        "Extracts an `i32` lane from `m256i`",
        extract_i32_from_m256i<const IMM: i32>(a: m256i) -> i32;
        "Extracts an `i64` lane from `m256i`",
        extract_i64_from_m256i<const IMM: i32>(a: m256i) -> i64;
        "Extracts an `m128` from `m256`",
        extract_m128_from_m256<const IMM: i32>(a: m256) -> m128;
        "Extracts an `m128d` from `m256d`",
        extract_m128d_from_m256d<const IMM: i32>(a: m256d) -> m128d;
        "Extracts an `m128i` from `m256i`",
        extract_m128i_from_m256i<const IMM: i32>(a: m256i) -> m128i;
        "Round `f32` lanes towards negative infinity.",
        floor_m256(a: m256) -> m256;
        "Round `f64` lanes towards negative infinity.",
        floor_m256d(a: m256d) -> m256d;
        "Inserts an `i16` to `m256i`",
        insert_i16_to_m256i<const IMM: i32>(a: m256i, i: i16) -> m256i;
        "Inserts an `i32` to `m256i`",
        insert_i32_to_m256i<const IMM: i32>(a: m256i, i: i32) -> m256i;
        "Inserts an `i64` to `m256i`",
        insert_i64_to_m256i<const IMM: i32>(a: m256i, i: i64) -> m256i;
        "Inserts an `i8` to `m256i`",
        insert_i8_to_m256i<const IMM: i32>(a: m256i, i: i8) -> m256i;
        "Inserts an `m128` to `m256`",
        insert_m128_to_m256<const IMM: i32>(a: m256, b: m128) -> m256;
        "Inserts an `m128d` to `m256d`",
        insert_m128d_to_m256d<const IMM: i32>(a: m256d, b: m128d) -> m256d;
        "Slowly inserts an `m128i` to `m256i`.",
        insert_m128i_to_m256i_slow_avx<const IMM: i32>(a: m256i, b: m128i) -> m256i;
        "Load an `f32` and splat it to all lanes of an `m256d`",
        load_f32_splat_m256(a: &f32) -> m256;
        "Load an `f64` and splat it to all lanes of an `m256d`",
        load_f64_splat_m256d(a: &f64) -> m256d;
        "Load an `m128` and splat it to the lower and upper half of an `m256`",
        load_m128_splat_m256(a: &m128) -> m256;
        "Load an `m128d` and splat it to the lower and upper half of an `m256d`",
        load_m128d_splat_m256d(a: &m128d) -> m256d;
        "Load data from memory into a register.",
        load_m256(a: &m256) -> m256;
        "Load data from memory into a register.",
        load_m256d(a: &m256d) -> m256d;
        "Load data from memory into a register.",
        load_m256i(a: &m256i) -> m256i;
        "Load data from memory into a register according to a mask.",
        load_masked_m128(a: &m128, mask: m128i) -> m128;
        "Load data from memory into a register according to a mask.",
        load_masked_m128d(a: &m128d, mask: m128i) -> m128d;
        "Load data from memory into a register according to a mask.",
        load_masked_m256(a: &m256, mask: m256i) -> m256;
        "Load data from memory into a register according to a mask.",
        load_masked_m256d(a: &m256d, mask: m256i) -> m256d;
        "Load data from memory into a register.",
        load_unaligned_hi_lo_m256(a: &[f32; 4], b: &[f32; 4]) -> m256;
        "Load data from memory into a register.",
        load_unaligned_hi_lo_m256d(a: &[f64; 2], b: &[f64; 2]) -> m256d;
        "Load data from memory into a register.",
        load_unaligned_hi_lo_m256i(a: &[i8; 16], b: &[i8; 16]) -> m256i;
        "Load data from memory into a register.",
        load_unaligned_m256(a: &[f32; 8]) -> m256;
        "Load data from memory into a register.",
        load_unaligned_m256d(a: &[f64; 4]) -> m256d;
        "Load data from memory into a register.",
        load_unaligned_m256i(a: &[i8; 32]) -> m256i;
        "Lanewise `max(a, b)`.",
        max_m256(a: m256, b: m256) -> m256;
        "Lanewise `max(a, b)`.",
        max_m256d(a: m256d, b: m256d) -> m256d;
        "Lanewise `min(a, b)`.",
        min_m256(a: m256, b: m256) -> m256;
        "Lanewise `min(a, b)`.",
        min_m256d(a: m256d, b: m256d) -> m256d;
        "Collects the sign bit of each lane into a 4-bit value.",
        move_mask_m256(a: m256) -> i32;
        "Collects the sign bit of each lane into a 4-bit value.",
        move_mask_m256d(a: m256d) -> i32;
        "Lanewise `a * b` with `f32` lanes.",
        mul_m256(a: m256, b: m256) -> m256;
        "Lanewise `a * b` with `f64` lanes.",
        mul_m256d(a: m256d, b: m256d) -> m256d;
        "Shuffle 128 bits of floating point data at a time from `$a` and `$b` using
        an immediate control value.",
        permute2z_m256<const MASK: i32>(a: m256, b: m256) -> m256;
        "Shuffle 128 bits of floating point data at a time from `a` and `b` using an
        immediate control value.",
        permute2z_m256d<const MASK: i32>(a: m256d, b: m256d) -> m256d;
        "<em>Slowly</em> swizzle 128 bits of integer data from `a` and `b` using an
        immediate control value.",
        permute2z_m256i<const MASK: i32>(a: m256i, b: m256i) -> m256i;
        "Shuffle the `f32` lanes from `a` using an immediate control value.",
        permute_m128<const MASK: i32>(a: m128) -> m128;
        "Shuffle the `f64` lanes in `a` using an immediate control value.",
        permute_m128d<const MASK: i32>(a: m128d) -> m128d;
        "Shuffle the `f32` lanes in `a` using an immediate control value.",
        permute_m256<const MASK: i32>(a: m256) -> m256;
        "Shuffle the `f64` lanes from `a` together using an immediate control value.",
        permute_m256d<const MASK: i32>(a: m256d) -> m256d;
        "Reciprocal of `f32` lanes.",
        reciprocal_m256(a: m256) -> m256;
        "Reciprocal of `f32` lanes.",
        reciprocal_sqrt_m256(a: m256) -> m256;
        "Rounds each lane in the style specified.",
        round_m256<const OP: i32>(a: m256) -> m256;
        "Rounds each lane in the style specified.",
        round_m256d<const OP: i32>(a: m256d) -> m256d;
        "Set `i16` args into an `m256i` lane.",
        set_i16_m256i(e15: i16, e14: i16, e13: i16, e12: i16, e11: i16, e10: i16, e9: i16, e8: i16,
            e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16) -> m256i;
        "Set `i32` args into an `m256i` lane.",
        set_i32_m256i(e7: i32, e6: i32, e5: i32, e4: i32, e3: i32, e2: i32, e1: i32,
            e0: i32) -> m256i;
        "Set `i64` args into an `m256i` lane.",
        set_i64_m256i(e3: i64, e2: i64, e1: i64, e0: i64) -> m256i;
        "Set `i8` args into an `m256i` lane.",
        set_i8_m256i(e31: i8, e30: i8, e29: i8, e28: i8, e27: i8, e26: i8, e25: i8, e24: i8,
            e23: i8, e22: i8, e21: i8, e20: i8, e19: i8, e18: i8, e17: i8, e16: i8, e15: i8,
            e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8, e6: i8,
            e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8) -> m256i;
        "Set `m128` args into an `m256`.",
        set_m128_m256(high: m128, low: m128) -> m256;
        "Set `m128d` args into an `m256d`.",
        set_m128d_m256d(high: m128d, low: m128d) -> m256d;
        "Set `m128i` args into an `m256i`.",
        set_m128i_m256i(hi: m128i, lo: m128i) -> m256i;
        "Set `f32` args into an `m256` lane.",
        set_m256(e7: f32, e6: f32, e5: f32, e4: f32, e3: f32, e2: f32, e1: f32, e0: f32) -> m256;
        "Set `f64` args into an `m256d` lane.",
        set_m256d(e3: f64, e2: f64, e1: f64, e0: f64) -> m256d;
        "Set `i16` args into an `m256i` lane.",
        set_reversed_i16_m256i(e15: i16, e14: i16, e13: i16, e12: i16, e11: i16, e10: i16,
            e9: i16, e8: i16, e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16,
            e0: i16) -> m256i;
        "Set `i32` args into an `m256i` lane.",
        set_reversed_i32_m256i(e7: i32, e6: i32, e5: i32, e4: i32, e3: i32, e2: i32, e1: i32,
            e0: i32) -> m256i;
        "Set `i64` args into an `m256i` lane.",
        set_reversed_i64_m256i(e3: i64, e2: i64, e1: i64, e0: i64) -> m256i;
        "Set `i8` args into an `m256i` lane.",
        set_reversed_i8_m256i(e31: i8, e30: i8, e29: i8, e28: i8, e27: i8, e26: i8, e25: i8,
            e24: i8, e23: i8, e22: i8, e21: i8, e20: i8, e19: i8, e18: i8, e17: i8, e16: i8,
            e15: i8, e14: i8, e13: i8, e12: i8, e11: i8, e10: i8, e9: i8, e8: i8, e7: i8,
            e6: i8, e5: i8, e4: i8, e3: i8, e2: i8, e1: i8, e0: i8) -> m256i;
        "Set `m128` args into an `m256`.",
        set_reversed_m128_m256(hi: m128, lo: m128) -> m256;
        "Set `m128d` args into an `m256d`.",
        set_reversed_m128d_m256d(hi: m128d, lo: m128d) -> m256d;
        "Set `m128i` args into an `m256i`.",
        set_reversed_m128i_m256i(hi: m128i, lo: m128i) -> m256i;
        "Set `f32` args into an `m256` lane.",
        set_reversed_m256(e7: f32, e6: f32, e5: f32, e4: f32, e3: f32, e2: f32, e1: f32,
            e0: f32) -> m256;
        "Set `f64` args into an `m256d` lane.",
        set_reversed_m256d(e3: f64, e2: f64, e1: f64, e0: f64) -> m256d;
        "Splat an `i16` arg into an `m256i` lane.",
        set_splat_i16_m256i(i: i16) -> m256i;
        "Splat an `i32` arg into an `m256i` lane.",
        set_splat_i32_m256i(i: i32) -> m256i;
        "Splat an `i64` arg into an `m256i` lane.",
        set_splat_i64_m256i(i: i64) -> m256i;
        "Splat an `i8` arg into an `m256i` lane.",
        set_splat_i8_m256i(i: i8) -> m256i;
        "Splat an `f32` arg into an `m256` lane.",
        set_splat_m256(f: f32) -> m256;
        "Splat an `f64` arg into an `m256d` lane.",
        set_splat_m256d(f: f64) -> m256d;
        "Shuffle `f32` values in `a` using `i32` values in `v`.",
        shuffle_av_f32_all_m128(a: m128, v: m128i) -> m128;
        "Shuffle `f32` values in `a` using `i32` values in `v`.",
        shuffle_av_f32_half_m256(a: m256, v: m256i) -> m256;
        "Shuffle `f64` lanes in `a` using <strong>bit 1</strong> of the `i64` lanes in `v`",
        shuffle_av_f64_all_m128d(a: m128d, v: m128i) -> m128d;
        "Shuffle `f64` lanes in `a` using <strong>bit 1</strong> of the `i64` lanes in `v`.",
        shuffle_av_f64_half_m256d(a: m256d, b: m256i) -> m256d;
        "Shuffle the `f32` lanes from `a` and `b` together using an immediate control value.",
        shuffle_m256<const IMM: i32>(a: m256, b: m256) -> m256;
        "Shuffle the `f64` lanes from `a` and `b` together using an immediate control value.",
        shuffle_m256d<const IMM: i32>(a: m256d, b: m256d) -> m256d;
        "Lanewise `sqrt` on `f64` lanes.",
        sqrt_m256(a: m256) -> m256;
        "Lanewise `sqrt` on `f64` lanes.",
        sqrt_m256d(a: m256d) -> m256d;
        "Store data from a register into memory.",
        store_m256(addr: &mut m256, a: m256);
        "Store data from a register into memory.",
        store_m256d(addr: &mut m256d, a: m256d);
        "Store data from a register into memory.",
        store_m256i(addr: &mut m256i, a: m256i);
        "Store data from a register into memory according to a mask.",
        store_masked_m128(addr: &mut m128, mask: m128i, a: m128);
        "Store data from a register into memory according to a mask.",
        store_masked_m128d(addr: &mut m128d, mask: m128i, a: m128d);
        "Store data from a register into memory according to a mask.",
        store_masked_m256(addr: &mut m256, mask: m256i, a: m256);
        "Store data from a register into memory according to a mask.",
        store_masked_m256d(addr: &mut m256d, mask: m256i, a: m256d);
        "Store data from a register into memory.",
        store_unaligned_hi_lo_m256(hi_addr: &mut [f32; 4], lo_addr: &mut [f32; 4], a: m256);
        "Store data from a register into memory.",
        store_unaligned_hi_lo_m256d(hi_addr: &mut [f64; 2], lo_addr: &mut [f64; 2], a: m256d);
        "Store data from a register into memory.",
        store_unaligned_hi_lo_m256i(hi_addr: &mut [i8; 16], lo_addr: &mut [i8; 16], a: m256i);
        "Store data from a register into memory.",
        store_unaligned_m256(addr: &mut [f32; 8], a: m256);
        "Store data from a register into memory.",
        store_unaligned_m256d(addr: &mut [f64; 4], a: m256d);
        "Store data from a register into memory.",
        store_unaligned_m256i(addr: &mut [i8; 32], a: m256i);
        "Subtract adjacent `f32` lanes.",
        sub_horizontal_m256(a: m256, b: m256) -> m256;
        "Subtract adjacent `f64` lanes.",
        sub_horizontal_m256d(a: m256d, b: m256d) -> m256d;
        "Lanewise `a - b` with `f32` lanes.",
        sub_m256(a: m256, b: m256) -> m256;
        "Lanewise `a - b` with `f64` lanes.",
        sub_m256d(a: m256d, b: m256d) -> m256d;
        "Compute the bitwise of sign bit NOT of `a` and then AND with `b`,
        returns 1 if the result is zero, otherwise 0.",
        testc_m128(a: m128, b: m128) -> i32;
        "Compute the bitwise of sign bit NOT of `a` and then AND with `b`,
        returns 1 if the result is zero, otherwise 0.",
        testc_m128d(a: m128d, b: m128d) -> i32;
        "Compute the bitwise of sign bit NOT of `a` and then AND with `b`,
        returns 1 if the result is zero, otherwise 0.",
        testc_m256(a: m256, b: m256) -> i32;
        "Compute the bitwise of sign bit NOT of `a` and then AND with `b`,
        returns 1 if the result is zero, otherwise 0.",
        testc_m256d(a: m256d, b: m256d) -> i32;
        "Compute the bitwise NOT of `a` and then AND with `b`,
        returns 1 if the result is zero, otherwise 0.",
        testc_m256i(a: m256i, b: m256i) -> i32;
        "Computes the bitwise AND of 256 bits in `a` and
        `b`, returns 1 if the result is zero, otherwise 0.",
        testz_m128(a: m128, b: m128) -> i32;
        "Computes the bitwise of sign bitAND of 256 bits in `a` and
        `b`, returns 1 if the result is zero, otherwise 0.",
        testz_m128d(a: m128d, b: m128d) -> i32;
        "Computes the bitwise AND of 256 bits in `a` and
        `b`, returns 1 if the result is zero, otherwise 0.",
        testz_m256(a: m256, b: m256) -> i32;
        "Computes the bitwise of sign bit AND of 256 bits in `a` and
        `b`, returns 1 if the result is zero, otherwise 0.",
        testz_m256d(a: m256d, b: m256d) -> i32;
        "Computes the bitwise of sign bit AND of 256 bits in `a` and
        `b`, returns 1 if the result is zero, otherwise 0.",
        testz_m256i(a: m256i, b: m256i) -> i32;
        "Unpack and interleave the high lanes.",
        unpack_hi_m256(a: m256, b: m256) -> m256;
        "Unpack and interleave the high lanes.",
        unpack_hi_m256d(a: m256d, b: m256d) -> m256d;
        "Unpack and interleave the high lanes.",
        unpack_lo_m256(a: m256, b: m256) -> m256;
        "Unpack and interleave the high lanes.",
        unpack_lo_m256d(a: m256d, b: m256d) -> m256d;
        "Zero extend an `m128` to `m256`",
        zero_extend_m128(a: m128) -> m256;
        "Zero extend an `m128d` to `m256d`",
        zero_extend_m128d(a: m128d) -> m256d;
        "Zero extend an `m128i` to `m256i`",
        zero_extend_m128i(a: m128i) -> m256i;
        "A zeroed `m256`",
        zeroed_m256() -> m256;
        "A zeroed `m256d`",
        zeroed_m256d() -> m256d;
        "A zeroed `m256i`",
        zeroed_m256i() -> m256i;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `avx2` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/Advanced_Vector_Extensions>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "avx2";
    arch_fn! {
        "Absolute value of `i16` lanes.",
        abs_i16_m256i(a: m256i) -> m256i;
        "Absolute value of `i32` lanes.",
        abs_i32_m256i(a: m256i) -> m256i;
        "Absolute value of `i8` lanes.",
        abs_i8_m256i(a: m256i) -> m256i;
        "Horizontal `a + b` with lanes as `i16`.",
        add_horizontal_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Horizontal `a + b` with lanes as `i32`.",
        add_horizontal_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Horizontal saturating `a + b` with lanes as `i16`.",
        add_horizontal_saturating_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a + b` with lanes as `i16`.",
        add_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a + b` with lanes as `i32`.",
        add_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a + b` with lanes as `i64`.",
        add_i64_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a + b` with lanes as `i8`.",
        add_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise saturating `a + b` with lanes as `i16`.",
        add_saturating_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise saturating `a + b` with lanes as `i8`.",
        add_saturating_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise saturating `a + b` with lanes as `u16`.",
        add_saturating_u16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise saturating `a + b` with lanes as `u8`.",
        add_saturating_u8_m256i(a: m256i, b: m256i) -> m256i;
        "Average `u16` lanes.",
        average_u16_m256i(a: m256i, b: m256i) -> m256i;
        "Average `u8` lanes.",
        average_u8_m256i(a: m256i, b: m256i) -> m256i;
        "Bitwise `a &amp; b`.",
        bitand_m256i(a: m256i, b: m256i) -> m256i;
        "Bitwise `(!a) &amp; b`.",
        bitandnot_m256i(a: m256i, b: m256i) -> m256i;
        "Bitwise `a | b`",
        bitor_m256i(a: m256i, b: m256i) -> m256i;
        "Bitwise `a ^ b`.",
        bitxor_m256i(a: m256i, b: m256i) -> m256i;
        "Blends the `i16` lanes according to the immediate value.",
        blend_imm_i16_m256i<const IMM: i32>(a: m256i, b: m256i) -> m256i;
        "Blends the `i32` lanes in `a` and `b` into a single value.",
        blend_imm_i32_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i;
        "Blends the `i32` lanes according to the immediate value.",
        blend_imm_i32_m256i<const IMM: i32>(a: m256i, b: m256i) -> m256i;
        "Blend `i8` lanes according to a runtime varying mask.",
        blend_varying_i8_m256i(a: m256i, b: m256i, mask: m256i) -> m256i;
        "Shifts each `u128` lane left by a number of <strong>bytes</strong>.",
        byte_shl_imm_u128_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shifts each `u128` lane right by a number of <strong>bytes</strong>.",
        byte_shr_imm_u128_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Compare `i16` lanes for equality, mask output.",
        cmp_eq_mask_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Compare `i32` lanes for equality, mask output.",
        cmp_eq_mask_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Compare `i64` lanes for equality, mask output.",
        cmp_eq_mask_i64_m256i(a: m256i, b: m256i) -> m256i;
        "Compare `i8` lanes for equality, mask output.",
        cmp_eq_mask_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Compare `i16` lanes for `a &gt; b`, mask output.",
        cmp_gt_mask_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Compare `i32` lanes for `a &gt; b`, mask output.",
        cmp_gt_mask_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Compare `i64` lanes for `a &gt; b`, mask output.",
        cmp_gt_mask_i64_m256i(a: m256i, b: m256i) -> m256i;
        "Compare `i8` lanes for `a &gt; b`, mask output.",
        cmp_gt_mask_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Works like `combined_byte_shr_imm_m128i`, but twice as wide.",
        combined_byte_shr_imm_m256i<const IMM: i32>(a: m256i, b: m256i) -> m256i;
        "Convert `i8` values to `i16` values.",
        convert_to_i16_m256i_from_i8_m128i(a: m128i) -> m256i;
        "Convert lower 4 `u8` values to `i16` values.",
        convert_to_i16_m256i_from_lower4_u8_m128i(a: m128i) -> m256i;
        "Convert lower 8 `u8` values to `i16` values.",
        convert_to_i16_m256i_from_lower8_u8_m128i(a: m128i) -> m256i;
        "Convert `u8` values to `i16` values.",
        convert_to_i16_m256i_from_u8_m128i(a: m128i) -> m256i;
        "Convert `i16` values to `i32` values.",
        convert_to_i32_m256i_from_i16_m128i(a: m128i) -> m256i;
        "Convert the lower 8 `i8` values to `i32` values.",
        convert_to_i32_m256i_from_lower8_i8_m128i(a: m128i) -> m256i;
        "Convert `u16` values to `i32` values.",
        convert_to_i32_m256i_from_u16_m128i(a: m128i) -> m256i;
        "Convert `i32` values to `i64` values.",
        convert_to_i64_m256i_from_i32_m128i(a: m128i) -> m256i;
        "Convert `i16` values to `i64` values.",
        convert_to_i64_m256i_from_lower4_i16_m128i(a: m128i) -> m256i;
        "Convert the lower 4 `i8` values to `i64` values.",
        convert_to_i64_m256i_from_lower4_i8_m128i(a: m128i) -> m256i;
        "Convert `u16` values to `i64` values.",
        convert_to_i64_m256i_from_lower4_u16_m128i(a: m128i) -> m256i;
        "Convert `u32` values to `i64` values.",
        convert_to_i64_m256i_from_u32_m128i(a: m128i) -> m256i;
        "Gets an `i16` value out of an `m256i`, returns as `i32`.",
        extract_i16_as_i32_m256i<const LANE: i32>(a: m256i) -> i32;
        "Gets an `i8` value out of an `m256i`, returns as `i32`.",
        extract_i8_as_i32_m256i<const LANE: i32>(a: m256i) -> i32;
        "Gets an `m128i` value out of an `m256i`.",
        extract_m128i_m256i<const LANE: i32>(a: m256i) -> m128i;
        "Inserts an `m128i` to an `m256i` at the high or low position.",
        insert_m128i_to_m256i<const LANE: i32>(a: m256i, b: m128i) -> m256i;
        "Loads the reference given and zeroes any `i32` lanes not in the mask.",
        load_masked_i32_m128i(a: &m128i, mask: m128i) -> m128i;
        "Loads the reference given and zeroes any `i32` lanes not in the mask.",
        load_masked_i32_m256i(a: &m256i, mask: m256i) -> m256i;
        "Loads the reference given and zeroes any `i64` lanes not in the mask.",
        load_masked_i64_m128i(a: &m128i, mask: m128i) -> m128i;
        "Loads the reference given and zeroes any `i64` lanes not in the mask.",
        load_masked_i64_m256i(a: &m256i, mask: m256i) -> m256i;
        "Lanewise `max(a, b)` with lanes as `i16`.",
        max_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `max(a, b)` with lanes as `i32`.",
        max_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `max(a, b)` with lanes as `i8`.",
        max_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `max(a, b)` with lanes as `u16`.",
        max_u16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `max(a, b)` with lanes as `u32`.",
        max_u32_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `max(a, b)` with lanes as `u8`.",
        max_u8_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `min(a, b)` with lanes as `i16`.",
        min_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `min(a, b)` with lanes as `i32`.",
        min_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `min(a, b)` with lanes as `i8`.",
        min_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `min(a, b)` with lanes as `u16`.",
        min_u16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `min(a, b)` with lanes as `u32`.",
        min_u32_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `min(a, b)` with lanes as `u8`.",
        min_u8_m256i(a: m256i, b: m256i) -> m256i;
        "Create an `i32` mask of each sign bit in the `i8` lanes.",
        move_mask_i8_m256i(a: m256i) -> i32;
        "Multiply `i16` lanes producing `i32` values, horizontal add pairs of `i32`
        values to produce the final output.",
        mul_i16_horizontal_add_m256i(a: m256i, b: m256i) -> m256i;
        "Multiply the `i16` lanes and keep the high half of each 32-bit output.",
        mul_i16_keep_high_m256i(a: m256i, b: m256i) -> m256i;
        "Multiply the `i16` lanes and keep the low half of each 32-bit output.",
        mul_i16_keep_low_m256i(a: m256i, b: m256i) -> m256i;
        "Multiply `i16` lanes into `i32` intermediates, keep the high 18 bits, round
        by adding 1, right shift by 1.",
        mul_i16_scale_round_m256i(a: m256i, b: m256i) -> m256i;
        "Multiply the `i32` lanes and keep the low half of each 64-bit output.",
        mul_i32_keep_low_m256i(a: m256i, b: m256i) -> m256i;
        "Multiply the lower `i32` within each `i64` lane, `i64` output.",
        mul_i64_low_bits_m256i(a: m256i, b: m256i) -> m256i;
        "Multiply the `u16` lanes and keep the high half of each 32-bit output.",
        mul_u16_keep_high_m256i(a: m256i, b: m256i) -> m256i;
        "Multiply the lower `u32` within each `u64` lane, `u64` output.",
        mul_u64_low_bits_m256i(a: m256i, b: m256i) -> m256i;
        "This is dumb and weird.",
        mul_u8i8_add_horizontal_saturating_m256i(a: m256i, b: m256i) -> m256i;
        "Computes eight `u16` “sum of absolute difference” values according to the
        bytes selected.",
        multi_packed_sum_abs_diff_u8_m256i<const IMM: i32>(a: m256i, b: m256i) -> m256i;
        "Saturating convert `i16` to `i8`, and pack the values.",
        pack_i16_to_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Saturating convert `i16` to `u8`, and pack the values.",
        pack_i16_to_u8_m256i(a: m256i, b: m256i) -> m256i;
        "Saturating convert `i32` to `i16`, and pack the values.",
        pack_i32_to_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Saturating convert `i32` to `u16`, and pack the values.",
        pack_i32_to_u16_m256i(a: m256i, b: m256i) -> m256i;
        "Sets the lowest `i16` lane of an `m128i` as all lanes of an `m256i`.",
        set_splat_i16_m128i_s_m256i(a: m128i) -> m256i;
        "Sets the lowest `i32` lane of an `m128i` as all lanes of an `m256i`.",
        set_splat_i32_m128i_s_m256i(a: m128i) -> m256i;
        "Sets the lowest `i64` lane of an `m128i` as all lanes of an `m256i`.",
        set_splat_i64_m128i_s_m256i(a: m128i) -> m256i;
        "Sets the lowest `i8` lane of an `m128i` as all lanes of an `m256i`.",
        set_splat_i8_m128i_s_m256i(a: m128i) -> m256i;
        "Sets the lowest lane of an `m128` as all lanes of an `m256`.",
        set_splat_m128_s_m256(a: m128) -> m256;
        "Sets the lowest lane of an `m128d` as all lanes of an `m256d`.",
        set_splat_m128d_s_m256d(a: m128d) -> m256d;
        "Lanewise `u16` shift left by the lower `u64` lane of `count`.",
        shl_all_u16_m256i(a: m256i, count: m128i) -> m256i;
        "Shift all `u32` lanes left by the lower `u64` lane of `count`.",
        shl_all_u32_m256i(a: m256i, count: m128i) -> m256i;
        "Shift all `u64` lanes left by the lower `u64` lane of `count`.",
        shl_all_u64_m256i(a: m256i, count: m128i) -> m256i;
        "Shift `u32` values to the left by `count` bits.",
        shl_each_u32_m128i(a: m128i, count: m128i) -> m128i;
        "Lanewise `u32` shift left by the matching `i32` lane in `count`.",
        shl_each_u32_m256i(a: m256i, count: m256i) -> m256i;
        "Shift `u64` values to the left by `count` bits.",
        shl_each_u64_m128i(a: m128i, count: m128i) -> m128i;
        "Lanewise `u64` shift left by the matching `u64` lane in `count`.",
        shl_each_u64_m256i(a: m256i, count: m256i) -> m256i;
        "Shifts all `u16` lanes left by an immediate.",
        shl_imm_u16_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shifts all `u32` lanes left by an immediate.",
        shl_imm_u32_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shifts all `u64` lanes left by an immediate.",
        shl_imm_u64_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Lanewise `i16` shift right by the lower `i64` lane of `count`.",
        shr_all_i16_m256i(a: m256i, count: m128i) -> m256i;
        "Lanewise `i32` shift right by the lower `i64` lane of `count`.",
        shr_all_i32_m256i(a: m256i, count: m128i) -> m256i;
        "Lanewise `u16` shift right by the lower `u64` lane of `count`.",
        shr_all_u16_m256i(a: m256i, count: m128i) -> m256i;
        "Lanewise `u32` shift right by the lower `u64` lane of `count`.",
        shr_all_u32_m256i(a: m256i, count: m128i) -> m256i;
        "Lanewise `u64` shift right by the lower `u64` lane of `count`.",
        shr_all_u64_m256i(a: m256i, count: m128i) -> m256i;
        "Shift `i32` values to the right by `count` bits.",
        shr_each_i32_m128i(a: m128i, count: m128i) -> m128i;
        "Lanewise `i32` shift right by the matching `i32` lane in `count`.",
        shr_each_i32_m256i(a: m256i, count: m256i) -> m256i;
        "Shift `u32` values to the left by `count` bits.",
        shr_each_u32_m128i(a: m128i, count: m128i) -> m128i;
        "Lanewise `u32` shift right by the matching `u32` lane in `count`.",
        shr_each_u32_m256i(a: m256i, count: m256i) -> m256i;
        "Shift `u64` values to the left by `count` bits.",
        shr_each_u64_m128i(a: m128i, count: m128i) -> m128i;
        "Lanewise `u64` shift right by the matching `i64` lane in `count`.",
        shr_each_u64_m256i(a: m256i, count: m256i) -> m256i;
        "Shifts all `i16` lanes left by an immediate.",
        shr_imm_i16_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shifts all `i32` lanes left by an immediate.",
        shr_imm_i32_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shifts all `u16` lanes right by an immediate.",
        shr_imm_u16_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shifts all `u32` lanes right by an immediate.",
        shr_imm_u32_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shifts all `u64` lanes right by an immediate.",
        shr_imm_u64_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shuffle 128 bits of integer data from `$a` and `$b` using an immediate control value.",
        shuffle_abi_i128z_all_m256i<const MASK: i32>(a: m256i, b: m256i) -> m256i;
        "Shuffle the `f64` lanes from `$a` using an immediate control value.",
        shuffle_ai_f64_all_m256d<const IMM: i32>(a: m256d) -> m256d;
        "Shuffle the high `i16` lanes in `$a` using an immediate control value.",
        shuffle_ai_i16_h64half_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shuffle the low `i16` lanes in `$a` using an immediate control value.",
        shuffle_ai_i16_l64half_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shuffle the `i32` lanes in `a` using an immediate control value.",
        shuffle_ai_i32_half_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shuffle the `f64` lanes in `$a` using an immediate control value.",
        shuffle_ai_i64_all_m256i<const IMM: i32>(a: m256i) -> m256i;
        "Shuffle `f32` lanes in `a` using `i32` values in `v`.",
        shuffle_av_i32_all_m256(a: m256, v: m256i) -> m256;
        "Shuffle `i32` lanes in `a` using `i32` values in `v`.",
        shuffle_av_i32_all_m256i(a: m256i, v: m256i) -> m256i;
        "Shuffle `i8` lanes in `a` using `i8` values in `v`.",
        shuffle_av_i8z_half_m256i(a: m256i, v: m256i) -> m256i;
        "Lanewise `a * signum(b)` with lanes as `i16`",
        sign_apply_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a * signum(b)` with lanes as `i32`",
        sign_apply_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a * signum(b)` with lanes as `i8`",
        sign_apply_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Splat the lowest 16-bit lane across the entire 128 bits.",
        splat_i16_m128i_s_m128i(a: m128i) -> m128i;
        "Splat the lowest 32-bit lane across the entire 128 bits.",
        splat_i32_m128i_s_m128i(a: m128i) -> m128i;
        "Splat the lowest 64-bit lane across the entire 128 bits.",
        splat_i64_m128i_s_m128i(a: m128i) -> m128i;
        "Splat the lowest 8-bit lane across the entire 128 bits.",
        splat_i8_m128i_s_m128i(a: m128i) -> m128i;
        "Splat the lowest `f32` across all four lanes.",
        splat_m128_s_m128(a: m128) -> m128;
        "Splat the lower `f64` across both lanes of `m128d`.",
        splat_m128d_s_m128d(a: m128d) -> m128d;
        "Splat the 128-bits across 256-bits.",
        splat_m128i_m256i(a: m128i) -> m256i;
        "Stores the `i32` masked lanes given to the reference.",
        store_masked_i32_m128i(addr: &mut m128i, mask: m128i, a: m128i);
        "Stores the `i32` masked lanes given to the reference.",
        store_masked_i32_m256i(addr: &mut m256i, mask: m256i, a: m256i);
        "Stores the `i32` masked lanes given to the reference.",
        store_masked_i64_m128i(addr: &mut m128i, mask: m128i, a: m128i);
        "Stores the `i32` masked lanes given to the reference.",
        store_masked_i64_m256i(addr: &mut m256i, mask: m256i, a: m256i);
        "Horizontal `a - b` with lanes as `i16`.",
        sub_horizontal_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Horizontal `a - b` with lanes as `i32`.",
        sub_horizontal_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Horizontal saturating `a - b` with lanes as `i16`.",
        sub_horizontal_saturating_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a - b` with lanes as `i16`.",
        sub_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a - b` with lanes as `i32`.",
        sub_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a - b` with lanes as `i64`.",
        sub_i64_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise `a - b` with lanes as `i8`.",
        sub_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise saturating `a - b` with lanes as `i16`.",
        sub_saturating_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise saturating `a - b` with lanes as `i8`.",
        sub_saturating_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise saturating `a - b` with lanes as `u16`.",
        sub_saturating_u16_m256i(a: m256i, b: m256i) -> m256i;
        "Lanewise saturating `a - b` with lanes as `u8`.",
        sub_saturating_u8_m256i(a: m256i, b: m256i) -> m256i;
        "Compute “sum of `u8` absolute differences”.",
        sum_of_u8_abs_diff_m256i(a: m256i, b: m256i) -> m256i;
        "Unpack and interleave high `i16` lanes of `a` and `b`.",
        unpack_high_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Unpack and interleave high `i32` lanes of `a` and `b`.",
        unpack_high_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Unpack and interleave high `i64` lanes of `a` and `b`.",
        unpack_high_i64_m256i(a: m256i, b: m256i) -> m256i;
        "Unpack and interleave high `i8` lanes of `a` and `b`.",
        unpack_high_i8_m256i(a: m256i, b: m256i) -> m256i;
        "Unpack and interleave low `i16` lanes of `a` and `b`.",
        unpack_low_i16_m256i(a: m256i, b: m256i) -> m256i;
        "Unpack and interleave low `i32` lanes of `a` and `b`.",
        unpack_low_i32_m256i(a: m256i, b: m256i) -> m256i;
        "Unpack and interleave low `i64` lanes of `a` and `b`.",
        unpack_low_i64_m256i(a: m256i, b: m256i) -> m256i;
        "Unpack and interleave low `i8` lanes of `a` and `b`.",
        unpack_low_i8_m256i(a: m256i, b: m256i) -> m256i;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `bmi1` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set#BMI1_(Bit_Manipulation_Instruction_Set_1)>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "bmi1";
    arch_fn! {
        "Extract a span of bits from the `u32`, control value style.",
        bit_extract2_u32(a: u32, control: u32) -> u32;
        "Extract a span of bits from the `u64`, control value style.",
        bit_extract2_u64(a: u64, control: u64) -> u64;
        "Extract a span of bits from the `u32`, start and len style.",
        bit_extract_u32(a: u32, start: u32, len: u32) -> u32;
        "Extract a span of bits from the `u64`, start and len style.",
        bit_extract_u64(a: u64, start: u32, len: u32) -> u64;
        "Gets the mask of all bits up to and including the lowest set bit in a `u32`.",
        bit_lowest_set_mask_u32(a: u32) -> u32;
        "Gets the mask of all bits up to and including the lowest set bit in a `u64`.",
        bit_lowest_set_mask_u64(a: u64) -> u64;
        "Resets (clears) the lowest set bit.",
        bit_lowest_set_reset_u32(a: u32) -> u32;
        "Resets (clears) the lowest set bit.",
        bit_lowest_set_reset_u64(a: u64) -> u64;
        "Gets the <em>value</em> of the lowest set bit in a `u32`.",
        bit_lowest_set_value_u32(a: u32) -> u32;
        "Gets the <em>value</em> of the lowest set bit in a `u64`.",
        bit_lowest_set_value_u64(a: u64) -> u64;
        "Bitwise `(!a) &amp; b` for `u32`",
        bitandnot_u32(a: u32, b: u32) -> u32;
        "Bitwise `(!a) &amp; b` for `u64`",
        bitandnot_u64(a: u64, b: u64) -> u64;
        "Counts the number of trailing zero bits in a `u32`.",
        trailing_zero_count_u32(a: u32) -> u32;
        "Counts the number of trailing zero bits in a `u64`.",
        trailing_zero_count_u64(a: u64) -> u64;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `bmi2` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set#BMI2_(Bit_Manipulation_Instruction_Set_2)>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "bmi2";
    arch_fn! {
        "Zero out all high bits in a `u32` starting at the index given.",
        bit_zero_high_index_u32(a: u32, index: u32) -> u32;
        "Zero out all high bits in a `u64` starting at the index given.",
        bit_zero_high_index_u64(a: u64, index: u32) -> u64;
        "Multiply two `u32`, outputting the low bits and storing the high bits in the reference.",
        mul_extended_u32(a: u32, b: u32, extra: &mut u32) -> u32;
        "Multiply two `u64`, outputting the low bits and storing the high bits in the reference.",
        mul_extended_u64(a: u64, b: u64, extra: &mut u64) -> u64;
        "Deposit contiguous low bits from a `u32` according to a mask.",
        population_deposit_u32(a: u32, index: u32) -> u32;
        "Deposit contiguous low bits from a `u64` according to a mask.",
        population_deposit_u64(a: u64, index: u64) -> u64;
        "Extract bits from a `u32` according to a mask.",
        population_extract_u32(a: u32, index: u32) -> u32;
        "Extract bits from a `u64` according to a mask.",
        population_extract_u64(a: u64, index: u64) -> u64;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `fma` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/FMA_instruction_set>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "fma";
    arch_fn! {
        "Lanewise fused `(a * b) + c`",
        fused_mul_add_m128(a: m128, b: m128, c: m128) -> m128;
        "Low lane fused `(a * b) + c`, other lanes unchanged",
        fused_mul_add_m128_s(a: m128, b: m128, c: m128) -> m128;
        "Lanewise fused `(a * b) + c`",
        fused_mul_add_m128d(a: m128d, b: m128d, c: m128d) -> m128d;
        "Low lane fused `(a * b) + c`, other lanes unchanged",
        fused_mul_add_m128d_s(a: m128d, b: m128d, c: m128d) -> m128d;
        "Lanewise fused `(a * b) + c`",
        fused_mul_add_m256(a: m256, b: m256, c: m256) -> m256;
        "Lanewise fused `(a * b) + c`",
        fused_mul_add_m256d(a: m256d, b: m256d, c: m256d) -> m256d;
        "Lanewise fused `(a * b) addsub c` (adds odd lanes and subtracts even lanes)",
        fused_mul_addsub_m128(a: m128, b: m128, c: m128) -> m128;
        "Lanewise fused `(a * b) addsub c` (adds odd lanes and subtracts even lanes)",
        fused_mul_addsub_m128d(a: m128d, b: m128d, c: m128d) -> m128d;
        "Lanewise fused `(a * b) addsub c` (adds odd lanes and subtracts even lanes)",
        fused_mul_addsub_m256(a: m256, b: m256, c: m256) -> m256;
        "Lanewise fused `(a * b) addsub c` (adds odd lanes and subtracts even lanes)",
        fused_mul_addsub_m256d(a: m256d, b: m256d, c: m256d) -> m256d;
        "Lanewise fused `-(a * b) + c`",
        fused_mul_neg_add_m128(a: m128, b: m128, c: m128) -> m128;
        "Low lane `-(a * b) + c`, other lanes unchanged.",
        fused_mul_neg_add_m128_s(a: m128, b: m128, c: m128) -> m128;
        "Lanewise fused `-(a * b) + c`",
        fused_mul_neg_add_m128d(a: m128d, b: m128d, c: m128d) -> m128d;
        "Low lane `-(a * b) + c`, other lanes unchanged.",
        fused_mul_neg_add_m128d_s(a: m128d, b: m128d, c: m128d) -> m128d;
        "Lanewise fused `-(a * b) + c`",
        fused_mul_neg_add_m256(a: m256, b: m256, c: m256) -> m256;
        "Lanewise fused `-(a * b) + c`",
        fused_mul_neg_add_m256d(a: m256d, b: m256d, c: m256d) -> m256d;
        "Lanewise fused `-(a * b) - c`",
        fused_mul_neg_sub_m128(a: m128, b: m128, c: m128) -> m128;
        "Low lane fused `-(a * b) - c`, other lanes unchanged.",
        fused_mul_neg_sub_m128_s(a: m128, b: m128, c: m128) -> m128;
        "Lanewise fused `-(a * b) - c`",
        fused_mul_neg_sub_m128d(a: m128d, b: m128d, c: m128d) -> m128d;
        "Low lane fused `-(a * b) - c`, other lanes unchanged.",
        fused_mul_neg_sub_m128d_s(a: m128d, b: m128d, c: m128d) -> m128d;
        "Lanewise fused `-(a * b) - c`",
        fused_mul_neg_sub_m256(a: m256, b: m256, c: m256) -> m256;
        "Lanewise fused `-(a * b) - c`",
        fused_mul_neg_sub_m256d(a: m256d, b: m256d, c: m256d) -> m256d;
        "Lanewise fused `(a * b) - c`",
        fused_mul_sub_m128(a: m128, b: m128, c: m128) -> m128;
        "Low lane fused `(a * b) - c`, other lanes unchanged.",
        fused_mul_sub_m128_s(a: m128, b: m128, c: m128) -> m128;
        "Lanewise fused `(a * b) - c`",
        fused_mul_sub_m128d(a: m128d, b: m128d, c: m128d) -> m128d;
        "Low lane fused `(a * b) - c`, other lanes unchanged.",
        fused_mul_sub_m128d_s(a: m128d, b: m128d, c: m128d) -> m128d;
        "Lanewise fused `(a * b) - c`",
        fused_mul_sub_m256(a: m256, b: m256, c: m256) -> m256;
        "Lanewise fused `(a * b) - c`",
        fused_mul_sub_m256d(a: m256d, b: m256d, c: m256d) -> m256d;
        "Lanewise fused `(a * b) subadd c` (subtracts odd lanes and adds even lanes)",
        fused_mul_subadd_m128(a: m128, b: m128, c: m128) -> m128;
        "Lanewise fused `(a * b) subadd c` (subtracts odd lanes and adds even lanes)",
        fused_mul_subadd_m128d(a: m128d, b: m128d, c: m128d) -> m128d;
        "Lanewise fused `(a * b) subadd c` (subtracts odd lanes and adds even lanes)",
        fused_mul_subadd_m256(a: m256, b: m256, c: m256) -> m256;
        "Lanewise fused `(a * b) subadd c` (subtracts odd lanes and adds even lanes)",
        fused_mul_subadd_m256d(a: m256d, b: m256d, c: m256d) -> m256d;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `lzcnt` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set#ABM_(Advanced_Bit_Manipulation)>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "lzcnt";
    arch_fn! {
        "Count the leading zeroes in a `u32`.",
        leading_zero_count_u32(a: u32) -> u32;
        "Count the leading zeroes in a `u64`.",
        leading_zero_count_u64(a: u64) -> u64;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `pclmulqdq` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/CLMUL_instruction_set>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "pclmulqdq";
    arch_fn! {
        "Performs a “carryless” multiplication of two `i64` values.",
        mul_i64_carryless_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `popcnt` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/X86_Bit_manipulation_instruction_set#ABM_(Advanced_Bit_Manipulation)>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "popcnt";
    arch_fn! {
        "Count the number of bits set within an `i32`",
        population_count_i32(a: i32) -> i32;
        "Count the number of bits set within an `i64`",
        population_count_i64(a: i64) -> i32;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `rdrand` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/RDRAND>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "rdrand";
    arch_fn! {
        "Try to obtain a random `u16` from the hardware RNG.",
        rdrand_u16(out: &mut u16) -> i32;
        "Try to obtain a random `u32` from the hardware RNG.",
        rdrand_u32(out: &mut u32) -> i32;
        "Try to obtain a random `u64` from the hardware RNG.",
        rdrand_u64(out: &mut u64) -> i32;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `rdseed` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/RDRAND>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "rdseed";
    arch_fn! {
        "Try to obtain a random `u16` from the hardware RNG.",
        rdseed_u16(out: &mut u16) -> i32;
        "Try to obtain a random `u32` from the hardware RNG.",
        rdseed_u32(out: &mut u32) -> i32;
        "Try to obtain a random `u64` from the hardware RNG.",
        rdseed_u64(out: &mut u64) -> i32;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `sse` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/Streaming_SIMD_Extensions>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "sse";
    arch_fn! {
        "Lanewise `a + b`.",
        add_m128(a: m128, b: m128) -> m128;
        "Low lane `a + b`, other lanes unchanged.",
        add_m128_s(a: m128, b: m128) -> m128;
        "Bitwise `a &amp; b`.",
        bitand_m128(a: m128, b: m128) -> m128;
        "Bitwise `(!a) &amp; b`.",
        bitandnot_m128(a: m128, b: m128) -> m128;
        "Bitwise `a | b`.",
        bitor_m128(a: m128, b: m128) -> m128;
        "Bitwise `a ^ b`.",
        bitxor_m128(a: m128, b: m128) -> m128;
        "Low lane equality.",
        cmp_eq_i32_m128_s(a: m128, b: m128) -> i32;
        "Lanewise `a == b`.",
        cmp_eq_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `a == b`, other lanes unchanged.",
        cmp_eq_mask_m128_s(a: m128, b: m128) -> m128;
        "Low lane greater than or equal to.",
        cmp_ge_i32_m128_s(a: m128, b: m128) -> i32;
        "Lanewise `a &gt;= b`.",
        cmp_ge_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `a &gt;= b`, other lanes unchanged.",
        cmp_ge_mask_m128_s(a: m128, b: m128) -> m128;
        "Low lane greater than.",
        cmp_gt_i32_m128_s(a: m128, b: m128) -> i32;
        "Lanewise `a &gt; b`.",
        cmp_gt_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `a &gt; b`, other lanes unchanged.",
        cmp_gt_mask_m128_s(a: m128, b: m128) -> m128;
        "Low lane less than or equal to.",
        cmp_le_i32_m128_s(a: m128, b: m128) -> i32;
        "Lanewise `a &lt;= b`.",
        cmp_le_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `a &lt;= b`, other lanes unchanged.",
        cmp_le_mask_m128_s(a: m128, b: m128) -> m128;
        "Low lane less than.",
        cmp_lt_i32_m128_s(a: m128, b: m128) -> i32;
        "Lanewise `a &lt; b`.",
        cmp_lt_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `a &lt; b`, other lanes unchanged.",
        cmp_lt_mask_m128_s(a: m128, b: m128) -> m128;
        "Low lane not equal to.",
        cmp_neq_i32_m128_s(a: m128, b: m128) -> i32;
        "Lanewise `a != b`.",
        cmp_neq_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `a != b`, other lanes unchanged.",
        cmp_neq_mask_m128_s(a: m128, b: m128) -> m128;
        "Lanewise `!(a &gt;= b)`.",
        cmp_nge_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `!(a &gt;= b)`, other lanes unchanged.",
        cmp_nge_mask_m128_s(a: m128, b: m128) -> m128;
        "Lanewise `!(a &gt; b)`.",
        cmp_ngt_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `!(a &gt; b)`, other lanes unchanged.",
        cmp_ngt_mask_m128_s(a: m128, b: m128) -> m128;
        "Lanewise `!(a &lt;= b)`.",
        cmp_nle_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `!(a &lt;= b)`, other lanes unchanged.",
        cmp_nle_mask_m128_s(a: m128, b: m128) -> m128;
        "Lanewise `!(a &lt; b)`.",
        cmp_nlt_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `!(a &lt; b)`, other lanes unchanged.",
        cmp_nlt_mask_m128_s(a: m128, b: m128) -> m128;
        "Lanewise `(!a.is_nan()) &amp; (!b.is_nan())`.",
        cmp_ordered_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `(!a.is_nan()) &amp; (!b.is_nan())`, other lanes unchanged.",
        cmp_ordered_mask_m128_s(a: m128, b: m128) -> m128;
        "Lanewise `a.is_nan() | b.is_nan()`.",
        cmp_unord_mask_m128(a: m128, b: m128) -> m128;
        "Low lane `a.is_nan() | b.is_nan()`, other lanes unchanged.",
        cmp_unord_mask_m128_s(a: m128, b: m128) -> m128;
        "Convert `i32` to `f32` and replace the low lane of the input.",
        convert_i32_replace_m128_s(a: m128, i: i32) -> m128;
        "Lanewise `a / b`.",
        div_m128(a: m128, b: m128) -> m128;
        "Low lane `a / b`, other lanes unchanged.",
        div_m128_s(a: m128, b: m128) -> m128;
        "Gets the low lane as an individual `f32` value.",
        get_f32_from_m128_s(a: m128) -> f32;
        "Converts the low lane to `i32` and extracts as an individual value.",
        get_i32_from_m128_s(a: m128) -> i32;
        "Loads the `f32` reference into the low lane of the register.",
        load_f32_m128_s(a: &f32) -> m128;
        "Loads the `f32` reference into all lanes of a register.",
        load_f32_splat_m128(a: &f32) -> m128;
        "Loads the reference into a register.",
        load_m128(a: &m128) -> m128;
        "Loads the reference into a register with reversed order.",
        load_reverse_m128(a: &m128) -> m128;
        "Loads the reference into a register.",
        load_unaligned_m128(a: &[f32; 4]) -> m128;
        "Lanewise `max(a, b)`.",
        max_m128(a: m128, b: m128) -> m128;
        "Low lane `max(a, b)`, other lanes unchanged.",
        max_m128_s(a: m128, b: m128) -> m128;
        "Lanewise `min(a, b)`.",
        min_m128(a: m128, b: m128) -> m128;
        "Low lane `min(a, b)`, other lanes unchanged.",
        min_m128_s(a: m128, b: m128) -> m128;
        "Move the high lanes of `b` to the low lanes of `a`, other lanes unchanged.",
        move_high_low_m128(a: m128, b: m128) -> m128;
        "Move the low lanes of `b` to the high lanes of `a`, other lanes unchanged.",
        move_low_high_m128(a: m128, b: m128) -> m128;
        "Move the low lane of `b` to `a`, other lanes unchanged.",
        move_m128_s(a: m128, b: m128) -> m128;
        "Gathers the sign bit of each lane.",
        move_mask_m128(a: m128) -> i32;
        "Lanewise `a * b`.",
        mul_m128(a: m128, b: m128) -> m128;
        "Low lane `a * b`, other lanes unchanged.",
        mul_m128_s(a: m128, b: m128) -> m128;
        "Lanewise `1.0 / a` approximation.",
        reciprocal_m128(a: m128) -> m128;
        "Low lane `1.0 / a` approximation, other lanes unchanged.",
        reciprocal_m128_s(a: m128) -> m128;
        "Lanewise `1.0 / sqrt(a)` approximation.",
        reciprocal_sqrt_m128(a: m128) -> m128;
        "Low lane `1.0 / sqrt(a)` approximation, other lanes unchanged.",
        reciprocal_sqrt_m128_s(a: m128) -> m128;
        "Sets the args into an `m128`, first arg is the high lane.",
        set_m128(three: f32, two: f32, one: f32, zero: f32) -> m128;
        "Sets the args into an `m128`, first arg is the high lane.",
        set_m128_s(low: f32) -> m128;
        "Sets the args into an `m128`, first arg is the low lane.",
        set_reversed_m128(zero: f32, one: f32, two: f32, three: f32) -> m128;
        "Splats the value to all lanes.",
        set_splat_m128(all: f32) -> m128;
        "Shuffle the `f32` lanes from `$a` and `$b` together using an immediate control value.",
        shuffle_abi_f32_all_m128<const MASK: i32>(a: m128, b: m128) -> m128;
        "Lanewise `sqrt(a)`.",
        sqrt_m128(a: m128) -> m128;
        "Low lane `sqrt(a)`, other lanes unchanged.",
        sqrt_m128_s(a: m128) -> m128;
        "Stores the value to the reference given.",
        store_m128(r: &mut m128, a: m128);
        "Stores the low lane value to the reference given.",
        store_m128_s(r: &mut f32, a: m128);
        "Stores the value to the reference given in reverse order.",
        store_reverse_m128(r: &mut m128, a: m128);
        "Stores the low lane value to all lanes of the reference given.",
        store_splat_m128(r: &mut m128, a: m128);
        "Stores the value to the reference given.",
        store_unaligned_m128(r: &mut [f32; 4], a: m128);
        "Lanewise `a - b`.",
        sub_m128(a: m128, b: m128) -> m128;
        "Low lane `a - b`, other lanes unchanged.",
        sub_m128_s(a: m128, b: m128) -> m128;
        "Transpose four `m128` as if they were a 4x4 matrix.",
        transpose_four_m128(a: &mut m128, b: &mut m128, c: &mut m128, d: &mut m128);
        "Unpack and interleave high lanes of `a` and `b`.",
        unpack_high_m128(a: m128, b: m128) -> m128;
        "Unpack and interleave low lanes of `a` and `b`.",
        unpack_low_m128(a: m128, b: m128) -> m128;
        "All lanes zero.",
        zeroed_m128() -> m128;
    }
}
impl_arch! {
    #[doc = "# Generic functions requiring the `sse` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/Streaming_SIMD_Extensions>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "sse";

    /// Fetches the cache line containing `addr` into all levels of the cache hierarchy,
    /// anticipating write.
    pub fn prefetch_et0<T>(addr: &T) {
        prefetch_et0(addr);
    }
    /// Fetches into L2 and higher, anticipating write.
    pub fn prefetch_et1<T>(addr: &T) {
        prefetch_et1(addr);
    }
    /// Fetch data using the non-temporal access (NTA) hint.
    ///
    /// It may be a place closer than main memory but outside of the cache hierarchy.
    ///
    /// This is used to reduce access latency without polluting the cache.
    pub fn prefetch_nta<T>(addr: &T) {
        prefetch_nta(addr);
    }
    /// Fetches the cache line containing `addr` into all levels of the cache hierarchy.
    pub fn prefetch_t0<T>(addr: &T) {
        prefetch_t0(addr);
    }
    /// Fetches into L2 and higher.
    pub fn prefetch_t1<T>(addr: &T) {
        prefetch_t1(addr);
    }
    /// Fetches into L3 and higher or an implementation-specific choice
    /// (e.g., L2 if there is no L3).
    pub fn prefetch_t2<T>(addr: &T) {
        prefetch_t2(addr);
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `sse2` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/SSE2>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "sse2";
    arch_fn! {
        "Lanewise `a + b` with lanes as `i16`.",
        add_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a + b` with lanes as `i32`.",
        add_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a + b` with lanes as `i64`.",
        add_i64_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a + b` with lanes as `i8`.",
        add_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a + b`.",
        add_m128d(a: m128d, b: m128d) -> m128d;
        "Lowest lane `a + b`, high lane unchanged.",
        add_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise saturating `a + b` with lanes as `i16`.",
        add_saturating_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise saturating `a + b` with lanes as `i8`.",
        add_saturating_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise saturating `a + b` with lanes as `u16`.",
        add_saturating_u16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise saturating `a + b` with lanes as `u8`.",
        add_saturating_u8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise average of the `u16` values.",
        average_u16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise average of the `u8` values.",
        average_u8_m128i(a: m128i, b: m128i) -> m128i;
        "Bitwise `a &amp; b`.",
        bitand_m128d(a: m128d, b: m128d) -> m128d;
        "Bitwise `a &amp; b`.",
        bitand_m128i(a: m128i, b: m128i) -> m128i;
        "Bitwise `(!a) &amp; b`.",
        bitandnot_m128d(a: m128d, b: m128d) -> m128d;
        "Bitwise `(!a) &amp; b`.",
        bitandnot_m128i(a: m128i, b: m128i) -> m128i;
        "Bitwise `a | b`.",
        bitor_m128d(a: m128d, b: m128d) -> m128d;
        "Bitwise `a | b`.",
        bitor_m128i(a: m128i, b: m128i) -> m128i;
        "Bitwise `a ^ b`.",
        bitxor_m128d(a: m128d, b: m128d) -> m128d;
        "Bitwise `a ^ b`.",
        bitxor_m128i(a: m128i, b: m128i) -> m128i;
        "Shifts all bits in the entire register left by a number of **bytes**.",
        byte_shl_imm_u128_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shifts all bits in the entire register right by a number of **bytes**.",
        byte_shr_imm_u128_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Bit-preserving cast to `m128` from `m128d`",
        cast_to_m128_from_m128d(a: m128d) -> m128;
        "Bit-preserving cast to `m128` from `m128i`",
        cast_to_m128_from_m128i(a: m128i) -> m128;
        "Bit-preserving cast to `m128d` from `m128`",
        cast_to_m128d_from_m128(a: m128) -> m128d;
        "Bit-preserving cast to `m128d` from `m128i`",
        cast_to_m128d_from_m128i(a: m128i) -> m128d;
        "Bit-preserving cast to `m128i` from `m128`",
        cast_to_m128i_from_m128(a: m128) -> m128i;
        "Bit-preserving cast to `m128i` from `m128d`",
        cast_to_m128i_from_m128d(a: m128d) -> m128i;
        "Low lane `f64` equal to.",
        cmp_eq_i32_m128d_s(a: m128d, b: m128d) -> i32;
        "Lanewise `a == b` with lanes as `i16`.",
        cmp_eq_mask_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a == b` with lanes as `i32`.",
        cmp_eq_mask_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a == b` with lanes as `i8`.",
        cmp_eq_mask_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a == b`, mask output.",
        cmp_eq_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `a == b`, other lanes unchanged.",
        cmp_eq_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Low lane `f64` greater than or equal to.",
        cmp_ge_i32_m128d_s(a: m128d, b: m128d) -> i32;
        "Lanewise `a &gt;= b`.",
        cmp_ge_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `a &gt;= b`, other lanes unchanged.",
        cmp_ge_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Low lane `f64` greater than.",
        cmp_gt_i32_m128d_s(a: m128d, b: m128d) -> i32;
        "Lanewise `a &gt; b` with lanes as `i16`.",
        cmp_gt_mask_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a &gt; b` with lanes as `i32`.",
        cmp_gt_mask_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a &gt; b` with lanes as `i8`.",
        cmp_gt_mask_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a &gt; b`.",
        cmp_gt_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `a &gt; b`, other lanes unchanged.",
        cmp_gt_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Low lane `f64` less than or equal to.",
        cmp_le_i32_m128d_s(a: m128d, b: m128d) -> i32;
        "Lanewise `a &lt;= b`.",
        cmp_le_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `a &lt;= b`, other lanes unchanged.",
        cmp_le_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Low lane `f64` less than.",
        cmp_lt_i32_m128d_s(a: m128d, b: m128d) -> i32;
        "Lanewise `a &lt; b` with lanes as `i16`.",
        cmp_lt_mask_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a &lt; b` with lanes as `i32`.",
        cmp_lt_mask_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a &lt; b` with lanes as `i8`.",
        cmp_lt_mask_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a &lt; b`.",
        cmp_lt_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `a &lt; b`, other lane unchanged.",
        cmp_lt_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Low lane `f64` less than.",
        cmp_neq_i32_m128d_s(a: m128d, b: m128d) -> i32;
        "Lanewise `a != b`.",
        cmp_neq_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `a != b`, other lane unchanged.",
        cmp_neq_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `!(a &gt;= b)`.",
        cmp_nge_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `!(a &gt;= b)`, other lane unchanged.",
        cmp_nge_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `!(a &gt; b)`.",
        cmp_ngt_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `!(a &gt; b)`, other lane unchanged.",
        cmp_ngt_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `!(a &lt;= b)`.",
        cmp_nle_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `!(a &lt;= b)`, other lane unchanged.",
        cmp_nle_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `!(a &lt; b)`.",
        cmp_nlt_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `!(a &lt; b)`, other lane unchanged.",
        cmp_nlt_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `(!a.is_nan()) &amp; (!b.is_nan())`.",
        cmp_ordered_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `(!a.is_nan()) &amp; (!b.is_nan())`, other lane unchanged.",
        cmp_ordered_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `a.is_nan() | b.is_nan()`.",
        cmp_unord_mask_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `a.is_nan() | b.is_nan()`, other lane unchanged.",
        cmp_unord_mask_m128d_s(a: m128d, b: m128d) -> m128d;
        "Convert `i32` to `f64` and replace the low lane of the input.",
        convert_i32_replace_m128d_s(a: m128d, i: i32) -> m128d;
        "Convert `i64` to `f64` and replace the low lane of the input.",
        convert_i64_replace_m128d_s(a: m128d, i: i64) -> m128d;
        "Converts the lower `f32` to `f64` and replace the low lane of the input",
        convert_m128_s_replace_m128d_s(a: m128d, b: m128) -> m128d;
        "Converts the low `f64` to `f32` and replaces the low lane of the input.",
        convert_m128d_s_replace_m128_s(a: m128, b: m128d) -> m128;
        "Rounds the `f32` lanes to `i32` lanes.",
        convert_to_i32_m128i_from_m128(a: m128) -> m128i;
        "Rounds the two `f64` lanes to the low two `i32` lanes.",
        convert_to_i32_m128i_from_m128d(a: m128d) -> m128i;
        "Rounds the four `i32` lanes to four `f32` lanes.",
        convert_to_m128_from_i32_m128i(a: m128i) -> m128;
        "Rounds the two `f64` lanes to the low two `f32` lanes.",
        convert_to_m128_from_m128d(a: m128d) -> m128;
        "Rounds the lower two `i32` lanes to two `f64` lanes.",
        convert_to_m128d_from_lower2_i32_m128i(a: m128i) -> m128d;
        "Rounds the two `f64` lanes to the low two `f32` lanes.",
        convert_to_m128d_from_lower2_m128(a: m128) -> m128d;
        "Copy the low `i64` lane to a new register, upper bits 0.",
        copy_i64_m128i_s(a: m128i) -> m128i;
        "Copies the `a` value and replaces the low lane with the low `b` value.",
        copy_replace_low_f64_m128d(a: m128d, b: m128d) -> m128d;
        "Lanewise `a / b`.",
        div_m128d(a: m128d, b: m128d) -> m128d;
        "Lowest lane `a / b`, high lane unchanged.",
        div_m128d_s(a: m128d, b: m128d) -> m128d;
        "Gets an `i16` value out of an `m128i`, returns as `i32`.",
        extract_i16_as_i32_m128i<const LANE: i32>(a: m128i) -> i32;
        "Gets the lower lane as an `f64` value.",
        get_f64_from_m128d_s(a: m128d) -> f64;
        "Converts the lower lane to an `i32` value.",
        get_i32_from_m128d_s(a: m128d) -> i32;
        "Converts the lower lane to an `i32` value.",
        get_i32_from_m128i_s(a: m128i) -> i32;
        "Converts the lower lane to an `i64` value.",
        get_i64_from_m128d_s(a: m128d) -> i64;
        "Converts the lower lane to an `i64` value.",
        get_i64_from_m128i_s(a: m128i) -> i64;
        "Inserts the low 16 bits of an `i32` value into an `m128i`.",
        insert_i16_from_i32_m128i<const LANE: i32>(a: m128i, i: i32) -> m128i;
        "Loads the reference into the low lane of the register.",
        load_f64_m128d_s(a: &f64) -> m128d;
        "Loads the `f64` reference into all lanes of a register.",
        load_f64_splat_m128d(a: &f64) -> m128d;
        "Loads the low `i64` into a register.",
        load_i64_m128i_s(a: &m128i) -> m128i;
        "Loads the reference into a register.",
        load_m128d(a: &m128d) -> m128d;
        "Loads the reference into a register.",
        load_m128i(a: &m128i) -> m128i;
        "Loads the reference into a register, replacing the high lane.",
        load_replace_high_m128d(a: m128d, b: &f64) -> m128d;
        "Loads the reference into a register, replacing the low lane.",
        load_replace_low_m128d(a: m128d, b: &f64) -> m128d;
        "Loads the reference into a register with reversed order.",
        load_reverse_m128d(a: &m128d) -> m128d;
        "Loads the reference into a register.",
        load_unaligned_m128d(a: &[f64; 2]) -> m128d;
        "Loads the reference into a register.",
        load_unaligned_m128i(a: &[u8; 16]) -> m128i;
        "Lanewise `max(a, b)` with lanes as `i16`.",
        max_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `max(a, b)`.",
        max_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `max(a, b)`, other lanes unchanged.",
        max_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `max(a, b)` with lanes as `u8`.",
        max_u8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `min(a, b)` with lanes as `i16`.",
        min_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `min(a, b)`.",
        min_m128d(a: m128d, b: m128d) -> m128d;
        "Low lane `min(a, b)`, other lanes unchanged.",
        min_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `min(a, b)` with lanes as `u8`.",
        min_u8_m128i(a: m128i, b: m128i) -> m128i;
        "Gathers the `i8` sign bit of each lane.",
        move_mask_i8_m128i(a: m128i) -> i32;
        "Gathers the sign bit of each lane.",
        move_mask_m128d(a: m128d) -> i32;
        "Multiply `i16` lanes producing `i32` values, horizontal add pairs of `i32`
        values to produce the final output.",
        mul_i16_horizontal_add_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a * b` with lanes as `i16`, keep the high bits of the `i32` intermediates.",
        mul_i16_keep_high_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a * b` with lanes as `i16`, keep the low bits of the `i32` intermediates.",
        mul_i16_keep_low_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a * b`.",
        mul_m128d(a: m128d, b: m128d) -> m128d;
        "Lowest lane `a * b`, high lane unchanged.",
        mul_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `a * b` with lanes as `u16`, keep the high bits of the `u32` intermediates.",
        mul_u16_keep_high_m128i(a: m128i, b: m128i) -> m128i;
        "Multiplies the odd `u32` lanes and gives the widened (`u64`) results.",
        mul_widen_u32_odd_m128i(a: m128i, b: m128i) -> m128i;
        "Saturating convert `i16` to `i8`, and pack the values.",
        pack_i16_to_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Saturating convert `i16` to `u8`, and pack the values.",
        pack_i16_to_u8_m128i(a: m128i, b: m128i) -> m128i;
        "Saturating convert `i32` to `i16`, and pack the values.",
        pack_i32_to_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Sets the args into an `m128i`, first arg is the high lane.",
        set_i16_m128i(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16, g: i16, h: i16) -> m128i;
        "Sets the args into an `m128i`, first arg is the high lane.",
        set_i32_m128i(a: i32, b: i32, c: i32, d: i32) -> m128i;
        "Set an `i32` as the low 32-bit lane of an `m128i`, other lanes blank.",
        set_i32_m128i_s(i: i32) -> m128i;
        "Sets the args into an `m128i`, first arg is the high lane.",
        set_i64_m128i(a: i64, b: i64) -> m128i;
        "Set an `i64` as the low 64-bit lane of an `m128i`, other lanes blank.",
        set_i64_m128i_s(i: i64) -> m128i;
        "Sets the args into an `m128i`, first arg is the high lane.",
        set_i8_m128i(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8, i: i8, j: i8,
            k: i8, l: i8, m: i8, n: i8, o: i8, p: i8) -> m128i;
        "Sets the args into an `m128d`, first arg is the high lane.",
        set_m128d(a: f64, b: f64) -> m128d;
        "Sets the args into the low lane of a `m128d`.",
        set_m128d_s(a: f64) -> m128d;
        "Sets the args into an `m128i`, first arg is the low lane.",
        set_reversed_i16_m128i(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16, g: i16,
            h: i16) -> m128i;
        "Sets the args into an `m128i`, first arg is the low lane.",
        set_reversed_i32_m128i(a: i32, b: i32, c: i32, d: i32) -> m128i;
        "Sets the args into an `m128i`, first arg is the low lane.",
        set_reversed_i8_m128i(a: i8, b: i8, c: i8, d: i8, e: i8, f: i8, g: i8, h: i8, i: i8,
            j: i8, k: i8, l: i8, m: i8, n: i8, o: i8, p: i8) -> m128i;
        "Sets the args into an `m128d`, first arg is the low lane.",
        set_reversed_m128d(a: f64, b: f64) -> m128d;
        "Splats the `i16` to all lanes of the `m128i`.",
        set_splat_i16_m128i(i: i16) -> m128i;
        "Splats the `i32` to all lanes of the `m128i`.",
        set_splat_i32_m128i(i: i32) -> m128i;
        "Splats the `i64` to both lanes of the `m128i`.",
        set_splat_i64_m128i(i: i64) -> m128i;
        "Splats the `i8` to all lanes of the `m128i`.",
        set_splat_i8_m128i(i: i8) -> m128i;
        "Splats the args into both lanes of the `m128d`.",
        set_splat_m128d(a: f64) -> m128d;
        "Shift all `u16` lanes to the left by the `count` in the lower `u64` lane.",
        shl_all_u16_m128i(a: m128i, count: m128i) -> m128i;
        "Shift all `u32` lanes to the left by the `count` in the lower `u64` lane.",
        shl_all_u32_m128i(a: m128i, count: m128i) -> m128i;
        "Shift all `u64` lanes to the left by the `count` in the lower `u64` lane.",
        shl_all_u64_m128i(a: m128i, count: m128i) -> m128i;
        "Shifts all `u16` lanes left by an immediate.",
        shl_imm_u16_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shifts all `u32` lanes left by an immediate.",
        shl_imm_u32_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shifts both `u64` lanes left by an immediate.",
        shl_imm_u64_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shift each `i16` lane to the right by the `count` in the lower `i64` lane.",
        shr_all_i16_m128i(a: m128i, count: m128i) -> m128i;
        "Shift each `i32` lane to the right by the `count` in the lower `i64` lane.",
        shr_all_i32_m128i(a: m128i, count: m128i) -> m128i;
        "Shift each `u16` lane to the right by the `count` in the lower `u64` lane.",
        shr_all_u16_m128i(a: m128i, count: m128i) -> m128i;
        "Shift each `u32` lane to the right by the `count` in the lower `u64` lane.",
        shr_all_u32_m128i(a: m128i, count: m128i) -> m128i;
        "Shift each `u64` lane to the right by the `count` in the lower `u64` lane.",
        shr_all_u64_m128i(a: m128i, count: m128i) -> m128i;
        "Shifts all `i16` lanes right by an immediate.",
        shr_imm_i16_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shifts all `i32` lanes right by an immediate.",
        shr_imm_i32_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shifts all `u16` lanes right by an immediate.",
        shr_imm_u16_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shifts all `u32` lanes right by an immediate.",
        shr_imm_u32_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shifts both `u64` lanes right by an immediate.",
        shr_imm_u64_m128i<const IMM: i32>(a: m128i) -> m128i;
        "Shuffle the `f64` lanes from `$a` and `$b` together using an immediate control value.",
        shuffle_abi_f64_all_m128d<const MASK: i32>(a: m128d, b: m128d) -> m128d;
        "Shuffle the `i32` lanes in `$a` using an immediate control value.",
        shuffle_ai_f32_all_m128i<const MASK: i32>(a: m128i) -> m128i;
        "Shuffle the high `i16` lanes in `$a` using an immediate control value.",
        shuffle_ai_i16_h64all_m128i<const MASK: i32>(a: m128i) -> m128i;
        "Shuffle the low `i16` lanes in `$a` using an immediate control value.",
        shuffle_ai_i16_l64all_m128i<const MASK: i32>(a: m128i) -> m128i;
        "Lanewise `sqrt(a)`.",
        sqrt_m128d(a: m128d) -> m128d;
        "Low lane `sqrt(b)`, upper lane is unchanged from `a`.",
        sqrt_m128d_s(a: m128d, b: m128d) -> m128d;
        "Stores the high lane value to the reference given.",
        store_high_m128d_s(r: &mut f64, a: m128d);
        "Stores the value to the reference given.",
        store_i64_m128i_s(r: &mut i64, a: m128i);
        "Stores the value to the reference given.",
        store_m128d(r: &mut m128d, a: m128d);
        "Stores the low lane value to the reference given.",
        store_m128d_s(r: &mut f64, a: m128d);
        "Stores the value to the reference given.",
        store_m128i(r: &mut m128i, a: m128i);
        "Stores the value to the reference given.",
        store_reversed_m128d(r: &mut m128d, a: m128d);
        "Stores the low lane value to all lanes of the reference given.",
        store_splat_m128d(r: &mut m128d, a: m128d);
        "Stores the value to the reference given.",
        store_unaligned_m128d(r: &mut [f64; 2], a: m128d);
        "Stores the value to the reference given.",
        store_unaligned_m128i(r: &mut [u8; 16], a: m128i);
        "Lanewise `a - b` with lanes as `i16`.",
        sub_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a - b` with lanes as `i32`.",
        sub_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a - b` with lanes as `i64`.",
        sub_i64_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a - b` with lanes as `i8`.",
        sub_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a - b`.",
        sub_m128d(a: m128d, b: m128d) -> m128d;
        "Lowest lane `a - b`, high lane unchanged.",
        sub_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise saturating `a - b` with lanes as `i16`.",
        sub_saturating_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise saturating `a - b` with lanes as `i8`.",
        sub_saturating_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise saturating `a - b` with lanes as `u16`.",
        sub_saturating_u16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise saturating `a - b` with lanes as `u8`.",
        sub_saturating_u8_m128i(a: m128i, b: m128i) -> m128i;
        "Compute “sum of `u8` absolute differences”.",
        sum_of_u8_abs_diff_m128i(a: m128i, b: m128i) -> m128i;
        "Truncate the `f32` lanes to `i32` lanes.",
        truncate_m128_to_m128i(a: m128) -> m128i;
        "Truncate the `f64` lanes to the lower `i32` lanes (upper `i32` lanes 0).",
        truncate_m128d_to_m128i(a: m128d) -> m128i;
        "Truncate the lower lane into an `i32`.",
        truncate_to_i32_m128d_s(a: m128d) -> i32;
        "Truncate the lower lane into an `i64`.",
        truncate_to_i64_m128d_s(a: m128d) -> i64;
        "Unpack and interleave high `i16` lanes of `a` and `b`.",
        unpack_high_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Unpack and interleave high `i32` lanes of `a` and `b`.",
        unpack_high_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Unpack and interleave high `i64` lanes of `a` and `b`.",
        unpack_high_i64_m128i(a: m128i, b: m128i) -> m128i;
        "Unpack and interleave high `i8` lanes of `a` and `b`.",
        unpack_high_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Unpack and interleave high lanes of `a` and `b`.",
        unpack_high_m128d(a: m128d, b: m128d) -> m128d;
        "Unpack and interleave low `i16` lanes of `a` and `b`.",
        unpack_low_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Unpack and interleave low `i32` lanes of `a` and `b`.",
        unpack_low_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Unpack and interleave low `i64` lanes of `a` and `b`.",
        unpack_low_i64_m128i(a: m128i, b: m128i) -> m128i;
        "Unpack and interleave low `i8` lanes of `a` and `b`.",
        unpack_low_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Unpack and interleave low lanes of `a` and `b`.",
        unpack_low_m128d(a: m128d, b: m128d) -> m128d;
        "Both lanes zero.",
        zeroed_m128d() -> m128d;
        "All lanes zero.",
        zeroed_m128i() -> m128i;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `sse3` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/SSE3>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "sse3";
    arch_fn! {
        "Add each lane horizontally, pack the outputs as `a` then `b`.",
        add_horizontal_m128(a: m128, b: m128) -> m128;
        "Add each lane horizontally, pack the outputs as `a` then `b`.",
        add_horizontal_m128d(a: m128d, b: m128d) -> m128d;
        "Alternately, from the top, add a lane and then subtract a lane.",
        addsub_m128(a: m128, b: m128) -> m128;
        "Add the high lane and subtract the low lane.",
        addsub_m128d(a: m128d, b: m128d) -> m128d;
        "Duplicate the odd lanes to the even lanes.",
        duplicate_even_lanes_m128(a: m128) -> m128;
        "Copy the low lane of the input to both lanes of the output.",
        duplicate_low_lane_m128d_s(a: m128d) -> m128d;
        "Duplicate the odd lanes to the even lanes.",
        duplicate_odd_lanes_m128(a: m128) -> m128;
        "Subtract each lane horizontally, pack the outputs as `a` then `b`.",
        sub_horizontal_m128(a: m128, b: m128) -> m128;
        "Subtract each lane horizontally, pack the outputs as `a` then `b`.",
        sub_horizontal_m128d(a: m128d, b: m128d) -> m128d;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `sse4.1` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/SSE4#SSE4.1>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "sse4.1";
    arch_fn! {
        "Blends the `i16` lanes according to the immediate mask.",
        blend_imm_i16_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i;
        "Blends the lanes according to the immediate mask.",
        blend_imm_m128<const IMM: i32>(a: m128, b: m128) -> m128;
        "Blends the `i16` lanes according to the immediate mask.",
        blend_imm_m128d<const IMM: i32>(a: m128d, b: m128d) -> m128d;
        "Blend the `i8` lanes according to a runtime varying mask.",
        blend_varying_i8_m128i(a: m128i, b: m128i, mask: m128i) -> m128i;
        "Blend the lanes according to a runtime varying mask.",
        blend_varying_m128(a: m128, b: m128, mask: m128) -> m128;
        "Blend the lanes according to a runtime varying mask.",
        blend_varying_m128d(a: m128d, b: m128d, mask: m128d) -> m128d;
        "Round each lane to a whole number, towards positive infinity.",
        ceil_m128(a: m128) -> m128;
        "Round the low lane of `b` toward positive infinity, other lanes `a`.",
        ceil_m128_s(a: m128, b: m128) -> m128;
        "Round each lane to a whole number, towards positive infinity.",
        ceil_m128d(a: m128d) -> m128d;
        "Round the low lane of `b` toward positive infinity, high lane is `a`.",
        ceil_m128d_s(a: m128d, b: m128d) -> m128d;
        "Lanewise `a == b` with lanes as `i64`.",
        cmp_eq_mask_i64_m128i(a: m128i, b: m128i) -> m128i;
        "Convert the lower two `i64` lanes to two `i32` lanes.",
        convert_to_i16_m128i_from_lower2_i16_m128i(a: m128i) -> m128i;
        "Convert the lower eight `i8` lanes to eight `i16` lanes.",
        convert_to_i16_m128i_from_lower8_i8_m128i(a: m128i) -> m128i;
        "Convert the lower four `i16` lanes to four `i32` lanes.",
        convert_to_i32_m128i_from_lower4_i16_m128i(a: m128i) -> m128i;
        "Convert the lower four `i8` lanes to four `i32` lanes.",
        convert_to_i32_m128i_from_lower4_i8_m128i(a: m128i) -> m128i;
        "Convert the lower two `i32` lanes to two `i64` lanes.",
        convert_to_i64_m128i_from_lower2_i32_m128i(a: m128i) -> m128i;
        "Convert the lower two `i8` lanes to two `i64` lanes.",
        convert_to_i64_m128i_from_lower2_i8_m128i(a: m128i) -> m128i;
        "Convert the lower eight `u8` lanes to eight `u16` lanes.",
        convert_to_u16_m128i_from_lower8_u8_m128i(a: m128i) -> m128i;
        "Convert the lower four `u16` lanes to four `u32` lanes.",
        convert_to_u32_m128i_from_lower4_u16_m128i(a: m128i) -> m128i;
        "Convert the lower four `u8` lanes to four `u32` lanes.",
        convert_to_u32_m128i_from_lower4_u8_m128i(a: m128i) -> m128i;
        "Convert the lower two `u16` lanes to two `u64` lanes.",
        convert_to_u64_m128i_from_lower2_u16_m128i(a: m128i) -> m128i;
        "Convert the lower two `u32` lanes to two `u64` lanes.",
        convert_to_u64_m128i_from_lower2_u32_m128i(a: m128i) -> m128i;
        "Convert the lower two `u8` lanes to two `u64` lanes.",
        convert_to_u64_m128i_from_lower2_u8_m128i(a: m128i) -> m128i;
        "Performs a dot product of two `m128` registers.",
        dot_product_m128<const IMM: i32>(a: m128, b: m128) -> m128;
        "Performs a dot product of two `m128d` registers.",
        dot_product_m128d<const IMM: i32>(a: m128d, b: m128d) -> m128d;
        "Gets the `f32` lane requested. Returns as an `i32` bit pattern.",
        extract_f32_as_i32_bits_imm_m128<const IMM: i32>(a: m128) -> i32;
        "Gets the `i32` lane requested. Only the lowest 2 bits are considered.",
        extract_i32_imm_m128i<const IMM: i32>(a: m128i) -> i32;
        "Gets the `i64` lane requested. Only the lowest bit is considered.",
        extract_i64_imm_m128i<const IMM: i32>(a: m128i) -> i64;
        "Gets the `i8` lane requested. Only the lowest 4 bits are considered.",
        extract_i8_as_i32_imm_m128i<const IMM: i32>(a: m128i) -> i32;
        "Round each lane to a whole number, towards negative infinity",
        floor_m128(a: m128) -> m128;
        "Round the low lane of `b` toward negative infinity, other lanes `a`.",
        floor_m128_s(a: m128, b: m128) -> m128;
        "Round each lane to a whole number, towards negative infinity",
        floor_m128d(a: m128d) -> m128d;
        "Round the low lane of `b` toward negative infinity, high lane is `a`.",
        floor_m128d_s(a: m128d, b: m128d) -> m128d;
        "Inserts a lane from `$b` into `$a`, optionally at a new position.",
        insert_f32_imm_m128<const IMM: i32>(a: m128, b: m128) -> m128;
        "Inserts a new value for the `i32` lane specified.",
        insert_i32_imm_m128i<const IMM: i32>(a: m128i, new: i32) -> m128i;
        "Inserts a new value for the `i64` lane specified.",
        insert_i64_imm_m128i<const IMM: i32>(a: m128i, new: i64) -> m128i;
        "Inserts a new value for the `i64` lane specified.",
        insert_i8_imm_m128i<const IMM: i32>(a: m128i, new: i32) -> m128i;
        "Lanewise `max(a, b)` with lanes as `i32`.",
        max_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `max(a, b)` with lanes as `i8`.",
        max_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `max(a, b)` with lanes as `u16`.",
        max_u16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `max(a, b)` with lanes as `u32`.",
        max_u32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `min(a, b)` with lanes as `i32`.",
        min_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `min(a, b)` with lanes as `i8`.",
        min_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Min `u16` value, position, and other lanes zeroed.",
        min_position_u16_m128i(a: m128i) -> m128i;
        "Lanewise `min(a, b)` with lanes as `u16`.",
        min_u16_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `min(a, b)` with lanes as `u32`.",
        min_u32_m128i(a: m128i, b: m128i) -> m128i;
        "Lanewise `a * b` with 32-bit lanes.",
        mul_32_m128i(a: m128i, b: m128i) -> m128i;
        "Multiplies the odd `i32` lanes and gives the widened (`i64`) results.",
        mul_widen_i32_odd_m128i(a: m128i, b: m128i) -> m128i;
        "Computes eight `u16` “sum of absolute difference” values according to the bytes selected.",
        multi_packed_sum_abs_diff_u8_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i;
        "Saturating convert `i32` to `u16`, and pack the values.",
        pack_i32_to_u16_m128i(a: m128i, b: m128i) -> m128i;
        "Rounds each lane in the style specified.",
        round_m128<const MODE: i32>(a: m128) -> m128;
        "Rounds `$b` low as specified, other lanes use `$a`.",
        round_m128_s<const MODE: i32>(a: m128, b: m128) -> m128;
        "Rounds each lane in the style specified.",
        round_m128d<const MODE: i32>(a: m128d) -> m128d;
        "Rounds `$b` low as specified, keeps `$a` high.",
        round_m128d_s<const MODE: i32>(a: m128d, b: m128d) -> m128d;
        "Tests if all bits are 1.",
        test_all_ones_m128i(a: m128i) -> i32;
        "Returns if all masked bits are 0, `(a &amp; mask) as u128 == 0`",
        test_all_zeroes_m128i(a: m128i, mask: m128i) -> i32;
        "Returns if, among the masked bits, there’s both 0s and 1s",
        test_mixed_ones_and_zeroes_m128i(a: m128i, mask: m128i) -> i32;
        "Compute the bitwise NOT of `a` and then AND with `b`,
        returns 1 if the result is zero, otherwise 0.",
        testc_m128i(a: m128i, b: m128i) -> i32;
        "Computes the bitwise AND of 256 bits in `a` and
        `b`, returns 1 if the result is zero, otherwise 0.",
        testz_m128i(a: m128i, b: m128i) -> i32;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `sse4.2` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/SSE4#SSE4.2>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "sse4.2";
    arch_fn! {
        "Lanewise `a &gt; b` with lanes as `i64`.",
        cmp_gt_mask_i64_m128i(a: m128i, b: m128i) -> m128i;
        "Accumulates the `u16` into a running CRC32 value.",
        crc32_u16(crc: u32, v: u16) -> u32;
        "Accumulates the `u32` into a running CRC32 value.",
        crc32_u32(crc: u32, v: u32) -> u32;
        "Accumulates the `u64` into a running CRC32 value.",
        crc32_u64(crc: u64, v: u64) -> u64;
        "Accumulates the `u8` into a running CRC32 value.",
        crc32_u8(crc: u32, v: u8) -> u32;
        "Search for `needle` in `haystack, with explicit string length.",
        search_explicit_str_for_index<const IMM: i32>(
        needle: m128i, needle_len: i32, haystack: m128i, haystack_len: i32) -> i32;
        "Search for `needle` in `haystack, with explicit string length.",
        search_explicit_str_for_mask<const IMM: i32>(
        needle: m128i, needle_len: i32, haystack: m128i, haystack_len: i32) -> m128i;
        "Search for `needle` in `haystack, with implicit string length.",
        search_implicit_str_for_index<const IMM: i32>(needle: m128i, haystack: m128i) -> i32;
        "Search for `needle` in `haystack, with implicit string length.",
        search_implicit_str_for_mask<const IMM: i32>(needle: m128i, haystack: m128i) -> m128i;
    }
}
impl_arch! {
    #[doc = "# Functions requiring the `ssse3` target feature.\n\n---"]
    #[doc = "See: <https://en.wikipedia.org/wiki/SSSE3>"]
    features = "dep_safe_arch", any_target_arch = "x86", "x86_64", target_features = "ssse3";
    arch_fn! {
        "Lanewise absolute value with lanes as `i16`.",
        abs_i16_m128i(a: m128i) -> m128i;
        "Lanewise absolute value with lanes as `i32`.",
        abs_i32_m128i(a: m128i) -> m128i;
        "Lanewise absolute value with lanes as `i8`.",
        abs_i8_m128i(a: m128i) -> m128i;
        "Add horizontal pairs of `i16` values, pack the outputs as `a` then `b`.",
        add_horizontal_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Add horizontal pairs of `i32` values, pack the outputs as `a` then `b`.",
        add_horizontal_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Add horizontal pairs of `i16` values, saturating, pack the outputs as `a` then `b`.",
        add_horizontal_saturating_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Counts `$a` as the high bytes and `$b` as the low bytes then performs a
        **byte** shift to the right by the immediate value.",
        combined_byte_shr_imm_m128i<const IMM: i32>(a: m128i, b: m128i) -> m128i;
        "Multiply `i16` lanes into `i32` intermediates, keep the high 18 bits,
        round by adding 1, right shift by 1.",
        mul_i16_scale_round_m128i(a: m128i, b: m128i) -> m128i;
        "This is dumb and weird.",
        mul_u8i8_add_horizontal_saturating_m128i(a: m128i, b: m128i) -> m128i;
        "Shuffle `i8` lanes in `a` using `i8` values in `v`.",
        shuffle_av_i8z_all_m128i(a: m128i, v: m128i) -> m128i;
        "Applies the sign of `i16` values in `b` to the values in `a`.",
        sign_apply_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Applies the sign of `i32` values in `b` to the values in `a`.",
        sign_apply_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Applies the sign of `i8` values in `b` to the values in `a`.",
        sign_apply_i8_m128i(a: m128i, b: m128i) -> m128i;
        "Subtract horizontal pairs of `i16` values, pack the outputs as `a` then `b`.",
        sub_horizontal_i16_m128i(a: m128i, b: m128i) -> m128i;
        "Subtract horizontal pairs of `i32` values, pack the outputs as `a` then `b`.",
        sub_horizontal_i32_m128i(a: m128i, b: m128i) -> m128i;
        "Subtract horizontal pairs of `i16` values, saturating, pack the outputs as `a` then `b`.",
        sub_horizontal_saturating_i16_m128i(a: m128i, b: m128i) -> m128i;
    }
}

/* macro helpers */

/// Generates an impl Arch block with optional conditional configurations and documentation.
macro_rules! impl_arch {
    (
        $( #[doc = $doc:literal] )*
        $( features = $( $feature:literal ),+ $(,)? )? // all
        $( any_target_arch = $( $target_arch:literal ),+ $(,)? )? // any
        $( target_features = $( $target_feature:literal ),+ $(,)? )? // all
        ;
        $($item:item)*
    ) => {
        $( #[doc = $doc] )*
        $(
        #[cfg(any($(feature = $feature),+))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(any($(feature = $feature),+))))]
        )?
        $(
        #[cfg(any($(target_arch = $target_arch),+))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(any($(target_arch = $target_arch),+))))]
        )?
        $(
        #[cfg(any($(target_feature = $target_feature),+))]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(any($(target_feature = $target_feature),+))))]
        )?
        impl Arch { $($item)* }
    };
}
use impl_arch;

/// Helps to re-export standalone functions as namespaced methods of a struct.
#[allow(unused_macros, reason = "feature-gated")]
macro_rules! arch_fn {
    () => {};
    (   // Function with return type
        $doc:literal,
        $fn_name:ident$(<$(const $const_name:ident: $const_ty:ty),*>)?
        ($($param:ident: $ty:ty),* $(,)?) -> $ret:ty
     ) => { $crate::paste! { // NOTE: compiles faster using paste! than concat! + stringify!
        #[doc = $doc]
        #[doc = "\n\nSee: [`" $fn_name "`][crate::_dep::safe_arch::" $fn_name "]."] // faster
        // #[doc = concat!("\n\nSee: [`", stringify!($fn_name), "`][", stringify!($fn_name), "].")]
        #[must_use]
        pub fn $fn_name$(<$(const $const_name: $const_ty),*>)?($($param: $ty),*) -> $ret {
            $fn_name$(::<$($const_name),*>)?($($param),*)
        }
    }};
    (   // Function without return type
        $doc:literal,
        $fn_name:ident$(<$(const $const_name:ident: $const_ty:ty),*>)?
        ($($param:ident: $ty:ty),* $(,)?)
    ) => { $crate::paste! {
        #[doc = $doc]
        #[doc = "\n\nSee: [`" $fn_name "`][crate::_dep::safe_arch::" $fn_name "]."] // faster
        // #[doc = concat!("\n\nSee: [`", stringify!($fn_name), "`][", stringify!($fn_name), "].")]
        pub fn $fn_name$(<$(const $const_name: $const_ty),*>)?($($param: $ty),*) {
            $fn_name$(::<$($const_name),*>)?($($param),*)
        }
    }};
    (   // List of functions
        $($doc:literal,
        $fn_name:ident$(<$(const $const_name:ident: $const_ty:ty),*>)?
        ($($param:ident: $ty:ty),* $(,)?) $(-> $ret:ty)?);+ $(;)?
    ) => {
        $( arch_fn![
            $doc,
            $fn_name$(<$(const $const_name: $const_ty),*>)?($($param: $ty),*) $(-> $ret)?
        ]; )+
    };
}
#[allow(unused_imports, reason = "feature-gated")]
use arch_fn;
