// devela/src/num/quant/scale/_test.rs

use crate::Scale;

mod int {
    use super::*;
    use crate::{
        IntError::{NonZeroRequired, Overflow},
        IntResult as Result,
        Sign::{Negative, Positive},
    };

    const CONST_ROUND: Result<i32> = Scale(5_i32).mul_div_round(3, 2);
    const CONST_FLOOR: Result<i32> = Scale(-5_i32).mul_div_floor(3, 2);

    #[test]
    fn is_const() {
        assert_eq!(CONST_ROUND, Ok(8));
        assert_eq!(CONST_FLOOR, Ok(-8));
    }
    #[test]
    fn signed_rounding_modes() {
        assert_eq!(Scale(5_i32).mul_div_trunc(3, 2), Ok(7));
        assert_eq!(Scale(5_i32).mul_div_floor(3, 2), Ok(7));
        assert_eq!(Scale(5_i32).mul_div_ceil(3, 2), Ok(8));
        assert_eq!(Scale(5_i32).mul_div_round(3, 2), Ok(8));
        assert_eq!(Scale(-5_i32).mul_div_trunc(3, 2), Ok(-7));
        assert_eq!(Scale(-5_i32).mul_div_floor(3, 2), Ok(-8));
        assert_eq!(Scale(-5_i32).mul_div_ceil(3, 2), Ok(-7));
        assert_eq!(Scale(-5_i32).mul_div_round(3, 2), Ok(-8));
    }
    #[test]
    fn signed_negative_denominator() {
        assert_eq!(Scale(5_i32).mul_div_trunc(3, -2), Ok(-7));
        assert_eq!(Scale(5_i32).mul_div_floor(3, -2), Ok(-8));
        assert_eq!(Scale(5_i32).mul_div_ceil(3, -2), Ok(-7));
        assert_eq!(Scale(5_i32).mul_div_round(3, -2), Ok(-8));
        assert_eq!(Scale(-5_i32).mul_div_round(3, -2), Ok(8));
    }
    #[test]
    fn unsigned_rounding_modes() {
        assert_eq!(Scale(5_u8).mul_div_trunc(3, 2), Ok(7));
        assert_eq!(Scale(5_u8).mul_div_floor(3, 2), Ok(7));
        assert_eq!(Scale(5_u8).mul_div_ceil(3, 2), Ok(8));
        assert_eq!(Scale(5_u8).mul_div_round(3, 2), Ok(8));
    }
    #[test]
    fn exact_results_agree() {
        assert_eq!(Scale(8_i32).mul_div_trunc(3, 2), Ok(12));
        assert_eq!(Scale(8_i32).mul_div_floor(3, 2), Ok(12));
        assert_eq!(Scale(8_i32).mul_div_ceil(3, 2), Ok(12));
        assert_eq!(Scale(8_i32).mul_div_round(3, 2), Ok(12));
    }
    #[test]
    fn ties_round_away_from_zero() {
        assert_eq!(Scale(1_i32).mul_div_round(1, 2), Ok(1));
        assert_eq!(Scale(-1_i32).mul_div_round(1, 2), Ok(-1));
        assert_eq!(Scale(3_i32).mul_div_round(1, 2), Ok(2));
        assert_eq!(Scale(-3_i32).mul_div_round(1, 2), Ok(-2));
    }
    #[test]
    fn zero_denominator() {
        assert_eq!(Scale(1_i32).mul_div_round(1, 0), Err(NonZeroRequired));
    }
    #[test]
    fn final_range_overflow() {
        assert_eq!(Scale(200_u8).mul_div_trunc(2, 1), Err(Overflow(Some(Positive))));
        assert_eq!(Scale(-100_i8).mul_div_trunc(2, 1), Err(Overflow(Some(Negative))));
    }
    #[test]
    fn signed_division_overflow() {
        assert_eq!(Scale(i128::MIN).mul_div_trunc(1, -1), Err(Overflow(Some(Positive))));
    }
}
