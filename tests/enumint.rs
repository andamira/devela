use devela_macros::enumint;

enumint!(u8, Enum253, 1, 253);
enumint!(u8, Enum4, 2, 5);
enumint!(i8, EnumI4, -2, 3);

#[test]
fn enumint_niches() {
    /* positive repr */

    assert_eq!(4, Enum4::VALID_VALUES);
    assert_eq!(252, Enum4::NICHE_VALUES);
    assert_eq!(1, size_of::<Enum4>());
    assert_eq!(1, size_of::<Option<Enum4>>());
    assert_eq!(1, size_of::<Option<Option<Enum4>>>());
    // ...we could nest 252 times before increasing memory size

    assert_eq!(253, Enum253::VALID_VALUES);
    assert_eq!(3, Enum253::NICHE_VALUES);
    assert_eq!(1, size_of::<Enum253>()); // 0 niches used
    assert_eq!(1, size_of::<Option<Enum253>>()); // 1 niche used
    assert_eq!(1, size_of::<Option<Option<Enum253>>>()); // 2 niches used
    assert_eq!(1, size_of::<Option<Option<Option<Enum253>>>>()); // all 3 niches used
    assert_eq!(2, size_of::<Option<Option<Option<Option<Enum253>>>>>()); // increased

    /* negative repr */

    assert_eq!(6, EnumI4::VALID_VALUES);
    assert_eq!(250, EnumI4::NICHE_VALUES);
    assert_eq!(1, size_of::<EnumI4>());
    assert_eq!(1, size_of::<Option<EnumI4>>());
    assert_eq!(1, size_of::<Option<Option<EnumI4>>>());
}

#[test]
fn enumint_new() {
    /* positive repr */
    {
        use Enum4::{P2, P3, P4, P5};
        assert_eq![Enum4::MAX, P5.get()];

        // checked
        assert_eq![Enum4::new(0), None];
        assert_eq![Enum4::new(1), None];
        //
        assert_eq![Enum4::new(2), Some(P2)];
        assert_eq![Enum4::new(3), Some(P3)];
        assert_eq![Enum4::new(4), Some(P4)];
        assert_eq![Enum4::new(5), Some(P5)];
        //
        assert_eq![Enum4::new(6), None];
        assert_eq![Enum4::new(7), None];

        // saturating
        assert_eq![Enum4::new_saturated(0), P2];
        assert_eq![Enum4::new_saturated(1), P2];
        //
        assert_eq![Enum4::new_saturated(2), P2];
        assert_eq![Enum4::new_saturated(3), P3];
        assert_eq![Enum4::new_saturated(4), P4];
        assert_eq![Enum4::new_saturated(5), P5];
        //
        assert_eq![Enum4::new_saturated(6), P5];
        assert_eq![Enum4::new_saturated(7), P5];

        // wrapping
        assert_eq![Enum4::P4, Enum4::new_wrapped(0)];
        assert_eq![Enum4::P5, Enum4::new_wrapped(1)];
        //
        assert_eq![Enum4::P2, Enum4::new_wrapped(2)];
        assert_eq![Enum4::P3, Enum4::new_wrapped(3)];
        assert_eq![Enum4::P4, Enum4::new_wrapped(4)];
        assert_eq![Enum4::P5, Enum4::new_wrapped(5)];
        //
        assert_eq![Enum4::P2, Enum4::new_wrapped(6)];
        assert_eq![Enum4::P3, Enum4::new_wrapped(7)];
    }

    /* negative repr */
    {
        use EnumI4::{N1, N2, P0, P1, P2, P3};
        assert_eq![EnumI4::MIN, N2.get()];
        assert_eq![EnumI4::MAX, P3.get()];

        // checked
        assert_eq![EnumI4::new(-3), None];
        //
        assert_eq![EnumI4::new(-2), Some(N2)];
        assert_eq![EnumI4::new(-1), Some(N1)];
        assert_eq![EnumI4::new(0), Some(P0)];
        assert_eq![EnumI4::new(1), Some(P1)];
        assert_eq![EnumI4::new(2), Some(P2)];
        assert_eq![EnumI4::new(3), Some(P3)];
        //
        assert_eq![EnumI4::new(4), None];

        // saturating
        assert_eq![EnumI4::new_saturated(-4), N2];
        assert_eq![EnumI4::new_saturated(-3), N2];
        //
        assert_eq![EnumI4::new_saturated(-2), N2];
        assert_eq![EnumI4::new_saturated(-1), N1];
        assert_eq![EnumI4::new_saturated(0), P0];
        assert_eq![EnumI4::new_saturated(1), P1];
        assert_eq![EnumI4::new_saturated(2), P2];
        assert_eq![EnumI4::new_saturated(3), P3];
        //
        assert_eq![EnumI4::new_saturated(4), P3];
        assert_eq![EnumI4::new_saturated(5), P3];

        // wrapping
        assert_eq![EnumI4::new_wrapped(-4), P2];
        assert_eq![EnumI4::new_wrapped(-3), P3];
        //
        assert_eq![EnumI4::new_wrapped(-2), N2];
        assert_eq![EnumI4::new_wrapped(-1), N1];
        assert_eq![EnumI4::new_wrapped(0), P0];
        assert_eq![EnumI4::new_wrapped(1), P1];
        assert_eq![EnumI4::new_wrapped(2), P2];
        assert_eq![EnumI4::new_wrapped(3), P3];
        //
        assert_eq![EnumI4::new_wrapped(4), N2];
        assert_eq![EnumI4::new_wrapped(5), N1];
    }
}
