// devela/src/num/quant/ratio/_test.rs

use super::*;
use crate::{
    IntError,
    Ordering::{Equal, Less},
};

mod construct {
    use super::*;

    #[test]
    fn i32_constructs_and_accesses() {
        let r = RatioI32::new(3, 2).unwrap();
        assert_eq!(r.num(), 3);
        assert_eq!(r.den(), 2);
        assert_eq!(r.num_den(), (3, 2));
        assert_eq!(r.into_parts().0, 3);
        assert!(RatioI32::new(1, 0).is_none());
    }
    #[test]
    fn u32_constructs_and_accesses() {
        let r = RatioU32::new(16, 9).unwrap();
        assert_eq!(r.num(), 16);
        assert_eq!(r.den(), 9);
        assert_eq!(r.num_den(), (16, 9));
        assert!(RatioU32::new(1, 0).is_none());
    }
    #[test]
    fn try_new_reports_zero_denominator() {
        assert_eq!(RatioI32::try_new(1, 0), Err(IntError::NonZeroRequired));
    }
    #[test]
    fn constants_and_default() {
        assert_eq!(RatioI32::ZERO.num_den(), (0, 1));
        assert_eq!(RatioI32::ONE.num_den(), (1, 1));
        assert_eq!(RatioI32::NEG_ONE.num_den(), (-1, 1));
        assert_eq!(RatioU32::default().num_den(), (0, 1));
    }
}

mod query {
    use super::*;

    // #[test]
    // fn signs() { ... }
    #[test]
    fn is_integer_handles_min_div_neg_one() {
        assert!(RatioI8::new(i8::MIN, -1).unwrap().is_integer());
        assert!(RatioI8::new(i8::MIN, 2).unwrap().is_integer());
        assert!(!RatioI8::new(7, 3).unwrap().is_integer());
    }
    // #[test]
    // fn proper_and_reduced() { ... }
    #[test]
    fn cmp_value_is_semantic_not_structural() {
        let a = RatioI32::new(1, 2).unwrap();
        let b = RatioI32::new(2, 4).unwrap();
        assert_ne!(a, b);
        assert_eq!(a.cmp_value(b), Ok(Equal));
    }
    #[test]
    fn cmp_value_handles_negative_denominators() {
        let a = RatioI32::new(1, -2).unwrap(); // -0.5
        let b = RatioI32::new(1, 3).unwrap(); //  0.333...
        assert_eq!(a.cmp_value(b), Ok(Less));
    }
}

mod modify {
    use super::*;

    #[test]
    fn recip() {
        assert_eq!(RatioI32::new(3, 2).unwrap().recip().unwrap().num_den(), (2, 3));
        assert_eq!(RatioI32::new(-3, 2).unwrap().recip().unwrap().num_den(), (2, -3));
        assert!(RatioI32::new(0, 2).unwrap().recip().is_err());
    }
    #[test]
    fn recip_reports_no_inverse() {
        assert_eq!(RatioI32::new(0, 2).unwrap().recip(), Err(IntError::NoInverse));
    }
    #[test]
    fn reduced_preserves_den_sign() {
        assert_eq!(RatioI32::new(6, 8).unwrap().reduced().unwrap().num_den(), (3, 4));
        assert_eq!(RatioI32::new(6, -8).unwrap().reduced().unwrap().num_den(), (3, -4));
        assert_eq!(RatioI32::new(-6, -8).unwrap().reduced().unwrap().num_den(), (-3, -4));
        assert_eq!(RatioI32::new(0, 8).unwrap().reduced().unwrap().num_den(), (0, 1));
    }
    #[test]
    fn with_pos_den_moves_sign() {
        assert_eq!(RatioI32::new(1, 2).unwrap().with_pos_den().unwrap().num_den(), (1, 2));
        assert_eq!(RatioI32::new(1, -2).unwrap().with_pos_den().unwrap().num_den(), (-1, 2));
        assert_eq!(RatioI32::new(-1, -2).unwrap().with_pos_den().unwrap().num_den(), (1, 2));
        assert_eq!(RatioI32::new(0, -2).unwrap().with_pos_den().unwrap().num_den(), (0, 2));
    }
    #[test]
    fn reduced_pos_den() {
        assert_eq!(RatioI32::new(6, 8).unwrap().reduced_pos_den().unwrap().num_den(), (3, 4));
        assert_eq!(RatioI32::new(6, -8).unwrap().reduced_pos_den().unwrap().num_den(), (-3, 4));
        assert_eq!(RatioI32::new(-6, -8).unwrap().reduced_pos_den().unwrap().num_den(), (3, 4));
    }
    #[test]
    fn min_signed_edges_are_checked() {
        assert!(RatioI8::new(i8::MIN, 1).unwrap().gcd().is_err());
        assert!(RatioI8::new(1, i8::MIN).unwrap().gcd().is_err());
        assert!(RatioI8::new(1, i8::MIN).unwrap().with_pos_den().is_err());
        assert!(RatioI8::new(i8::MIN, -1).unwrap().with_pos_den().is_err());
    }
    #[test]
    fn or_self_variants_preserve_on_error() {
        let r = RatioI8::new(i8::MIN, 1).unwrap();
        assert_eq!(r.reduced_or_self().num_den(), (i8::MIN, 1));
    }
}

