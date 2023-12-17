// devela::data::conversion::cast::tests
//
// TOC
// - split_join_le
// - split_join_be

use super::Casting;

#[test]
fn split_join_le() {
    let v = u16::MAX / 2;
    let split = Casting(v).into_u8_le();
    assert_eq![v, Casting::<u16>::from_u8_le(split)];

    let v = u32::MAX / 2;
    let split = Casting(v).into_u16_le();
    assert_eq![v, Casting::<u32>::from_u16_le(split)];
    let split = Casting(v).into_u8_le();
    assert_eq![v, Casting::<u32>::from_u8_le(split)];

    let v = u64::MAX / 2;
    let split = Casting(v).into_u32_le();
    assert_eq![v, Casting::<u64>::from_u32_le(split)];
    let split = Casting(v).into_u16_le();
    assert_eq![v, Casting::<u64>::from_u16_le(split)];
    let split = Casting(v).into_u8_le();
    assert_eq![v, Casting::<u64>::from_u8_le(split)];

    let v = u128::MAX / 2;
    let split = Casting(v).into_u64_le();
    assert_eq![v, Casting::<u128>::from_u64_le(split)];
    let split = Casting(v).into_u32_le();
    assert_eq![v, Casting::<u128>::from_u32_le(split)];
    let split = Casting(v).into_u16_le();
    assert_eq![v, Casting::<u128>::from_u16_le(split)];
    let split = Casting(v).into_u8_le();
    assert_eq![v, Casting::<u128>::from_u8_le(split)];
}

#[test]
fn split_join_be() {
    let v = u16::MAX / 2;
    let split = Casting(v).into_u8_be();
    assert_eq![v, Casting::<u16>::from_u8_be(split)];

    let v = u32::MAX / 2;
    let split = Casting(v).into_u16_be();
    assert_eq![v, Casting::<u32>::from_u16_be(split)];
    let split = Casting(v).into_u8_be();
    assert_eq![v, Casting::<u32>::from_u8_be(split)];

    let v = u64::MAX / 2;
    let split = Casting(v).into_u32_be();
    assert_eq![v, Casting::<u64>::from_u32_be(split)];
    let split = Casting(v).into_u16_be();
    assert_eq![v, Casting::<u64>::from_u16_be(split)];
    let split = Casting(v).into_u8_be();
    assert_eq![v, Casting::<u64>::from_u8_be(split)];

    let v = u128::MAX / 2;
    let split = Casting(v).into_u64_be();
    assert_eq![v, Casting::<u128>::from_u64_be(split)];
    let split = Casting(v).into_u32_be();
    assert_eq![v, Casting::<u128>::from_u32_be(split)];
    let split = Casting(v).into_u16_be();
    assert_eq![v, Casting::<u128>::from_u16_be(split)];
    let split = Casting(v).into_u8_be();
    assert_eq![v, Casting::<u128>::from_u8_be(split)];
}
