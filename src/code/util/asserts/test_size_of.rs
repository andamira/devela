// devela::code::util::asserts::test_size_of

#[doc = crate::_tags!(assert mem)]
/// Tests the stack size of a type.
///
/// Optionally checks the matching bit count
/// and whether `Option<T>` keeps the same stack size as `T`.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// This is a convenience macro around [`size_of`]
/// for quickly probing or locking the expected size of a type in tests.
///
/// # Modes
/// - `assert T = N`: asserts the byte-size inside an existing test.
/// - `assert T = N|B`: also checks the corresponding bit count.
/// - `assert T = N; niche Option`: also checks `Option<T>` has the same size.
/// - `assert { T = N; ... }`: asserts several type sizes.
/// - `probe T`: panics with the measured size.
/// - `name: T = N`: generates a named unit test.
/// - `T = N`: generates `test_size_of_T` for simple identifier types.
///
/// The `; niche Option` suffix checks size preservation:
/// `size_of::<Option<T>>() == size_of::<T>()`.
///
/// # Panics
/// Panics when the measured byte-size does not match the expected value.
///
/// Panics when an explicit bit count does not match the expected byte-size
/// converted to bits.
///
/// Panics when `; niche Option` is used and `Option<T>` does not have the same
/// stack size as `T`.
///
/// The `probe` mode always panics, intentionally, and prints the measured size.
///
/// # Examples
/// ```
/// # use devela::test_size_of;
/// test_size_of![assert u8 = 1];
/// test_size_of![assert u16 = 2|16];
/// test_size_of![assert [u8; 4] = 4];
/// test_size_of![assert core::num::NonZeroU8 = 1; niche Option];
///
/// test_size_of![assert {
///     u16 = 2;
///     u32 = 4|32;
///     [u8; 8] = 8;
///     core::num::NonZeroU8 = 1; niche Option;
/// }];
/// ```
///
/// Generated test:
/// ```
/// # use devela::test_size_of;
/// test_size_of![u32 = 4];
/// test_size_of![core_num_nonzero_u8: core::num::NonZeroU8 = 1; niche Option];
/// test_size_of![option_nonzero_u8: Option<core::num::NonZeroU8> = 1];
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! test_size_of {
    // Assertion-only mode, for use inside existing tests.
    //
    // Checks the byte-size, optionally the bit count, and optionally a
    // supported niche-preserving wrapper such as `Option<T>`.
    (assert $ty:ty = $bytes:literal $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?) => {{
        let (actual, expected): (usize, usize) = (size_of::<$ty>(), $bytes);
        if actual != expected {
            ::core::panic!(
    "size_of::<{}>() mismatch:\n  actual:   {} bytes ({} bits)\n  expected: {} bytes ({} bits)",
                ::core::any::type_name::<$ty>(),
                actual, actual.saturating_mul(8),
                expected, expected.saturating_mul(8),
            );
        }
        $(
            let (actual_bits, expected_bits): (usize, usize) = (expected.saturating_mul(8), $bits);
            if actual_bits != expected_bits {
                ::core::panic!(
    "size_of::<{}>() bit mismatch:\n  bytes:    {} bytes\n  actual:   {} bits\n  expected: {} bits",
                    ::core::any::type_name::<$ty>(),
                    expected, actual_bits, expected_bits,
                );
            }
        )?
        $( $crate::test_size_of![@assert_niche $ty, $wrap]; )?
    }};
    // Batch assertion-only mode.
    //
    // Delegates to a small TT muncher so `; niche Option;` is parsed unambiguously:
    // the first semicolon belongs to the niche suffix, the second one ends the batch entry.
    (assert { $($rest:tt)* }) => {{
        $crate::test_size_of![@assert_batch $($rest)*];
    }};
    // Batch parser: end of input.
    (@assert_batch) => {};
    // Batch parser: one entry with a niche-preserving wrapper suffix.
    (@assert_batch
        $ty:ty = $bytes:literal $(| $bits:literal)? ; niche $wrap:ident ; $($rest:tt)*) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?; niche $wrap];
        $crate::test_size_of![@assert_batch $($rest)*];
    }};
    // Batch parser: one plain size entry.
    (@assert_batch $ty:ty = $bytes:literal $(| $bits:literal)? ; $($rest:tt)*) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?];
        $crate::test_size_of![@assert_batch $($rest)*];
    }};
    // Checks that `Option<T>` preserves the same stack size as `T`.
    (@assert_niche $ty:ty, Option) => {{
        let (plain, wrapped): (usize, usize) = (size_of::<$ty>(), size_of::<Option<$ty>>());
        if wrapped != plain {
            ::core::panic!(
    "Option niche size mismatch for `{}`:\n  size_of::<{}>()         = {} bytes ({} bits)\n  size_of::<Option<{}>>() = {} bytes ({} bits)\n  expected: same stack size",
                ::core::any::type_name::<$ty>(), ::core::any::type_name::<$ty>(),
                plain, plain.saturating_mul(8), ::core::any::type_name::<$ty>(),
                wrapped, wrapped.saturating_mul(8),
            );
        }
    }};
    // Rejects unknown niche-wrapper names explicitly.
    (@assert_niche $ty:ty, $wrap:ident) => {
        ::core::compile_error!( concat!( "unsupported `test_size_of!` niche wrapper `",
                stringify!($wrap), "`; currently supported: `Option`"));
    };
    // Probe mode, intentionally fails and prints the measured size.
    (probe $ty:ty $(,)?) => {{
        let actual: usize = size_of::<$ty>();
        ::core::panic!("size_of::<{}>() = {} bytes ({} bits)",
            ::core::any::type_name::<$ty>(), actual, actual.saturating_mul(8),
        );
    }};
    // Named generated test with expected size.
    ($name:ident : $ty:ty = $bytes:literal $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?) => {
        #[cfg(test)] #[test] #[allow(nonstandard_style)]
        fn $name() { $crate::test_size_of![assert $ty = $bytes $(| $bits)? $(; niche $wrap)?]; }
    };
    // Named generated probe test.
    ($name:ident : probe $ty:ty $(,)?) => {
        #[cfg(test)] #[test] #[allow(nonstandard_style)]
        fn $name() { $crate::test_size_of![probe $ty]; }
    };
    // Shorthand generated test for simple identifier types.
    ($ty:ident = $bytes:literal $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?
        ) => { $crate::paste! {
        $crate::test_size_of!([<test_size_of_ $ty>] : $ty = $bytes $(| $bits)? $(; niche $wrap)?);
    }};
    // Shorthand generated probe test for simple identifier types.
    (probe $ty:ident $(,)?) => { $crate::paste! {
        $crate::test_size_of!([<test_size_of_ $ty>] : probe $ty);
    }};
}
#[doc(inline)]
pub use test_size_of;