mod arithmetic {
    use super::*;

    #[test]
    fn gcd_lcm() {
        let r = RatioI32::new(6, -8).unwrap();
        assert_eq!(r.gcd().unwrap(), 2);
        assert_eq!(r.lcm().unwrap(), 24);
        assert_eq!(RatioI32::new(0, 8).unwrap().gcd().unwrap(), 8);
        assert_eq!(RatioI32::new(0, 8).unwrap().lcm().unwrap(), 0);
    }
    #[test]
    fn lcm_overflow_is_err() {
        let r = RatioU8::new(250, 249).unwrap();
        assert!(r.lcm().is_err());
    }
    #[test]
    fn compose_raw_preserves_terms() {
        let a = RatioU32::new(2, 3).unwrap();
        let b = RatioU32::new(3, 5).unwrap();
        assert_eq!(a.compose_raw(b).unwrap().num_den(), (6, 15));
    }
    #[test]
    fn compose_cross_reduces() {
        let a = RatioU32::new(2, 3).unwrap();
        let b = RatioU32::new(3, 5).unwrap();
        assert_eq!(a.compose(b).unwrap().num_den(), (2, 5));
    }
    #[test]
    fn compose_avoids_raw_overflow_when_cross_reducible() {
        let a = RatioU8::new(200, 3).unwrap();
        let b = RatioU8::new(3, 2).unwrap();
        assert!(a.compose_raw(b).is_err());
        assert_eq!(a.compose(b).unwrap().num_den(), (100, 1));
    }
}
mod apply {
    use super::*;

