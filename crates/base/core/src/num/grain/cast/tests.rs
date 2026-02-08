// devela_base_core::num::grain::cast::tests
//
// TOC
// - split_join_le
// - split_join_be

use super::Cast;

#[test]
fn saturating_cast() {
    // Most extensive testing: i32 to u8 (most delicate case)
    assert_eq!(Cast(256_i32).saturating_cast_to_u8(), 255_u8); // Saturate at max
    assert_eq!(Cast(255_i32).saturating_cast_to_u8(), 255_u8); // Max fits
    assert_eq!(Cast(257_i32).saturating_cast_to_u8(), 255_u8); // Saturate at max
    assert_eq!(Cast(511_i32).saturating_cast_to_u8(), 255_u8); // Saturate at max
    assert_eq!(Cast(512_i32).saturating_cast_to_u8(), 255_u8); // Saturate at max
    assert_eq!(Cast(-1_i32).saturating_cast_to_u8(), 0_u8); // Saturate at min
    assert_eq!(Cast(-256_i32).saturating_cast_to_u8(), 0_u8); // Saturate at min
    assert_eq!(Cast(-257_i32).saturating_cast_to_u8(), 0_u8); // Saturate at min
    assert_eq!(Cast(i32::MAX).saturating_cast_to_u8(), 255_u8); // Saturate at max
    assert_eq!(Cast(i32::MIN).saturating_cast_to_u8(), 0_u8); // Saturate at min
    assert_eq!(Cast(128_i32).saturating_cast_to_u8(), 128_u8); // Within range
    assert_eq!(Cast(127_i32).saturating_cast_to_u8(), 127_u8); // Within range

    // i32 to i8 (signed to signed with different ranges)
    assert_eq!(Cast(127_i32).saturating_cast_to_i8(), 127_i8); // Max positive
    assert_eq!(Cast(128_i32).saturating_cast_to_i8(), 127_i8); // Saturate at max
    assert_eq!(Cast(255_i32).saturating_cast_to_i8(), 127_i8); // Saturate at max
    assert_eq!(Cast(256_i32).saturating_cast_to_i8(), 127_i8); // Saturate at max
    assert_eq!(Cast(-129_i32).saturating_cast_to_i8(), -128_i8); // Saturate at min
    assert_eq!(Cast(i32::MAX).saturating_cast_to_i8(), 127_i8); // Saturate at max
    assert_eq!(Cast(i32::MIN).saturating_cast_to_i8(), -128_i8); // Saturate at min

    // u32 to i32 (unsigned to signed - saturate at signed bounds)
    assert_eq!(Cast(i32::MAX as u32).saturating_cast_to_i32(), i32::MAX);
    assert_eq!(Cast(i32::MAX as u32 + 1).saturating_cast_to_i32(), i32::MAX); // Saturate at max
    assert_eq!(Cast(u32::MAX).saturating_cast_to_i32(), i32::MAX); // Saturate at max

    // u32 to u8 (unsigned to smaller unsigned)
    assert_eq!(Cast(255_u32).saturating_cast_to_u8(), 255_u8); // Max fits
    assert_eq!(Cast(256_u32).saturating_cast_to_u8(), 255_u8); // Saturate at max
    assert_eq!(Cast(u32::MAX).saturating_cast_to_u8(), 255_u8); // Saturate at max

    // i32 to u32 (signed to larger unsigned - saturate at 0 for negatives)
    assert_eq!(Cast(100_i32).saturating_cast_to_u32(), 100_u32); // Positive stays
    assert_eq!(Cast(-1_i32).saturating_cast_to_u32(), 0_u32); // Saturate at min (0)
    assert_eq!(Cast(i32::MIN).saturating_cast_to_u32(), 0_u32); // Saturate at min (0)
    assert_eq!(Cast(i32::MAX).saturating_cast_to_u32(), i32::MAX as u32); // Within range

    // u8 to i32 (small unsigned to larger signed - no saturation needed)
    assert_eq!(Cast(255_u8).saturating_cast_to_i32(), 255_i32); // Within range
    assert_eq!(Cast(0_u8).saturating_cast_to_i32(), 0_i32); // Within range

    // u8 to i8 (unsigned to same-size signed - saturate at signed max)
    assert_eq!(Cast(127_u8).saturating_cast_to_i8(), 127_i8); // Max positive fits
    assert_eq!(Cast(128_u8).saturating_cast_to_i8(), 127_i8); // Saturate at max
    assert_eq!(Cast(255_u8).saturating_cast_to_i8(), 127_i8); // Saturate at max

    // Edge cases with zero
    assert_eq!(Cast(0_i32).saturating_cast_to_u8(), 0_u8);
    assert_eq!(Cast(0_u8).saturating_cast_to_i32(), 0_i32);
    assert_eq!(Cast(0_i32).saturating_cast_to_i8(), 0_i8);

    // Extreme values
    assert_eq!(Cast(i64::MAX).saturating_cast_to_u8(), 255_u8); // Saturate at max
    assert_eq!(Cast(i64::MIN).saturating_cast_to_u8(), 0_u8); // Saturate at min
    assert_eq!(Cast(u64::MAX).saturating_cast_to_i8(), 127_i8); // Saturate at max

    // Special: Positive values that should NOT saturate
    assert_eq!(Cast(42_i32).saturating_cast_to_u8(), 42_u8);
    assert_eq!(Cast(100_u32).saturating_cast_to_i32(), 100_i32);
    assert_eq!(Cast(50_i16).saturating_cast_to_u16(), 50_u16);

    // Special: Negative values to unsigned always saturate to 0
    assert_eq!(Cast(-5_i32).saturating_cast_to_u16(), 0_u16);
    assert_eq!(Cast(-100_i64).saturating_cast_to_u32(), 0_u32);
}