#[cfg(test)]
mod tests {
    struct TestSizeOfUnit;

    #[allow(dead_code)]
    struct TestSizeOfTuple(u8, u16);

    type TestSizeOfNonZeroU8 = core::num::NonZeroU8;

    /* generated tests */

    test_size_of![TestSizeOfUnit = 0];
    test_size_of![TestSizeOfNonZeroU8 = 1; niche Option];

    test_size_of![test_size_of_explicit_ident: TestSizeOfTuple = 4];
    test_size_of![test_size_of_explicit_ident_bits: TestSizeOfTuple = 4|32];
    test_size_of![test_size_of_array: [u8; 4] = 4];
    test_size_of![test_size_of_array_bits: [u8; 4] = 4|32];
    test_size_of![test_size_of_option_nonzero_u8: Option<core::num::NonZeroU8> = 1];
    test_size_of![test_size_of_explicit_niche_option: core::num::NonZeroU8 = 1; niche Option];

    /* assertion mode */

    #[test]
    fn test_size_of_assert_one() {
        test_size_of![assert u8 = 1];
        test_size_of![assert u8 = 1|8];
        test_size_of![assert u16 = 2];
        test_size_of![assert u16 = 2|16];
        test_size_of![assert [u8; 4] = 4];
        test_size_of![assert [u8; 4] = 4|32];
        test_size_of![assert Option<core::num::NonZeroU8> = 1];
    }
    #[test]
    fn test_size_of_assert_niche_option() {
        test_size_of![assert core::num::NonZeroU8 = 1; niche Option];
        test_size_of![assert core::num::NonZeroU8 = 1|8; niche Option];
        test_size_of![assert TestSizeOfNonZeroU8 = 1; niche Option];
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
    fn test_size_of_assert_batch_bits() {
        test_size_of![assert {
            u8 = 1|8;
            u16 = 2|16;
            u32 = 4|32;
            [u8; 8] = 8|64;
            Option<core::num::NonZeroU8> = 1|8;
        }];
    }
    #[test]
    fn test_size_of_assert_batch_niche_option() {
        test_size_of![assert {
            core::num::NonZeroU8 = 1; niche Option;
            TestSizeOfNonZeroU8 = 1|8; niche Option;
        }];
    }
    /* size mismatch diagnostics */
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
    /* bit mismatch diagnostics */
    #[test]
    #[should_panic(expected = "size_of::<u16>() bit mismatch")]
    fn test_size_of_assert_bit_mismatch_panics() {
        test_size_of![assert u16 = 2|15];
    }
    #[test]
    #[should_panic(expected = "actual:   16 bits")]
    fn test_size_of_assert_bit_mismatch_reports_actual_bits() {
        test_size_of![assert u16 = 2|15];
    }
    #[test]
    #[should_panic(expected = "expected: 15 bits")]
    fn test_size_of_assert_bit_mismatch_reports_expected_bits() {
        test_size_of![assert u16 = 2|15];
    }
    /* niche mismatch diagnostics */
    #[test]
    #[should_panic(expected = "Option niche size mismatch for `u8`")]
    fn test_size_of_assert_niche_option_panics() {
        test_size_of![assert u8 = 1; niche Option];
    }
    #[test]
    #[should_panic(expected = "size_of::<Option<u8>>()")]
    fn test_size_of_assert_niche_option_reports_wrapped_type() {
        test_size_of![assert u8 = 1; niche Option];
    }
    #[test]
    #[should_panic(expected = "expected: same stack size")]
    fn test_size_of_assert_niche_option_reports_expected_relation() {
        test_size_of![assert u8 = 1; niche Option];
    }
    /* probe mode */
    #[test]
    #[should_panic(expected = "size_of::<u16>() = 2 bytes (16 bits)")]
    fn test_size_of_probe_panics_with_measured_size() {
        test_size_of![probe u16];
    }
}
