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
fn enumint_add() {
    {
        use Enum4::{_2, _3, _5};
        assert_eq![Enum4::MAX, _5.get()];

        assert_eq!(_2.checked_add(_3), Some(_5)); // 2 + 3 = 5
        assert_eq!(_2.checked_add(_5), None); // 2 + 5 overflows, should return None

        assert_eq!(_2.saturating_add(_3), _5); // 2 + 3 = 5
        assert_eq!(_2.saturating_add(_5), _5); // 2 + 5 overflows, should return max (_5)

        assert_eq!(_2.wrapping_add(_3), _5); // 2 + 3 = 5
        assert_eq!(_2.wrapping_add(_5), _3); // 2 + 5 wraps around to 3
    }
}
