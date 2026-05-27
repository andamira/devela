// devela::code::util::asserts::test_size_of

#[doc = crate::_tags!(assert)]
/// Tests the byte-size of a type, optionally checking the matching bit count.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// This is a convenience macro around [`size_of`]
/// for quickly probing or locking the expected size of a type in tests.
///
/// # Modes
/// - `assert T = N`: asserts the size inside an existing test.
/// - `assert { T = N; ... }`: asserts several type sizes.
/// - `probe T`: panics with the measured size.
/// - `name: T = N`: generates a named unit test.
/// - `T = N`: generates `test_size_of_T` for simple identifier types.
///
/// # Panics
/// Panics when the measured byte-size does not match the expected value.
///
/// The `probe` mode always panics, intentionally, and prints the measured size.
///
/// # Examples
/// ```
/// # use devela::test_size_of;
/// test_size_of![assert u8 = 1];
/// test_size_of![assert [u8; 4] = 4];
///
/// test_size_of![assert {
///     u16 = 2;
///     [u8; 8] = 8;
/// }];
/// ```
///
/// Generated test:
/// ```
/// # use devela::test_size_of;
/// test_size_of![u32 = 4];
/// test_size_of![option_nonzero_u8: Option<core::num::NonZeroU8> = 1];
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! test_size_of {
    (
        // Assertion-only mode, for use inside existing tests.
        assert $ty:ty = $bytes:literal $(| $bits:literal)? $(,)?
    ) => {{
        let actual: usize = size_of::<$ty>();
        let expected: usize = $bytes;
        if actual != expected {
            ::core::panic!(
        "size_of::<{}>() mismatch:\n  actual:   {} bytes ({} bits)\n  expected: {} bytes ({} bits)",
                ::core::any::type_name::<$ty>(),
                actual, actual.saturating_mul(8),
                expected, expected.saturating_mul(8),
            );
        }
        $(
            let actual_bits: usize = expected.saturating_mul(8);
            let expected_bits: usize = $bits;

            if actual_bits != expected_bits {
                ::core::panic!(
    "size_of::<{}>() bit mismatch:\n  bytes:    {} bytes\n  actual:   {} bits\n  expected: {} bits",
                    ::core::any::type_name::<$ty>(),
                    expected,
                    actual_bits,
                    expected_bits,
                );
            }
        )?
    }};
    (
        // Batch assertion-only mode.
        assert { $($ty:ty = $bytes:literal $(| $bits:literal)?;)+ }
    ) => {{
        $( $crate::test_size_of![assert $ty = $bytes $(| $bits)?]; )+
    }};
    (
        // Probe mode, intentionally fails and prints the measured size.
        probe $ty:ty $(,)?
    ) => {{
        let actual: usize = size_of::<$ty>();
        ::core::panic!("size_of::<{}>() = {} bytes ({} bits)",
            ::core::any::type_name::<$ty>(), actual, actual.saturating_mul(8),
        );
    }};
    (
        // Named generated test with expected size.
        $name:ident : $ty:ty = $bytes:literal $(| $bits:literal)? $(,)?
    ) => {
        #[cfg(test)] #[test] #[allow(nonstandard_style)]
        fn $name() { $crate::test_size_of![assert $ty = $bytes $(| $bits)?]; }
    };
    (
        // Named generated probe test.
        $name:ident : probe $ty:ty $(,)?
    ) => {
        #[cfg(test)] #[test] #[allow(nonstandard_style)]
        fn $name() { $crate::test_size_of![probe $ty]; }
    };
    (
        // Shorthand generated test for simple identifier types.
        $ty:ident = $bytes:literal $(| $bits:literal)? $(,)?
    ) => {
        $crate::paste! {
            $crate::test_size_of!([<test_size_of_ $ty>] : $ty = $bytes $(| $bits)?);
        }
    };
    (
        // Shorthand generated probe test for simple identifier types.
        probe $ty:ident $(,)?
    ) => { $crate::paste! { $crate::test_size_of!([<test_size_of_ $ty>] : probe $ty); } };
}
#[doc(inline)]
pub use test_size_of;

#[cfg(test)]
mod tests {
    struct TestSizeOfUnit;
    #[allow(dead_code)]
    struct TestSizeOfTuple(u8, u16);

    test_size_of![TestSizeOfUnit = 0];
    test_size_of![test_size_of_explicit_ident: TestSizeOfTuple = 4];
    test_size_of![test_size_of_array: [u8; 4] = 4];
    test_size_of![test_size_of_option_nonzero_u8: Option<core::num::NonZeroU8> = 1];

    #[test]
    fn test_size_of_assert_one() {
        test_size_of![assert u8 = 1];
        test_size_of![assert u16 = 2];
        test_size_of![assert [u8; 4] = 4];
        test_size_of![assert Option<core::num::NonZeroU8> = 1];
    }
    #[test]
    fn test_size_of_assert_batch() {
        test_size_of![assert {
            u8 = 1;
            u16 = 2;
            u32 = 4;
            [u8; 8] = 8;
            Option<core::num::NonZeroU8> = 1;
        }];
    }
    #[test]
    #[should_panic(expected = "size_of::<u16>() mismatch")]
    fn test_size_of_assert_panics() {
        test_size_of![assert u16 = 1];
    }
    #[test]
    #[should_panic(expected = "actual:   2 bytes (16 bits)")]
    fn test_size_of_assert_panic_reports_actual_size() {
        test_size_of![assert u16 = 1];
    }
    #[test]
    #[should_panic(expected = "expected: 1 bytes (8 bits)")]
    fn test_size_of_assert_panic_reports_expected_size() {
        test_size_of![assert u16 = 1];
    }
    #[test]
    #[should_panic(expected = "size_of::<u16>() = 2 bytes (16 bits)")]
    fn test_size_of_probe_panics_with_measured_size() {
        test_size_of![probe u16];
    }
}
