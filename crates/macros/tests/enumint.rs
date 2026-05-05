// devela_macros/tests/enumint.rs
//
//!
//

use devela_macros::enumint;

enumint!(U4, u8, 2, 5);
enumint!(I6, i8, -2, 3);
enumint!(U253, u8, 1, 253);

enumint!(I16_6, i16, -2, 3);
enumint!(pub PublicU4, u8, 2, 5);
enumint!(pub(crate) CrateI6, i16, -2, 3);

#[test]
fn enumint_constants_and_niches() {
    assert_eq!(U4::MIN, 2);
    assert_eq!(U4::MAX, 5);
    assert_eq!(U4::VALUES, 4);
    assert_eq!(U4::NICHES, 252);

    assert_eq!(I6::MIN, -2);
    assert_eq!(I6::MAX, 3);
    assert_eq!(I6::VALUES, 6);
    assert_eq!(I6::NICHES, 250);

    assert_eq!(I16_6::VALUES, 6);
    assert_eq!(I16_6::NICHES, 65_530);
}
#[test]
fn enumint_option_niche_layout() {
    assert_eq!(size_of::<U4>(), 1);
    assert_eq!(size_of::<Option<U4>>(), 1);
    assert_eq!(size_of::<Option<Option<U4>>>(), 1);

    assert_eq!(size_of::<U253>(), 1);
    assert_eq!(size_of::<Option<U253>>(), 1);
    assert_eq!(size_of::<Option<Option<U253>>>(), 1);
    assert_eq!(size_of::<Option<Option<Option<U253>>>>(), 1);
    assert_eq!(size_of::<Option<Option<Option<Option<U253>>>>>(), 2);

    assert_eq!(size_of::<I6>(), 1);
    assert_eq!(size_of::<Option<I6>>(), 1);
}
#[test]
fn enumint_checked_unsigned() {
    use U4::{P2, P3, P4, P5};
    let cases = [
        (0, None),
        (1, None),
        (2, Some(P2)),
        (3, Some(P3)),
        (4, Some(P4)),
        (5, Some(P5)),
        (6, None),
        (7, None),
    ];
    for (value, expected) in cases {
        assert_eq!(U4::new(value), expected, "value = {value}");
    }
}
#[test]
fn enumint_checked_signed() {
    use I6::{N1, N2, P0, P1, P2, P3};
    let cases = [
        (-3, None),
        (-2, Some(N2)),
        (-1, Some(N1)),
        (0, Some(P0)),
        (1, Some(P1)),
        (2, Some(P2)),
        (3, Some(P3)),
        (4, None),
    ];
    for (value, expected) in cases {
        assert_eq!(I6::new(value), expected, "value = {value}");
    }
}
#[test]
fn enumint_saturated_unsigned() {
    use U4::{P2, P3, P4, P5};
    let cases = [(0, P2), (1, P2), (2, P2), (3, P3), (4, P4), (5, P5), (6, P5), (7, P5)];
    for (value, expected) in cases {
        assert_eq!(U4::new_saturated(value), expected, "value = {value}");
    }
}
#[test]
fn enumint_wrapped_unsigned() {
    use U4::{P2, P3, P4, P5};
    let cases = [(0, P4), (1, P5), (2, P2), (3, P3), (4, P4), (5, P5), (6, P2), (7, P3)];
    for (value, expected) in cases {
        assert_eq!(U4::new_wrapped(value), expected, "value = {value}");
    }
}
#[test]
fn enumint_wrapped_signed() {
    use I6::{N1, N2, P0, P1, P2, P3};
    let cases = [
        (-4, P2),
        (-3, P3),
        (-2, N2),
        (-1, N1),
        (0, P0),
        (1, P1),
        (2, P2),
        (3, P3),
        (4, N2),
        (5, N1),
    ];
    for (value, expected) in cases {
        assert_eq!(I6::new_wrapped(value), expected, "value = {value}");
    }
}
#[test]
fn enumint_full_u8() {
    enumint!(FullU8, u8, 0, 255);

    assert_eq!(FullU8::MIN, 0);
    assert_eq!(FullU8::MAX, 255);
    assert_eq!(FullU8::VALUES, 256);
    assert_eq!(FullU8::NICHES, 0);

    assert_eq!(FullU8::new(0), Some(FullU8::P0));
    assert_eq!(FullU8::new(255), Some(FullU8::P255));

    assert_eq!(FullU8::new_wrapped(0), FullU8::P0);
    assert_eq!(FullU8::new_wrapped(255), FullU8::P255);
}
#[test]
fn enumint_full_i8() {
    enumint!(FullI8, i8, -128, 127);

    assert_eq!(FullI8::MIN, -128);
    assert_eq!(FullI8::MAX, 127);
    assert_eq!(FullI8::VALUES, 256);
    assert_eq!(FullI8::NICHES, 0);

    assert_eq!(FullI8::new(-128), Some(FullI8::N128));
    assert_eq!(FullI8::new(127), Some(FullI8::P127));

    assert_eq!(FullI8::new_wrapped(-128), FullI8::N128);
    assert_eq!(FullI8::new_wrapped(127), FullI8::P127);
}
#[test]
fn enumint_wide_i8_wrapping() {
    enumint!(WideI8, i8, -100, 100);

    assert_eq!(WideI8::new_wrapped(-101), WideI8::P100);
    assert_eq!(WideI8::new_wrapped(101), WideI8::N100);
    assert_eq!(WideI8::new_wrapped(100), WideI8::P100);
    assert_eq!(WideI8::new_wrapped(-100), WideI8::N100);
}
#[test]
fn enumint_singletons() {
    enumint!(OneU8, u8, 7, 7);
    enumint!(OneI8, i8, -1, -1);

    assert_eq!(OneU8::VALUES, 1);
    assert_eq!(OneU8::NICHES, 255);
    assert_eq!(OneU8::new(7), Some(OneU8::P7));
    assert_eq!(OneU8::new_saturated(0), OneU8::P7);
    assert_eq!(OneU8::new_wrapped(255), OneU8::P7);

    assert_eq!(OneI8::VALUES, 1);
    assert_eq!(OneI8::NICHES, 255);
    assert_eq!(OneI8::new(-1), Some(OneI8::N1));
    assert_eq!(OneI8::new_saturated(-128), OneI8::N1);
    assert_eq!(OneI8::new_wrapped(127), OneI8::N1);
}
#[test]
fn enumint_const_methods() {
    const NEW: Option<U4> = U4::new(3);
    const SAT: U4 = U4::new_saturated(99);
    const WRAP: U4 = U4::new_wrapped(6);
    const GET: u8 = U4::P4.get();

    assert_eq!(NEW, Some(U4::P3));
    assert_eq!(SAT, U4::P5);
    assert_eq!(WRAP, U4::P2);
    assert_eq!(GET, 4);
}
#[test]
fn enumint_roundtrip_unsigned() {
    for n in 0u8..=255 {
        let got = U4::new(n).map(U4::get);
        let expected = if (2..=5).contains(&n) { Some(n) } else { None };
        assert_eq!(got, expected, "n = {n}");
    }
}
#[test]
fn enumint_roundtrip_signed() {
    for n in i8::MIN..=i8::MAX {
        let got = I6::new(n).map(I6::get);
        let expected = if (-2..=3).contains(&n) { Some(n) } else { None };
        assert_eq!(got, expected, "n = {n}");
    }
}