#[test]
fn wrapping_cast() {
    // Most extensive testing: i32 to u8 (most delicate case)
    assert_eq!(Cast(256_i32).wrapping_cast_to_u8(), 0_u8); // Exactly one overflow
    assert_eq!(Cast(255_i32).wrapping_cast_to_u8(), 255_u8); // Max fits
    assert_eq!(Cast(257_i32).wrapping_cast_to_u8(), 1_u8); // One over + 1
    assert_eq!(Cast(511_i32).wrapping_cast_to_u8(), 255_u8); // Double overflow - 1
    assert_eq!(Cast(512_i32).wrapping_cast_to_u8(), 0_u8); // Double overflow
    assert_eq!(Cast(-1_i32).wrapping_cast_to_u8(), 255_u8); // Negative to unsigned
    assert_eq!(Cast(-256_i32).wrapping_cast_to_u8(), 0_u8); // Negative exact multiple
    assert_eq!(Cast(-257_i32).wrapping_cast_to_u8(), 255_u8); // Negative overflow
    assert_eq!(Cast(i32::MAX).wrapping_cast_to_u8(), 255_u8); // Max i32 to u8
    assert_eq!(Cast(i32::MIN).wrapping_cast_to_u8(), 0_u8); // Min i32 to u8
    assert_eq!(Cast(128_i32).wrapping_cast_to_u8(), 128_u8); // Boundary
    assert_eq!(Cast(127_i32).wrapping_cast_to_u8(), 127_u8); // Boundary

    // i32 to i8 (signed to signed with different ranges)
    assert_eq!(Cast(127_i32).wrapping_cast_to_i8(), 127_i8); // Max positive
    assert_eq!(Cast(128_i32).wrapping_cast_to_i8(), -128_i8); // Overflow to negative
    assert_eq!(Cast(255_i32).wrapping_cast_to_i8(), -1_i8); // Wrapping
    assert_eq!(Cast(256_i32).wrapping_cast_to_i8(), 0_i8); // Wrapping
    assert_eq!(Cast(-129_i32).wrapping_cast_to_i8(), 127_i8); // Negative overflow
    assert_eq!(Cast(i32::MAX).wrapping_cast_to_i8(), -1_i8); // Max to min

    // u32 to i32 (unsigned to signed, usually safe but test boundaries)
    assert_eq!(Cast(i32::MAX as u32).wrapping_cast_to_i32(), i32::MAX);
    assert_eq!(Cast(i32::MAX as u32 + 1).wrapping_cast_to_i32(), i32::MIN);
    assert_eq!(Cast(u32::MAX).wrapping_cast_to_i32(), -1_i32); // All bits set

    // u32 to u8 (unsigned to smaller unsigned)
    assert_eq!(Cast(255_u32).wrapping_cast_to_u8(), 255_u8); // Max fits
    assert_eq!(Cast(256_u32).wrapping_cast_to_u8(), 0_u8); // Simple overflow
    assert_eq!(Cast(u32::MAX).wrapping_cast_to_u8(), 255_u8); // Max to max

    // i32 to u32 (signed to larger unsigned - special attention to negatives)
    assert_eq!(Cast(100_i32).wrapping_cast_to_u32(), 100_u32); // Positive stays
    assert_eq!(Cast(-1_i32).wrapping_cast_to_u32(), u32::MAX); // Negative to max
    assert_eq!(Cast(i32::MIN).wrapping_cast_to_u32(), 2147483648_u32); // Min to large

    // u8 to i32 (small unsigned to larger signed - usually safe)
    assert_eq!(Cast(255_u8).wrapping_cast_to_i32(), 255_i32); // Max u8 stays
    assert_eq!(Cast(0_u8).wrapping_cast_to_i32(), 0_i32); // Zero

    // u8 to i8 (unsigned to same-size signed)
    assert_eq!(Cast(127_u8).wrapping_cast_to_i8(), 127_i8); // Positive fits
    assert_eq!(Cast(128_u8).wrapping_cast_to_i8(), -128_i8); // Becomes negative
    assert_eq!(Cast(255_u8).wrapping_cast_to_i8(), -1_i8); // Max to -1

    // Edge cases with zero
    assert_eq!(Cast(0_i32).wrapping_cast_to_u8(), 0_u8);
    assert_eq!(Cast(0_u8).wrapping_cast_to_i32(), 0_i32);
    assert_eq!(Cast(0_i32).wrapping_cast_to_i8(), 0_i8);

    // Extreme values
    assert_eq!(Cast(i64::MAX).wrapping_cast_to_u8(), 255_u8);
    assert_eq!(Cast(i64::MIN).wrapping_cast_to_u8(), 0_u8);
    assert_eq!(Cast(u64::MAX).wrapping_cast_to_i8(), -1_i8);
}

