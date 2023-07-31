// devela::primitive
//
//! helpers for primitives.
//

mod join;
mod split;

pub use join::*;
pub use split::*;

#[cfg(test)]
mod tests {
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
}
