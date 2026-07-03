// devela/src/num/grain/lim/_test.rs

use crate::{
    BoundI8Example as I, BoundI8SymExample as Is,
    Boundary1d::{Lower, Upper},
};

const RAW_RESERVED: i8 = i8::MIN;
const fn v(n: i8) -> I {
    I::new(n)
}
fn assert_meta(x: I, count: u8, dir: Option<crate::Boundary1d>) {
    assert_eq![x.bound_count(), count];
    assert_eq![x.bound_dir(), dir];
}
fn assert_empty_meta(x: I) {
    assert_meta(x, 0, None);
}
fn assert_upper(x: I, count: u8) {
    assert_meta(x, count, Some(Upper));
}
fn assert_lower(x: I, count: u8) {
    assert_meta(x, count, Some(Lower));
}

mod contract {
    use super::*;
    #[test]
    fn constants_match_contract() {
        assert_eq![I::CARRIER_BITS, 8];
        assert_eq![I::VALUE_BITS, 5];
        assert_eq![I::META_BITS, 3];
        assert_eq![I::COUNT_BITS, 2];
        assert_eq![I::MAX_COUNT, 3];
        assert_eq![I::MIN_VALUE, -16];
        assert_eq![I::MAX_VALUE, 15];
        assert_eq![I::RESERVED_RAW, RAW_RESERVED];
        assert_eq![I::ZERO.get(), 0];
        assert_eq![I::ZERO.raw(), 0];
        assert_empty_meta(I::ZERO);
    }
    #[test]
    fn checked_constructor_accepts_payload_range() {
        assert_eq![I::new_checked(0).unwrap(), I::ZERO];
        assert_eq![I::new_checked(I::MIN_VALUE).unwrap(), I::MIN];
        assert_eq![I::new_checked(I::MAX_VALUE).unwrap(), I::MAX];
    }
    #[test]
    fn checked_constructor_rejects_outside_payload_range() {
        assert![I::new_checked(I::MIN_VALUE - 1).is_none()];
        assert![I::new_checked(I::MAX_VALUE + 1).is_none()];
    }
    #[test]
    fn saturating_constructor_records_lower_and_upper_events() {
        let lo = I::new(I::MIN_VALUE - 1);
        assert_eq![lo, I::MIN];
        assert_lower(lo, 1);
        let hi = I::new(I::MAX_VALUE + 1);
        assert_eq![hi, I::MAX];
        assert_upper(hi, 1);
    }
    #[test]
    fn raw_reserved_is_rejected() {
        assert![I::from_raw(RAW_RESERVED).is_none()];
    }
    #[test]
    fn raw_count_zero_canonicalizes_direction() {
        let raw = RAW_RESERVED + 7; // count == 0, dir == upper, value == 7
        let x = I::from_raw(raw).unwrap();
        assert_eq![x.get(), 7];
        assert_eq![x.raw(), 7];
        assert_empty_meta(x);
    }
}
mod construction {
    use super::*;
    #[test]
    fn constructors_check_and_saturate() {
        assert_eq![I::new(0).get(), 0];
        assert_eq![I::new_checked(0).unwrap().get(), 0];
        assert_eq![I::new_checked(I::MIN_VALUE).unwrap(), I::MIN];
        assert_eq![I::new_checked(I::MAX_VALUE).unwrap(), I::MAX];
        assert![I::new_checked(I::MIN_VALUE - 1).is_none()];
        assert![I::new_checked(I::MAX_VALUE + 1).is_none()];
        let lo = I::new(I::MIN_VALUE - 1);
        assert_eq![lo.get(), I::MIN_VALUE];
        assert_eq![lo.bound_count(), 1];
        assert_eq![lo.bound_dir(), Some(Lower)];
        let hi = I::new(I::MAX_VALUE + 1);
        assert_eq![hi.get(), I::MAX_VALUE];
        assert_eq![hi.bound_count(), 1];
        assert_eq![hi.bound_dir(), Some(Upper)];
    }
}
mod metadata {
    use super::*;
    #[test]
    fn clear_meta_preserves_value_only() {
        let x = I::MAX.sat_add(v(1));
        assert_upper(x, 1);
        let y = x.clear_meta();
        assert_eq![y, I::MAX];
        assert_empty_meta(y);
    }
    #[test]
    fn with_bound_meta_sets_and_saturates_count() {
        let x = v(7).with_bound_meta(99, Some(Upper));
        assert_eq![x.get(), 7];
        assert_eq![x.bound_count(), I::MAX_COUNT];
        assert_eq![x.bound_dir(), Some(Upper)];
    }
    #[test]
    fn with_bound_meta_canonicalizes_zero_count() {
        let x = v(7).with_bound_meta(0, Some(Upper));
        assert_eq![x.get(), 7];
        assert_empty_meta(x);
        assert_eq![x.raw(), 7];
    }
    #[test]
    fn boundary_count_saturates() {
        let one = v(1);
        let a = I::MAX.sat_add_meta(one);
        let b = a.sat_add_meta(one);
        let c = b.sat_add_meta(one);
        let d = c.sat_add_meta(one);
        assert_eq![I::MAX_COUNT, 3];
        assert_upper(a, 1);
        assert_upper(b, 2);
        assert_upper(c, 3);
        assert_upper(d, 3);
    }
}
mod sign_magnitude {
    /* sign predicates, abs, dist policy */
    use super::*;
    #[test]
    fn sign_predicates_work() {
        assert![v(-1).is_negative()];
        assert![v(1).is_positive()];
        assert![I::ZERO.is_zero()];
        assert![I::ZERO.is_nonnegative()];
        assert![I::ZERO.is_nonpositive()];
        assert![v(1).is_nonnegative()];
        assert![v(-1).is_nonpositive()];
    }
    #[test]
    fn abs_returns_magnitude_without_metadata() {
        let x = v(-7).abs();
        assert_eq![x.get(), 7];
        assert_empty_meta(x);
    }
    #[test]
    fn abs_meta_preserves_existing_metadata_and_records_clip() {
        let bounded = I::MIN.sat_sub(v(1));
        let x = bounded.abs_meta();
        assert_eq![x.get(), 15];
        assert_eq![x.bound_count(), 2];
        assert_eq![x.bound_dir(), Some(Upper)];
    }
    #[test]
    fn abs_nometa_drops_metadata() {
        let bounded = I::MIN.sat_sub(v(1));
        let x = bounded.abs_nometa();
        assert_eq![x.get(), 15];
        assert_empty_meta(x);
    }
    #[test]
    fn abs_saturates_payload_minimum_magnitude() {
        let x = I::MIN.abs_meta();
        assert_eq![x, I::MAX];
        assert_upper(x, 1);
    }
    #[test]
    fn abs_nometa_saturates_without_metadata() {
        let x = I::MIN.abs_nometa();
        assert_eq![x, I::MAX];
        assert_empty_meta(x);
    }
}
mod ordering {
    use super::*;

