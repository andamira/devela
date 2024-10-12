use devela_macros::enumint;

enumint!(MyEnum, 2, 5);

/* This generates the following enum:

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MyEnum {
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
}
*/

fn main() {
    // constants
    assert_eq![2, MyEnum::MIN];
    assert_eq![5, MyEnum::MAX];
    assert_eq![4, MyEnum::VALID_VALUES];
    assert_eq![252, MyEnum::NICHE_VALUES];

    // cast
    assert_eq![2, MyEnum::_2.get()];
    assert_eq![2, MyEnum::_2 as u8];

    // construct
    assert_eq![Some(MyEnum::_3), MyEnum::new(3)];
    assert_eq![None, MyEnum::new(1)];
    assert_eq![None, MyEnum::new(6)];

    assert_eq![MyEnum::_2, MyEnum::new_saturated(1)];
    assert_eq![MyEnum::_5, MyEnum::new_saturated(6)];

    assert_eq![MyEnum::_5, MyEnum::new_wrapped(1)];
    assert_eq![MyEnum::_2, MyEnum::new_wrapped(6)];
}
