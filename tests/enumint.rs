use devela_macros::enumint;

enumint!(Enum253, 1, 253);
enumint!(Enum4, 2, 5);

#[test]
fn enumint_niches() {
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
}

#[test]
fn enumint_new() {
    {
        use Enum4::{_2, _3, _4, _5};
        assert_eq![Enum4::MAX, _5.get()];

        // checked
        assert_eq![Enum4::new(0), None];
        assert_eq![Enum4::new(1), None];
        //
        assert_eq![Enum4::new(2), Some(_2)];
        assert_eq![Enum4::new(3), Some(_3)];
        assert_eq![Enum4::new(4), Some(_4)];
        assert_eq![Enum4::new(5), Some(_5)];
        //
        assert_eq![Enum4::new(6), None];
        assert_eq![Enum4::new(7), None];

        // saturating
        assert_eq![Enum4::new_saturated(0), _2];
        assert_eq![Enum4::new_saturated(1), _2];
        //
        assert_eq![Enum4::new_saturated(2), _2];
        assert_eq![Enum4::new_saturated(3), _3];
        assert_eq![Enum4::new_saturated(4), _4];
        assert_eq![Enum4::new_saturated(5), _5];
        //
        assert_eq![Enum4::new_saturated(6), _5];
        assert_eq![Enum4::new_saturated(7), _5];

        // wrapping
        assert_eq![Enum4::_4, Enum4::new_wrapped(0)];
        assert_eq![Enum4::_5, Enum4::new_wrapped(1)];
        //
        assert_eq![Enum4::_2, Enum4::new_wrapped(2)];
        assert_eq![Enum4::_3, Enum4::new_wrapped(3)];
        assert_eq![Enum4::_4, Enum4::new_wrapped(4)];
        assert_eq![Enum4::_5, Enum4::new_wrapped(5)];
        //
        assert_eq![Enum4::_2, Enum4::new_wrapped(6)];
        assert_eq![Enum4::_3, Enum4::new_wrapped(7)];
    }
}