    #[test]
    fn apply_unsigned() {
        let r = RatioU32::new(3, 2).unwrap();
        assert_eq!(r.apply_floor(5), Ok(7)); // 7.5
        assert_eq!(r.apply_ceil(5), Ok(8));
        assert_eq!(r.apply_round(5), Ok(8));
    }
    #[test]
    fn apply_signed_floor_ceil() {
        let r = RatioI32::new(3, 2).unwrap();
        assert_eq!(r.apply_floor(5), Ok(7)); //  7.5 -> 7
        assert_eq!(r.apply_ceil(5), Ok(8)); //  7.5 -> 8
        assert_eq!(r.apply_floor(-5), Ok(-8)); // -7.5 -> -8
        assert_eq!(r.apply_ceil(-5), Ok(-7)); // -7.5 -> -7
    }
    #[test]
    fn apply_signed_negative_den() {
        let r = RatioI32::new(3, -2).unwrap();
        assert_eq!(r.apply_floor(5), Ok(-8)); // -7.5 -> -8
        assert_eq!(r.apply_ceil(5), Ok(-7)); // -7.5 -> -7
        assert_eq!(r.apply_round(5), Ok(-8)); // ties away
    }
    #[test]
    fn apply_uses_widened_intermediate() {
        let r = RatioI8::new(100, 2).unwrap();
        assert_eq!(r.apply_floor(2), Ok(100));
    }
    #[test]
    fn apply_overflow_is_err() {
        let r = RatioI8::new(127, 1).unwrap();
        assert!(r.apply_floor(2).is_err());
    }
    #[test]
    fn apply_step_rounds_to_step() {
        let r = RatioU32::new(235, 100).unwrap();
        assert_eq!(r.inverse_apply_round(8192), Ok(3486));
        assert_eq!(r.inverse_apply_round_step(1920, 8), Ok(816));
        assert_eq!(r.inverse_apply_floor_step(1920, 8), Ok(816));
        assert_eq!(r.inverse_apply_ceil_step(1920, 8), Ok(824));
    }
    #[test]
    fn apply_step_rejects_zero_step() {
        let r = RatioU32::new(3, 2).unwrap();
        assert_eq!(r.apply_round_step(5, 0), Err(IntError::NonZeroRequired));
    }
    #[test]
    fn apply_step_rejects_negative_step() {
        let r = RatioI32::new(3, 2).unwrap();
        assert_eq!(r.apply_round_step(5, -8), Err(IntError::NonNegativeRequired));
    }
    #[test]
    fn inverse_apply_reports_no_inverse() {
        let r = RatioI32::new(0, 1).unwrap();
        assert_eq!(r.inverse_apply_floor(10), Err(IntError::NoInverse));
    }
    #[test]
    fn projection_height_for_width() {
        let r = RatioU32::new(235, 100).unwrap(); // 2.35:1
        assert_eq!(r.inverse_apply_round(8192), Ok(3486));
        assert_eq!(r.inverse_apply_round_step(1920, 8), Ok(816));
    }
    #[test]
    fn with_fixed_num_or_den() {
        let r = RatioU32::new(235, 100).unwrap();
        assert_eq!(r.with_num_round(8192).unwrap().num_den(), (8192, 3486));
        assert_eq!(r.with_den_round(816).unwrap().den(), 816);
    }
}

mod format {
    use super::*;

    #[test]
    fn write_with_sep() {
        let r = RatioI32::new(-4, 3).unwrap();
        let mut buf = [0u8; 8];
        let len = r.write_with_sep(&mut buf, ':').unwrap();
        assert_eq!(&buf[..len], b"-4:3");
    }
    // #[test]
    // fn write_with_sep_rejects_short_buffer() { ... }
    #[test]
    fn write_with_sep_rejects_short_buffer() {
        let r = RatioU32::new(16, 9).unwrap();
        let mut buf = [0u8; 3];
        assert_eq!(r.write_with_sep(&mut buf, ':'), Err(IntError::MismatchedSizes));
    }
    #[test]
    fn write_str_with_utf8_sep() {
        let r = RatioU32::new(16, 9).unwrap();
        let mut buf = [0u8; 16];
        assert_eq!(r.write_str_with_sep(&mut buf, '∶').unwrap(), "16∶9");
    }
    #[test]
    #[cfg(feature = "alloc")]
    fn display_uses_colon() {
        use crate::ToString;
        assert_eq!(RatioU32::new(16, 9).unwrap().to_string(), "16:9");
        assert_eq!(RatioI32::new(-4, 3).unwrap().to_string(), "-4:3");
    }
    #[test]
    fn write_signed_min_value() {
        let r = RatioI8::new(i8::MIN, 1).unwrap();
        let mut buf = [0u8; RatioI8::STR_BUF_LEN];
        assert_eq!(r.write_str_with_sep(&mut buf, ':').unwrap(), "-128:1");
    }
}