    #[test]
    fn max_min_preserve_selected_value() {
        let a = I::new(-5);
        let b = I::new(-7);
        assert_eq![a.max_meta(b), a];
        assert_eq![a.min_meta(b), b];
        let bounded = I::MAX.sat_add(I::new(1));
        assert_eq![bounded.max_meta(I::new(0)).bound_count(), 1];
        assert_eq![bounded.max_meta(I::new(0)).bound_dir(), Some(Upper)];
    }
    #[test]
    fn min_max_handle_negative_values() {
        let a = v(-5);
        let b = v(-7);
        assert_eq![a.max(b), a];
        assert_eq![a.min(b), b];
        assert_empty_meta(a.max(b));
        assert_empty_meta(a.min(b));
    }
    #[test]
    fn extrema_clear_metadata() {
        let bounded = I::MAX.sat_add(I::new(1));
        assert_eq![bounded.bound_count(), 1];
        let x = bounded.max(I::new(0));
        assert_eq![x.get(), I::MAX_VALUE];
        assert_eq![x.bound_count(), 0];
        assert_eq![x.bound_dir(), None];
    }
    #[test]
    fn extrema_meta_preserve_selected_operand_metadata() {
        let bounded = I::MAX.sat_add(v(1));
        let x = bounded.max_meta(v(0));
        assert_eq![x, bounded];
        assert_upper(x, 1);
    }
    #[test]
    fn extrema_meta_self_wins_ties() {
        let a = v(3).with_bound_meta(1, Some(Upper));
        let b = v(3).with_bound_meta(1, Some(Lower));
        assert_eq![a.max_meta(b).bound_dir(), Some(Upper)];
        assert_eq![a.min_meta(b).bound_dir(), Some(Upper)];
    }
    #[test]
    fn clamp_handles_negative_values() {
        let x = v(-7);
        let min = v(-5);
        let max = v(3);
        assert_eq![x.clamp(min, max), min];
        assert_empty_meta(x.clamp(min, max));
    }
    #[test]
    fn clamp_meta_selects_operand_metadata() {
        let bounded = I::MAX.sat_add(v(1));
        let a = bounded.clamp(v(0), I::MAX);
        assert_eq![a, I::MAX];
        assert_empty_meta(a);
        let b = bounded.clamp_meta(v(0), I::MAX);
        assert_eq![b, bounded];
        assert_upper(b, 1);
    }
    #[test]
    fn clamp_with_inverted_bounds_returns_min_argument() {
        let x = v(0);
        let min = v(10);
        let max = v(5);
        assert_eq![x.clamp(min, max), min];
    }
    #[test]
    fn clamp_clears_metadata_but_clamp_meta_selects_it() {
        let bounded = I::MAX.sat_add(I::new(1));
        let a = bounded.clamp(I::new(0), I::MAX);
        assert_eq![a, I::MAX];
        assert_eq![a.bound_count(), 0];
        let b = bounded.clamp_meta(I::new(0), I::MAX);
        assert_eq![b, bounded];
        assert_eq![b.bound_count(), 1];
    }
    #[test]
    fn zero_projections_follow_metadata_policy() {
        let upper = I::MAX.sat_add(v(1));
        let lower = I::MIN.sat_sub(v(1));
        assert_eq![upper.max_zero(), I::MAX];
        assert_empty_meta(upper.max_zero());
        assert_eq![upper.max_zero_meta(), upper];
        assert_upper(upper.max_zero_meta(), 1);
        assert_eq![lower.max_zero_meta(), I::ZERO];
        assert_empty_meta(lower.max_zero_meta());
    }
}
mod saturating {
    use super::*;
    #[test]
    fn sat_add_records_upper_boundary() {
        let x = I::MAX.sat_add(v(1));
        assert_eq![x, I::MAX];
        assert_upper(x, 1);
    }
    #[test]
    fn sat_sub_records_lower_boundary() {
        let x = I::MIN.sat_sub(v(1));
        assert_eq![x, I::MIN];
        assert_lower(x, 1);
    }
    #[test]
    fn sat_add_ignores_inherited_metadata() {
        let bounded = I::MAX.sat_add(v(1));
        let x = bounded.sat_add(v(-1));
        assert_eq![x.get(), I::MAX_VALUE - 1];
        assert_empty_meta(x);
    }
    #[test]
    fn sat_add_meta_preserves_inherited_metadata() {
        let bounded = I::MAX.sat_add(v(1));
        let x = bounded.sat_add_meta(v(-1));
        assert_eq![x.get(), I::MAX_VALUE - 1];
        assert_upper(x, 1);
    }
    #[test]
    fn sat_add_tracks_upper_boundary() {
        let one = I::new(1);
        let a = I::MAX.sat_add(one);
        assert_eq![a.get(), I::MAX_VALUE];
        assert_eq![a.bound_count(), 1];
        assert_eq![a.bound_dir(), Some(Upper)];
        let b = a.sat_add_meta(one);
        assert_eq![b.get(), I::MAX_VALUE];
        assert_eq![b.bound_count(), 2];
        assert_eq![b.bound_dir(), Some(Upper)];
    }
    #[test]
    fn sat_sub_tracks_lower_boundary() {
        let one = I::new(1);
        let a = I::MIN.sat_sub(one);
        assert_eq![a.get(), I::MIN_VALUE];
        assert_eq![a.bound_count(), 1];
        assert_eq![a.bound_dir(), Some(Lower)];
        let b = a.sat_sub_meta(one);
        assert_eq![b.get(), I::MIN_VALUE];
        assert_eq![b.bound_count(), 2];
        assert_eq![b.bound_dir(), Some(Lower)];
    }
    #[test]
    fn sat_add_does_not_preserve_inherited_metadata() {
        let one = I::new(1);
        let bounded = I::MAX.sat_add(one);
        assert_eq![bounded.bound_count(), 1];
        let x = bounded.sat_add(I::new(-1));
        assert_eq![x.get(), I::MAX_VALUE - 1];
        assert_eq![x.bound_count(), 0];
        assert_eq![x.bound_dir(), None];
    }
    #[test]
    fn sat_add_nometa_generates_no_metadata() {
        let x = I::MAX.sat_add_nometa(v(1));
        assert_eq![x, I::MAX];
        assert_empty_meta(x);
    }
    #[test]
    fn sat_mul_records_boundary() {
        let x = v(5).sat_mul(v(5));
        assert_eq![x, I::MAX];
        assert_upper(x, 1);
    }
    #[test]
    fn sat_mul_nometa_generates_no_metadata() {
        let x = v(5).sat_mul_nometa(v(5));
        assert_eq![x, I::MAX];
        assert_empty_meta(x);
    }
    #[test]
    fn sat_mul_div_rejects_zero_denominator() {
        assert![v(4).sat_mul_div(2, 0).is_none()];
    }
    #[test]
    fn sat_mul_div_records_boundary() {
        let x = v(10).sat_mul_div(2, 1).unwrap();
        assert_eq![x, I::MAX];
        assert_upper(x, 1);
    }
    #[test]
    fn zero_floor_drops_lower_result_metadata_when_zero_wins() {
        let x = I::MIN.sat_sub_floor_zero(v(1));
        assert_eq![x, I::ZERO];
        assert_empty_meta(x);
    }
    #[test]
    fn zero_floor_keeps_upper_result_metadata_when_saturated_value_wins() {
        let x = I::MAX.sat_add_floor_zero(v(1));
        assert_eq![x, I::MAX];
        assert_upper(x, 1);
    }
    #[test]
    fn zero_ceiling_drops_upper_result_metadata_when_zero_wins() {
        let x = I::MAX.sat_add_ceil_zero(v(1));
        assert_eq![x, I::ZERO];
        assert_empty_meta(x);
    }
    #[test]
    fn zero_ceiling_keeps_lower_result_metadata_when_saturated_value_wins() {
        let x = I::MIN.sat_sub_ceil_zero(v(1));
        assert_eq![x, I::MIN];
        assert_lower(x, 1);
    }
    #[test]
    fn sat_dist_saturates_large_distance() {
        let x = I::MIN.sat_dist(I::MAX);
        assert_eq![x, I::MAX];
        assert_upper(x, 1);
    }
    #[test]
    fn sat_dist_nometa_generates_no_metadata() {
        let x = I::MIN.sat_dist_nometa(I::MAX);
        assert_eq![x, I::MAX];
        assert_empty_meta(x);
    }
    #[test]
    fn sat_dist_meta_preserves_inherited_metadata_when_result_fits() {
        let bounded = I::MAX.sat_add(v(1));
        let x = bounded.sat_dist_meta(v(10));
        assert_eq![x.get(), 5];
        assert_upper(x, 1);
    }
}
mod checked {
    use super::*;
    #[test]
    fn che_add_sub_return_none_on_escape() {
        assert![I::MAX.che_add(v(1)).is_none()];
        assert![I::MIN.che_sub(v(1)).is_none()];
    }
    #[test]
    fn che_add_sub_return_value_when_fitting() {
        assert_eq![v(4).che_add(v(3)).unwrap(), v(7)];
        assert_eq![v(4).che_sub(v(5)).unwrap(), v(-1)];
    }
    #[test]
    fn che_ops_ignore_inherited_metadata_by_default() {
        let bounded = I::MAX.sat_add(v(1));
        let x = bounded.che_sub(v(1)).unwrap();
        assert_eq![x.get(), I::MAX_VALUE - 1];
        assert_empty_meta(x);
    }
    #[test]
    fn che_ops_meta_preserve_existing_metadata() {
        let bounded = I::MAX.sat_add(v(1));
        let x = bounded.che_sub_meta(v(1)).unwrap();
        assert_eq![x.get(), I::MAX_VALUE - 1];
        assert_upper(x, 1);
        let y = bounded.che_mul_meta(v(1)).unwrap();
        assert_eq![y.get(), I::MAX_VALUE];
        assert_upper(y, 1);
    }
    #[test]
    fn che_mul_div_rejects_zero_denominator() {
        assert![v(4).che_mul_div(2, 0).is_none()];
    }
    #[test]
    fn che_dist_returns_none_when_distance_exceeds_payload_range() {
        assert![I::MIN.che_dist(I::MAX).is_none()];
    }
    #[test]
    fn che_dist_meta_preserves_metadata_when_distance_fits() {
        let bounded = I::MAX.sat_add(v(1));
        let x = bounded.che_dist_meta(v(10)).unwrap();
        assert_eq![x.get(), 5];
        assert_upper(x, 1);
    }
}
mod modular {
    use super::*;
    #[test]
    fn mod_add_wraps_upward() {
        let x = v(5).mod_add(v(2), v(3)).unwrap();
        assert_eq![x.get(), 1];
        assert_upper(x, 2);
    }
    #[test]
    fn mod_add_wraps_downward() {
        let x = v(1).mod_add(v(-3), v(7)).unwrap();
        assert_eq![x.get(), 5];
        assert_lower(x, 1);
    }
    #[test]
    fn mod_add_without_wrap_has_no_boundary_event() {
        let x = v(1).mod_add(v(3), v(10)).unwrap();
        assert_eq![x.get(), 4];
        assert_empty_meta(x);
    }
    #[test]
    fn mod_add_rejects_non_positive_modulo() {
        let one = I::new(1);
        assert![one.mod_add(one, I::ZERO).is_none()];
        assert![one.mod_add(one, I::new(-10)).is_none()];
    }
    #[test]
    fn mod_sub_wraps_downward() {
        let x = v(1).mod_sub(v(3), v(7)).unwrap();
        assert_eq![x.get(), 5];
        assert_lower(x, 1);
    }
    #[test]
    fn mod_sub_wraps_upward() {
        let x = v(5).mod_sub(v(-3), v(7)).unwrap();
        assert_eq![x.get(), 1];
        assert_upper(x, 1);
    }
    #[test]
    fn mod_ops_reject_non_positive_modulo() {
        assert![v(1).mod_add(v(1), I::ZERO).is_none()];
        assert![v(1).mod_add(v(1), v(-10)).is_none()];
        assert![v(1).mod_sub(v(1), I::ZERO).is_none()];
        assert![v(1).mod_sub(v(1), v(-10)).is_none()];
    }
}
mod upcasted {
    use super::*;
    #[test]
    fn upcasted_arithmetic_returns_exact_results() {
        assert_eq![v(10).add_up(v(10)), 20];
        assert_eq![v(-10).sub_up(v(10)), -20];
        assert_eq![v(10).mul_up(v(10)), 100];
    }
    #[test]
    fn dist_up_returns_exact_distance() {
        assert_eq![I::MIN.dist_up(I::MAX), 31];
    }
}
mod symmetric {
    use super::*;
    #[test]
    fn constants_match_contract() {
        assert_eq![Is::CARRIER_BITS, 8];
        assert_eq![Is::VALUE_BITS, 5];
        assert_eq![Is::META_BITS, 3];
        assert_eq![Is::COUNT_BITS, 2];
        assert![Is::IS_SYMMETRIC];
        assert_eq![Is::MIN_VALUE, -15];
        assert_eq![Is::MAX_VALUE, 15];
    }
    #[test]
    fn constructor_rejects_negative_extra_endpoint() {
        assert![Is::new_checked(-16).is_none()];
        let x = Is::new(-16);
        assert_eq![x, Is::MIN];
        assert_eq![x.bound_count(), 1];
        assert_eq![x.bound_dir(), Some(Lower)];
    }
    #[test]
    fn abs_of_min_does_not_clip() {
        let x = Is::MIN.abs();
        assert_eq![x, Is::MAX];
        assert_eq![x.bound_count(), 0];
        assert_eq![x.bound_dir(), None];
    }
    #[test]
    fn raw_negative_extra_endpoint_is_rejected() {
        // Low 5 payload bits `10000` decode as -16.
        let raw_payload_min = 0b1_0000i8;
        assert![Is::from_raw(raw_payload_min).is_none()];
        assert![Is::from_raw(RAW_RESERVED).is_none()];
    }
}
