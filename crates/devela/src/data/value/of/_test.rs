//

mod one {
    use crate::{Infallible, Oneof};

    type Bytes = Oneof<2, u8, i8>;
    type Unums = Oneof<4, u8, u16, u32, u64>;

    #[test]
    fn validate() {
        assert![Bytes::validate()];
        assert![Unums::validate()];

        assert![Oneof::<0>::validate()];
        assert![Oneof::<1, i8>::validate()];
        assert![Oneof::<2, (), i8>::validate()];

        assert![!Oneof::<0, i8>::validate()];
        assert![!Oneof::<2, i8>::validate()];
        assert![!Oneof::<1, Infallible, i8>::validate()];
        assert![!Oneof::<2, i32, Infallible, i8>::validate()];
    }
    #[test]
    fn map() {
        let a: Oneof<2, i32, f64> = Oneof::_0(10);
        assert_eq![Oneof::_0(20), a.map_0(|v| v * 2)];
        assert_eq![Oneof::_0(10), a.map_1(|v| v * 2.0)];
        let b: Oneof<2, i32, f64> = Oneof::_1(3.14);
        assert_eq![Oneof::_1(3.14), b.map_0(|v| v * 2)];
        assert_eq![Oneof::_1(6.28), b.map_1(|v| v * 2.0)];
    }
    #[test]
    fn field_access() {
        let mut u = Unums::_2(32);
        assert_eq![u.is_2(), true];
        assert_eq![u.into_2(), Some(32)];
        assert_eq![u.as_ref_2(), Some(&32)];
        assert_eq![u.as_mut_2(), Some(&mut 32)];
        //
        assert_eq![u.is_0(), false];
        assert_eq![u.into_0(), None];
        assert_eq![u.as_ref_0(), None];
        assert_eq![u.as_mut_0(), None];
    }
    #[test]
    fn positioning() {
        let u = Unums::_2(32);
        assert_eq![u.variant_index(), 2];
        assert_eq![u.is_variant_index(2), true];
        assert_eq![u.is_variant_index(3), false];
        // assert_eq![u.variant_name(), "_2"];
        // assert_eq![u.is_variant_name("_2"), true];
        // assert_eq![u.is_variant_name("_1"), false];

        let u = Unums::_0(32);
        assert_eq![u.variant_index(), 0];
        assert_eq![u.is_variant_index(0), true];
        assert_eq![u.is_variant_index(1), false];
        // assert_eq![u.variant_name(), "_0"];
        // assert_eq![u.is_variant_name("_0"), true];
        // assert_eq![u.is_variant_name("_1"), false];
    }
    #[test]
    fn tuple() {
        let u = Unums::_2(32);
        assert_eq![
            u.into_tuple_options(),
            (None, None, Some(32), None, None, None, None, None, None, None, None, None)
        ];
        assert_eq![
            u.as_tuple_ref_options(),
            (None, None, Some(&32), None, None, None, None, None, None, None, None, None)
        ];
        // FIXME
        // assert_eq![u.into_tuple_defaults(), (0, 0, 32, 0, (), (), (), (), (), (), (), ())];
    }
}
