// devela/src/data/value/word/_test.rs

use super::{Word, WordTry, word};

/* tuple definition */

word! {
    struct TupleWord(u8);
}

/* named definition */

word! {
    struct NamedWord {
        bits: u8,
    }
}

/* existing tuple */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ExistingTuple(u16);

word! {
    impl ExistingTuple(u16);
}

/* existing named */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ExistingNamed {
    bits: u16,
}

word! {
    impl ExistingNamed {
        bits: u16,
    }
}

/* fallible tuple */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NibbleError;

word! {
    struct Nibble(u8);

    type Error = NibbleError;
    try_from_raw(raw) {
        if raw <= 0x0F {
            Ok(Self(raw))
        } else {
            Err(NibbleError)
        }
    }
}

/* fallible named */

word! {
    struct NamedNibble {
        bits: u8,
    }

    type Error = NibbleError;
    try_from_raw(raw) {
        if raw <= 0x0F {
            Ok(Self { bits: raw })
        } else {
            Err(NibbleError)
        }
    }
}

/* arbitrary infallible representation */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pair {
    lo: u8,
    hi: u8,
}

word! {
    impl Pair => [u8; 2] {
        raw(pair) {
            [pair.lo, pair.hi]
        }
        from_raw(raw) {
            Self {
                lo: raw[0],
                hi: raw[1],
            }
        }
    }
}

/* arbitrary fallible representation: enum */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SmallEnum {
    A,
    B,
    C,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SmallEnumError;

word! {
    impl SmallEnum => u8 {
        type Error = SmallEnumError;

        raw(value) {
            match value {
                Self::A => 0,
                Self::B => 1,
                Self::C => 2,
            }
        }
        try_from_raw(raw) {
            match raw {
                0 => Ok(Self::A),
                1 => Ok(Self::B),
                2 => Ok(Self::C),
                _ => Err(SmallEnumError),
            }
        }
    }
}

/* const surface */

const TUPLE_CONST: TupleWord = TupleWord::from_raw(7);
const NAMED_CONST: NamedWord = NamedWord::from_raw(8);
const PAIR_CONST: Pair = Pair::from_raw([9, 10]);

const NIBBLE_CONST: Nibble = match Nibble::try_from_raw(11) {
    Ok(value) => value,
    Err(_) => panic!("valid test nibble"),
};

fn requires_word<T: Word>() {}
fn requires_word_try<T: WordTry>() {}

#[test]
fn infallible_traits() {
    requires_word::<TupleWord>();
    requires_word::<NamedWord>();
    requires_word::<ExistingTuple>();
    requires_word::<ExistingNamed>();
    requires_word::<Pair>();
}

#[test]
fn fallible_traits() {
    requires_word_try::<Nibble>();
    requires_word_try::<NamedNibble>();
    requires_word_try::<SmallEnum>();
}

#[test]
fn const_methods() {
    assert_eq!(TUPLE_CONST.raw(), 7);
    assert_eq!(NAMED_CONST.raw(), 8);
    assert_eq!(PAIR_CONST.raw(), [9, 10]);
    assert_eq!(NIBBLE_CONST.raw(), 11);
}

#[test]
fn infallible_roundtrip_laws() {
    for raw in u8::MIN..=u8::MAX {
        let word = TupleWord::from_raw(raw);

        assert_eq!(word.raw(), raw);
        assert_eq!(TupleWord::try_from_raw(raw), Ok(word));
        assert_eq!(TupleWord::from_raw(word.raw()), word);
    }
}

#[test]
fn fallible_roundtrip_laws() {
    for raw in u8::MIN..=u8::MAX {
        match Nibble::try_from_raw(raw) {
            Ok(word) => {
                assert_eq!(word.raw(), raw);
                assert_eq!(Nibble::try_from_raw(word.raw()), Ok(word));
            }
            Err(NibbleError) => {
                assert!(raw > 0x0F);
            }
        }
    }
}

#[test]
fn named_forms() {
    assert_eq!(NamedWord::from_raw(23).raw(), 23);
    assert_eq!(ExistingNamed::from_raw(1234).raw(), 1234);

    assert_eq!(NamedNibble::try_from_raw(15).map(NamedNibble::raw), Ok(15),);
    assert_eq!(NamedNibble::try_from_raw(16), Err(NibbleError));
}

#[test]
fn explicit_composite_lens() {
    let pair = Pair { lo: 3, hi: 17 };

    assert_eq!(pair.raw(), [3, 17]);
    assert_eq!(Pair::from_raw(pair.raw()), pair);

    assert_eq!(<Pair as WordTry>::try_from_raw([5, 8]), Ok(Pair { lo: 5, hi: 8 }),);
}

#[test]
fn explicit_enum_lens() {
    for (raw, value) in [(0, SmallEnum::A), (1, SmallEnum::B), (2, SmallEnum::C)] {
        assert_eq!(value.raw(), raw);
        assert_eq!(SmallEnum::try_from_raw(raw), Ok(value));
    }

    assert_eq!(SmallEnum::try_from_raw(3), Err(SmallEnumError));
    assert_eq!(SmallEnum::try_from_raw(255), Err(SmallEnumError));
}