#[test]
fn split_join_le() {
    let v = u16::MAX / 2;
    let split = Cast(v).into_u8_le();
    assert_eq![v, Cast::<u16>::from_u8_le(split)];

    let v = u32::MAX / 2;
    let split = Cast(v).into_u16_le();
    assert_eq![v, Cast::<u32>::from_u16_le(split)];
    let split = Cast(v).into_u8_le();
    assert_eq![v, Cast::<u32>::from_u8_le(split)];

    let v = u64::MAX / 2;
    let split = Cast(v).into_u32_le();
    assert_eq![v, Cast::<u64>::from_u32_le(split)];
    let split = Cast(v).into_u16_le();
    assert_eq![v, Cast::<u64>::from_u16_le(split)];
    let split = Cast(v).into_u8_le();
    assert_eq![v, Cast::<u64>::from_u8_le(split)];

    let v = u128::MAX / 2;
    let split = Cast(v).into_u64_le();
    assert_eq![v, Cast::<u128>::from_u64_le(split)];
    let split = Cast(v).into_u32_le();
    assert_eq![v, Cast::<u128>::from_u32_le(split)];
    let split = Cast(v).into_u16_le();
    assert_eq![v, Cast::<u128>::from_u16_le(split)];
    let split = Cast(v).into_u8_le();
    assert_eq![v, Cast::<u128>::from_u8_le(split)];
}

#[test]
fn split_join_be() {
    let v = u16::MAX / 2;
    let split = Cast(v).into_u8_be();
    assert_eq![v, Cast::<u16>::from_u8_be(split)];

    let v = u32::MAX / 2;
    let split = Cast(v).into_u16_be();
    assert_eq![v, Cast::<u32>::from_u16_be(split)];
    let split = Cast(v).into_u8_be();
    assert_eq![v, Cast::<u32>::from_u8_be(split)];

    let v = u64::MAX / 2;
    let split = Cast(v).into_u32_be();
    assert_eq![v, Cast::<u64>::from_u32_be(split)];
    let split = Cast(v).into_u16_be();
    assert_eq![v, Cast::<u64>::from_u16_be(split)];
    let split = Cast(v).into_u8_be();
    assert_eq![v, Cast::<u64>::from_u8_be(split)];

    let v = u128::MAX / 2;
    let split = Cast(v).into_u64_be();
    assert_eq![v, Cast::<u128>::from_u64_be(split)];
    let split = Cast(v).into_u32_be();
    assert_eq![v, Cast::<u128>::from_u32_be(split)];
    let split = Cast(v).into_u16_be();
    assert_eq![v, Cast::<u128>::from_u16_be(split)];
    let split = Cast(v).into_u8_be();
    assert_eq![v, Cast::<u128>::from_u8_be(split)];
}
