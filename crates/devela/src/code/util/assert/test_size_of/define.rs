//
//! Defines [`test_size_of!`].
//

#[doc = crate::_tags!(assert mem)]
/// Checks the representation size of a type.
#[doc = crate::_doc_meta!{
    location("code/util/assert", macro test_size_of),
}]
/// The representation size is [`core::mem::size_of::<T>()`]: the number of
/// bytes occupied directly by a value of `T`. It does not include memory
/// reached indirectly through pointers, such as heap allocations owned by
/// `Box`, `Vec`, or `String`.
///
/// Optionally checks the corresponding bit count and whether `Option<T>`
/// preserves or changes the representation size of `T`.
///
/// This is a convenience macro around [`core::mem::size_of`] for probing,
/// checking, or locking the expected size of a type.
///
/// It supports compile-time assertions, runtime assertions, generated unit
/// tests, and caller-buffer diagnostics without allocation.
///
/// # Modes
/// - `const T = N`: asserts the byte size at compile time.
/// - `const T = N|B`: also asserts the corresponding bit count.
/// - `assert T = N`: asserts the byte size at runtime.
/// - `assert T = N|B`: also checks the corresponding bit count.
/// - `assert { T = N; ... }`: checks several type sizes.
/// - `check_into buf; T = N`: returns `(ok, type_name, message)`.
/// - `line_into buf; T = N`: returns `(ok, line)` ready to print.
/// - `probe T`: panics with the measured size.
/// - `name: T = N`: generates a named unit test.
/// - `T = N`: generates `test_size_of_T` for simple identifier types.
///
/// The `const` mode expands to compile-time items and does not depend on the
/// test harness or execute target code. This makes it suitable for validating
/// layouts while cross-compiling.
///
/// # Niche-size checks
/// Any size-checking mode accepting a single type can additionally check the
/// `Option<T>` size relation:
///
/// - `; niche Option`: requires Option<T>() to have the same representation size.
/// - `; niche !Option`: requires Option<T>() to have a different representation size.
///
/// Without a niche suffix, the `Option<T>` size relation is left unchecked.
///
/// These suffixes test size preservation only. They do not inspect or make
/// guarantees about the compiler's underlying niche representation.
///
/// # Compile-time checks
/// The `const` mode verifies layout during compilation:
///
/// ```
/// use devela::{NonZeroU8, test_size_of};
///
/// test_size_of![const u16 = 2];
/// test_size_of![const u32 = 4|32];
/// test_size_of![const NonZeroU8 = 1|8; niche Option];
/// test_size_of![const u8 = 1|8; niche !Option];
/// ```
///
/// It can be combined with target configuration to lock platform-dependent
/// layouts without requiring those targets to run tests:
///
/// ```
/// # use devela::test_size_of;
/// #[cfg(target_pointer_width = "16")]
/// test_size_of![const usize = 2|16];
///
/// #[cfg(target_pointer_width = "32")]
/// test_size_of![const usize = 4|32];
///
/// #[cfg(target_pointer_width = "64")]
/// test_size_of![const usize = 8|64];
/// ```
///
/// Byte-size, bit-size, and positive niche-size mismatches are expressed as
/// compile-time size equalities, allowing the compiler diagnostic to report
/// both expected and actual sizes. Negative niche-size mismatches produce a
/// compile-time panic describing the violated relation.
///
/// # Runtime assertions
/// The `assert` mode performs the same checks as runtime assertions:
///
/// ```
/// use devela::{NonZeroU8, test_size_of};
///
/// test_size_of![assert u8 = 1];
/// test_size_of![assert u16 = 2|16];
/// test_size_of![assert [u8; 4] = 4];
///
/// test_size_of![assert u8 = 1; niche !Option];
/// test_size_of![assert NonZeroU8 = 1; niche Option];
///
/// test_size_of![assert {
///     u16 = 2|16; niche !Option;
///     u32 = 4|32;
///     [u8; 8] = 8;
///     NonZeroU8 = 1|8; niche Option;
/// }];
/// ```
///
/// # Buffer checks
/// The `check_into` and `line_into` modes write diagnostics into
/// caller-provided byte storage and never panic for size mismatches.
///
/// `check_into` returns:
/// - `true`, the type name, and an empty message on success.
/// - `false`, the type name, and a diagnostic message on failure.
///
/// `line_into` returns:
/// - `true` and a printable success line on success.
/// - `false` and the diagnostic message on failure.
///
/// Messages are truncated to fit the provided buffer.
///
/// ```
/// use devela::test_size_of;
///
/// let mut buf = [0; 128];
///
/// let (ok, ty, msg) = test_size_of![check_into &mut buf; u16 = 2|16];
/// assert!(ok);
/// assert_eq!(ty, "u16");
/// assert_eq!(msg, "");
///
/// let (ok, ty, msg) = test_size_of![check_into &mut buf; u16 = 1];
/// assert!(!ok);
/// assert_eq!(ty, "u16");
/// assert!(msg.contains("size_of::<u16>() byte mismatch"));
/// ```
///
/// ```
/// use devela::test_size_of;
///
/// let mut buf = [0; 128];
/// let (ok, line) = test_size_of![line_into &mut buf; u16 = 2|16];
///
/// assert!(ok);
/// assert_eq!(line, "ok: size_of::<u16>() = 2 bytes (16 bits)");
/// ```
///
/// # Generated tests
/// Without a mode prefix, the macro generates a `#[test]` function:
///
/// ```
/// use devela::{NonZeroU8, test_size_of};
///
/// test_size_of![u32 = 4];
/// test_size_of![u8_no_niche: u8 = 1; niche !Option];
/// test_size_of![nonzero_u8_niche: NonZeroU8 = 1; niche Option];
/// ```
///
/// # Panics
/// The `assert` modes panic when a requested size relation is not satisfied.
///
/// The `probe` mode always panics intentionally and reports the measured size.
///
/// The `const` mode instead fails during constant evaluation or type checking;
/// it does not require execution.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! test_size_of· {
    // Compile-time size assertion with an explicit negative niche-size check.
    (const $ty:ty = $bytes:literal $(| $bits:literal)?
        ; niche ! $wrap:ident $(,)?) => {
        $crate::__test_size_of![%const_size $ty = $bytes $(| $bits)?];
        $crate::__test_size_of![%const_niche_not $ty, $wrap];
    };
    // Compile-time size assertion, optionally checking a niche wrapper.
    (const $ty:ty = $bytes:literal $(| $bits:literal)?
        $(; niche $wrap:ident)? $(,)?) => {
        $crate::__test_size_of![%const_size $ty = $bytes $(| $bits)?];
        $(
            $crate::__test_size_of![%const_niche $ty, $wrap];
        )?
    };

    // Assertion-only mode with an explicit negative niche-size check.
    (assert $ty:ty = $bytes:literal $(| $bits:literal)? ; niche ! $wrap:ident $(,)?) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?];
        $crate::__test_size_of![%assert_niche_not $ty, $wrap];
    }};
    // Panicking check for use inside tests.
    (assert $ty:ty = $bytes:literal $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?) => {{
        let (actual, expected): (usize, usize) = (::core::mem::size_of::<$ty>(), $bytes);
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
        $( $crate::__test_size_of![%assert_niche $ty, $wrap]; )?
    }};
    // Panicking batch check.
    (assert { $($rest:tt)* }) => {{
        $crate::__test_size_of![%assert_batch $($rest)*];
    }};
    // Fallible buffer check with an explicit negative niche-size check.
    (check_into $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?
        ; niche ! $wrap:ident $(,)?) => {{
        $crate::__test_size_of![%check_into [! $wrap] $buf; $ty = $bytes $(| $bits)?]
    }};
    // Fallible buffer check; returns `(ok, type_name, message)`.
    (check_into $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?
        $(; niche $wrap:ident)? $(,)?) => {{
        $crate::__test_size_of![%check_into [$($wrap)?] $buf; $ty = $bytes $(| $bits)?]
    }};
    // Printable buffer check with an explicit negative niche-size check.
    (line_into $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?
        ; niche ! $wrap:ident $(,)?) => {{
        $crate::__test_size_of![%line_into [! $wrap] $buf; $ty = $bytes $(| $bits)?]
    }};
    // Printable buffer check; returns `(ok, line)`.
    (line_into $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?
        $(; niche $wrap:ident)? $(,)?) => {{
        $crate::__test_size_of![%line_into [$($wrap)?] $buf; $ty = $bytes $(| $bits)?]
    }};
    // Probe mode; always panics with the measured size.
    (probe $ty:ty $(,)?) => {{
        let actual: usize = ::core::mem::size_of::<$ty>();
        ::core::panic!("size_of::<{}>() = {} bytes ({} bits)",
            ::core::any::type_name::<$ty>(), actual, actual.saturating_mul(8),
        );
    }};
    // Named generated test with an explicit negative niche-size check.
    ($name:ident : $ty:ty = $bytes:literal $(| $bits:literal)? ; niche ! $wrap:ident $(,)?
        ) => {
        #[cfg(test)] #[test] #[allow(nonstandard_style)]
        fn $name() { $crate::test_size_of![assert $ty = $bytes $(| $bits)?; niche ! $wrap]; }
    };
    // Defines a named unit test for an expected size.
    ($name:ident : $ty:ty = $bytes:literal $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?
        ) => {
        #[cfg(test)] #[test] #[allow(nonstandard_style)]
        fn $name() { $crate::test_size_of![assert $ty = $bytes $(| $bits)? $(; niche $wrap)?]; }
    };
    // Defines a named unit test that probes the measured size.
    ($name:ident : probe $ty:ty $(,)?) => {
        #[cfg(test)] #[test] #[allow(nonstandard_style)]
        fn $name() { $crate::test_size_of![probe $ty]; }
    };
    // Shorthand generated test with an explicit negative niche-size check.
    ($ty:ident = $bytes:literal $(| $bits:literal)? ; niche ! $wrap:ident $(,)?
        ) => { $crate::paste! {
        $crate::test_size_of!([<test_size_of_ $ty>] : $ty = $bytes $(| $bits)?; niche ! $wrap);
    }};
    // Defines `test_size_of_T` for simple identifier types.
    ($ty:ident = $bytes:literal $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?
        ) => { $crate::paste! {
        $crate::test_size_of!([<test_size_of_ $ty>] : $ty = $bytes $(| $bits)? $(; niche $wrap)?);
    }};
    // Defines `test_size_of_T` probe for simple identifier types.
    (probe $ty:ident $(,)?) => { $crate::paste! {
        $crate::test_size_of!([<test_size_of_ $ty>] : probe $ty);
    }};
}
#[doc(inline)]
pub use test_size_of· as test_size_of;
