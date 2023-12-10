// devela::data::conversion::primitive::tests
//
// TOC
// - bit_mask_range
// - bits_ops
// - split_join_le
// - split_join_be

use super::*;

#[test]
#[rustfmt::skip]
fn bit_mask_range() {
    assert_eq![0b_0000_0001, bit_mask_checked_range_u8(0, 0).unwrap()];
    assert_eq![0b_0000_0001, bit_mask_range_u8(0, 0)];
    assert_eq![0b_1000_0000, bit_mask_checked_range_u8(7, 7).unwrap()];
    assert_eq![0b_1000_0000, bit_mask_range_u8(7, 7)];
    assert_eq![0b_0111_1110, bit_mask_checked_range_u8(1, 6).unwrap()];
    assert_eq![0b_0111_1110, bit_mask_range_u8(1, 6)];

    debug_assert![bit_mask_checked_range_u8(8, 8).is_err()];
    debug_assert![bit_mask_checked_range_u8(0, 8).is_err()];
    debug_assert![bit_mask_checked_range_u8(4, 1).is_err()];
    #[cfg(feature = "std")]
    {
        use std::panic::catch_unwind;
        debug_assert![catch_unwind(|| { let _ = bit_mask_checked_range_u8(8, 8); }).is_err()];
        debug_assert![catch_unwind(|| { let _ = bit_mask_checked_range_u8(0, 8); }).is_err()];
        debug_assert![catch_unwind(|| { let _ = bit_mask_checked_range_u8(4, 1); }).is_err()];
    }
}
#[test]
#[rustfmt::skip]
fn bits_ops() {
    let bits = 0b_1111_0000;
    assert_eq![0b_0011_0000, bit_get_checked_range_u8(bits, 2, 5).unwrap()];
    assert_eq![0b_0000_1100, bit_get_shifted_checked_range_u8(bits, 2, 5).unwrap()];
    assert_eq![0b_1111_1100, bit_set_checked_range_u8(bits, 2, 5).unwrap()];
    assert_eq![0b_1100_0000, bit_unset_checked_range_u8(bits, 2, 5).unwrap()];
    assert_eq![0b_1100_1100, bit_flip_checked_range_u8(bits, 2, 5).unwrap()];
}

#[test]
fn split_join_le() {
    let v = u16::MAX / 2;
    let split = u16_into_u8_le(v);
    assert_eq![v, u16_from_u8_le(split)];

    let v = u32::MAX / 2;
    let split = u32_into_u16_le(v);
    assert_eq![v, u32_from_u16_le(split)];
    let split = u32_into_u8_le(v);
    assert_eq![v, u32_from_u8_le(split)];

    let v = u64::MAX / 2;
    let split = u64_into_u32_le(v);
    assert_eq![v, u64_from_u32_le(split)];
    let split = u64_into_u16_le(v);
    assert_eq![v, u64_from_u16_le(split)];
    let split = u64_into_u8_le(v);
    assert_eq![v, u64_from_u8_le(split)];

    let v = u128::MAX / 2;
    let split = u128_into_u64_le(v);
    assert_eq![v, u128_from_u64_le(split)];
    let split = u128_into_u32_le(v);
    assert_eq![v, u128_from_u32_le(split)];
    let split = u128_into_u16_le(v);
    assert_eq![v, u128_from_u16_le(split)];
    let split = u128_into_u8_le(v);
    assert_eq![v, u128_from_u8_le(split)];
}

#[test]
fn split_join_be() {
    let v = u16::MAX / 2;
    let split = u16_into_u8_be(v);
    assert_eq![v, u16_from_u8_be(split)];

    let v = u32::MAX / 2;
    let split = u32_into_u16_be(v);
    assert_eq![v, u32_from_u16_be(split)];
    let split = u32_into_u8_be(v);
    assert_eq![v, u32_from_u8_be(split)];

    let v = u64::MAX / 2;
    let split = u64_into_u32_be(v);
    assert_eq![v, u64_from_u32_be(split)];
    let split = u64_into_u16_be(v);
    assert_eq![v, u64_from_u16_be(split)];
    let split = u64_into_u8_be(v);
    assert_eq![v, u64_from_u8_be(split)];

    let v = u128::MAX / 2;
    let split = u128_into_u64_be(v);
    assert_eq![v, u128_from_u64_be(split)];
    let split = u128_into_u32_be(v);
    assert_eq![v, u128_from_u32_be(split)];
    let split = u128_into_u16_be(v);
    assert_eq![v, u128_from_u16_be(split)];
    let split = u128_into_u8_be(v);
    assert_eq![v, u128_from_u8_be(split)];
}
