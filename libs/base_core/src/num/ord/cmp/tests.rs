// devela_base_core::num::ord::cmp

use super::Cmp;

#[test]
fn min_max_clamp() {
    assert_eq![Some(2), Cmp(2).pmin(5)];
    assert_eq![Some(2), Cmp(5).pmin(2)];
    assert_eq![Some(2.), Cmp(2.).pmin(5.)];

    assert_eq![Some(5), Cmp(2).pmax(5)];
    assert_eq![Some(5), Cmp(5).pmax(2)];
    assert_eq![Some(5.), Cmp(2.).pmax(5.)];

    assert_eq![Some(3), Cmp(3).pclamp(2, 5)];
    assert_eq![Some(3.), Cmp(3.).pclamp(2., 5.)];
    assert_eq![Some(2), Cmp(1).pclamp(2, 5)];
    assert_eq![Some(5), Cmp(7).pclamp(2, 5)];
}

#[test]
fn float() {
    let (zero, negzero, one, negone) = (Cmp(0.0_f32), Cmp(-0.0_f32), Cmp(1.0_f32), Cmp(-1.0_f32));
    let (nan1, nan2) = (Cmp(f32::NAN), Cmp(0.0_f32 / 0.0_f32));
    let (inf, neginf) = (Cmp(f32::INFINITY), Cmp(f32::NEG_INFINITY));
    let sub = Cmp(1.401298464e-45_f32);
    let (min, negmin) = (Cmp(f32::MIN_POSITIVE), Cmp(-f32::MIN_POSITIVE));

    assert![nan1.0.is_nan()];
    assert![nan2.0.is_nan()];
    assert![!zero.0.is_nan()];
    assert![!negzero.0.is_nan()];
    assert![!one.0.is_nan()];
    assert![!negone.0.is_nan()];
    assert![!inf.0.is_nan()];
    assert![!neginf.0.is_nan()];
    assert![!min.0.is_nan()];
    assert![!negmin.0.is_nan()];

    assert![negone.0.is_sign_negative()];
    assert![negzero.0.is_sign_negative()];
    assert![neginf.0.is_sign_negative()];
    assert![!negone.0.is_sign_positive()];
    assert![!negzero.0.is_sign_positive()];
    assert![!neginf.0.is_sign_positive()];

    assert![sub.0.is_subnormal() && !sub.0.is_normal()];
    assert![!zero.0.is_subnormal() && !zero.0.is_normal()];
    assert![one.0.is_normal() && !one.0.is_subnormal()];
    assert![min.0.is_normal() && !min.0.is_subnormal()];
    assert![negmin.0.is_normal() && !negmin.0.is_subnormal()];
}
