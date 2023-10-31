// devela::ops::conversion::primitive::tests

use super::*;

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