use devela_macros::enumint;

enumint![pub UEnum, u8, 2, 5]; // Generates the following enum:

/*
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum UEnum {
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
}
*/

enumint!(pub(crate) IEnum, i16, -2, 3); // Generates the following enum:

/*
#[repr(i16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum IEnum {
    N2 = -2,
    N1 = -1,
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
}
*/

fn main() {
    /* UNSIGNED */

    // constants
    assert_eq![UEnum::MIN, 2];
    assert_eq![UEnum::MAX, 5];
    assert_eq![UEnum::VALID_VALUES, 4];
    assert_eq![UEnum::NICHE_VALUES, 252];
    // cast
    assert_eq![UEnum::P2.get(), 2];
    assert_eq![UEnum::P2 as u8, 2];
    // construct
    assert_eq![UEnum::new(3), Some(UEnum::P3)];
    assert_eq![UEnum::new(1), None];
    assert_eq![UEnum::new(6), None];
    assert_eq![UEnum::new_saturated(1), UEnum::P2];
    assert_eq![UEnum::new_saturated(6), UEnum::P5];
    assert_eq![UEnum::new_wrapped(1), UEnum::P5];
    assert_eq![UEnum::new_wrapped(6), UEnum::P2];

    /* SIGNED */

    // constants
    assert_eq![IEnum::MIN, -2];
    assert_eq![IEnum::MAX, 3];
    assert_eq![IEnum::VALID_VALUES, 6];
    assert_eq![IEnum::NICHE_VALUES, 65_530];
    // cast
    assert_eq![IEnum::N1.get(), -1];
    assert_eq![IEnum::N1 as i8, -1];
    // construct
    assert_eq![IEnum::new(3), Some(IEnum::P3)];
    assert_eq![IEnum::new(-3), None];
    assert_eq![IEnum::new(4), None];
    assert_eq![IEnum::new_saturated(-3), IEnum::N2];
    assert_eq![IEnum::new_saturated(4), IEnum::P3];
    assert_eq![IEnum::new_wrapped(-3), IEnum::P3];
    assert_eq![IEnum::new_wrapped(4), IEnum::N2];
}
