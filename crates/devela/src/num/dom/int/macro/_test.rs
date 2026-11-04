//
//! Macro expansion, parity, and const-evaluation tests.

#[cfg(test)]
mod _test {
    use crate::{Int, int};

    int! {
        struct TestI32: i32;
        ops(base);
    }
    int! {
        struct TestU32: u32;
        ops(base);
    }

    const SIGNED_CONST: TestI32 = TestI32::new(-128).digits_base_sign(TestI32::new(16));
    const UNSIGNED_CONST: TestU32 = TestU32::new(126).digital_root();
    const _: () = assert!(SIGNED_CONST.get() == 3);
    const _: () = assert!(UNSIGNED_CONST.get() == 9);

    int! { struct TestFamily : <> impl (i8, i32, u8, u32); ops(base); }
    int! { struct TestLateFamily : <>; }
    int! { impl TestLateFamily : <> (i16, u16); ops(base); }
    int! { struct TestGrouped: (i64) impl; ops(base); }

    const FAMILY_CONST: TestFamily<i8> =
        TestFamily::<i8>::new(i8::MIN).digits_base(TestFamily::<i8>::new(2));
    const _: () = assert!(FAMILY_CONST.get() == 8);
    const _: () = assert!(TestLateFamily::<u16>::new(255).digital_root().get() == 3);
    const _: () = assert!(TestGrouped::new(i64::MIN).digital_root().get() == 8);

    #[test]
    fn generic_and_concrete_forms() {
        assert_eq!(TestFamily::<i8>::new(i8::MIN).digital_root().get(), 2);
        assert_eq!(TestFamily::<u8>::new(u8::MAX).digits_base(TestFamily::<u8>::new(2)).get(), 8);
        assert_eq!(TestFamily::<i32>::new(i32::MIN).digits().get(), 10);
        assert_eq!(TestFamily::<u32>::new(u32::MAX).digits().get(), 10);
        assert_eq!(
            TestLateFamily::<i16>::new(i16::MIN).digits_base(TestLateFamily::<i16>::new(2)).get(),
            16
        );
        assert_eq!(TestGrouped::new(i64::MIN).digital_root().get(), 8);
    }

    #[test]
    fn base_signed_matches_int() {
        for value in [0, 1, -1, 9, 10, 127, -128, i32::MIN, i32::MAX] {
            let generated = TestI32::new(value);
            assert_eq!(generated.digits().get(), Int(value).digits().0);
            assert_eq!(generated.digits_sign().get(), Int(value).digits_sign().0);
            for base in [0, 1, -1, 2, 10, 16, -16, i32::MIN] {
                let base_value = TestI32::new(base);
                assert_eq!(generated.digits_base(base_value).get(), Int(value).digits_base(base).0);
                assert_eq!(
                    generated.digits_base_sign(base_value).get(),
                    Int(value).digits_base_sign(base).0,
                );
            }
            assert_eq!(generated.digital_root().get(), Int(value).digital_root().0);
            for base in [0, 1, -1, 2, 10, 16, -16, i32::MIN] {
                assert_eq!(
                    generated.digital_root_base(TestI32::new(base)).get(),
                    Int(value).digital_root_base(base).0,
                );
            }
        }
    }
    #[test]
    fn base_unsigned_matches_int() {
        for value in [0, 1, 9, 10, 127, u32::MAX] {
            let generated = TestU32::new(value);
            assert_eq!(generated.digits().get(), Int(value).digits().0);
            assert_eq!(generated.digits_sign().get(), Int(value).digits_sign().0);
            for base in [0, 1, 2, 10, 16] {
                let base_value = TestU32::new(base);
                assert_eq!(generated.digits_base(base_value).get(), Int(value).digits_base(base).0);
                assert_eq!(
                    generated.digits_base_sign(base_value).get(),
                    Int(value).digits_base_sign(base).0,
                );
            }
            assert_eq!(generated.digital_root().get(), Int(value).digital_root().0);
            for base in [0, 1, 2, 10, 16] {
                assert_eq!(
                    generated.digital_root_base(TestU32::new(base)).get(),
                    Int(value).digital_root_base(base).0,
                );
            }
        }
    }
}
