// devela/src/code/util/assert/test_size_of/define.rs
//
//! Defines [`test_size_of!`].
//

#[doc = crate::_tags!(assert mem)]
/// Tests the stack size of a type.
#[doc = crate::_doc_meta!{
    location("code/util/assert", macro test_size_of),
}]
/// Optionally checks the matching bit count and whether `Option<T>` preserves
/// or changes the stack size of `T`.
///
/// This is a convenience macro around [`core::mem::size_of`] for probing,
/// checking, or locking the expected size of a type.
///
/// It supports both panicking test-oriented modes and caller-buffer modes for
/// fallible diagnostics without allocation.
///
/// # Modes
/// - `assert T = N`: asserts the byte-size inside an existing test.
/// - `assert T = N|B`: also checks the corresponding bit count.
/// - `assert T = N; niche Option`: also checks `Option<T>` has the same size.
/// - `assert T = N; niche !Option`: also checks `Option<T>` has a different size.
/// - `assert { T = N; ... }`: asserts several type sizes.
/// - `check_into buf; T = N`: returns `(ok, type_name, message)`.
/// - `line_into buf; T = N`: returns `(ok, line)` ready to print.
/// - `probe T`: panics with the measured size.
/// - `name: T = N`: generates a named unit test.
/// - `T = N`: generates `test_size_of_T` for simple identifier types.
///
/// The niche suffixes check the `Option<T>` size relation:
/// - `; niche Option`: `size_of::<Option<T>>() == size_of::<T>()`.
/// - `; niche !Option`: `size_of::<Option<T>>() != size_of::<T>()`.
///
/// Without a niche suffix, the `Option<T>` size relation is left unchecked.
///
/// These suffixes test size preservation only; they do not inspect the
/// compiler's niche representation directly.
///
/// # Buffer modes
/// The `check_into` and `line_into` modes write diagnostics into caller-provided
/// byte storage and never panic for size mismatches.
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
/// # Panics
/// The `assert` modes panic when the measured byte-size does not match the
/// expected value.
///
/// They panic when an explicit bit count does not match the expected byte-size
/// converted to bits.
///
/// They panic when `; niche Option` is used and `Option<T>` has a different
/// stack size than `T`, or when `; niche !Option` is used and both have the
/// same stack size.
///
/// The `probe` mode always panics, intentionally, and prints the measured size.
///
/// # Examples
/// Assertion checks:
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
/// Fallible checks:
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
/// Printable checks:
/// ```
/// use devela::test_size_of;
///
/// let mut buf = [0; 128];
///
/// let (ok, line) = test_size_of![line_into &mut buf; u16 = 2|16];
/// assert!(ok);
/// assert_eq!(line, "ok: size_of::<u16>() = 2 bytes (16 bits)");
/// ```
///
/// Generated tests:
/// ```
/// use devela::{NonZeroU8, test_size_of};
///
/// test_size_of![u32 = 4];
/// test_size_of![u8_no_niche: u8 = 1; niche !Option];
/// test_size_of![nonzero_u8_niche: ::NonZeroU8 = 1; niche Option];
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! test_size_of· {
    // Assertion-only mode with an explicit negative niche-size check.
    (assert $ty:ty = $bytes:literal $(| $bits:literal)? ; niche ! $wrap:ident $(,)?) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?];
        $crate::test_size_of![%assert_niche_not $ty, $wrap];
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
        $( $crate::test_size_of![%assert_niche $ty, $wrap]; )?
    }};
    // Panicking batch check.
    (assert { $($rest:tt)* }) => {{
        $crate::test_size_of![%assert_batch $($rest)*];
    }};
    // Fallible buffer check with an explicit negative niche-size check.
    (check_into $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?
        ; niche ! $wrap:ident $(,)?) => {{
        $crate::test_size_of![%check_into [! $wrap] $buf; $ty = $bytes $(| $bits)?]
    }};
    // Fallible buffer check; returns `(ok, type_name, message)`.
    (check_into $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?
        $(; niche $wrap:ident)? $(,)?) => {{
        $crate::test_size_of![%check_into [$($wrap)?] $buf; $ty = $bytes $(| $bits)?]
    }};
    // Printable buffer check with an explicit negative niche-size check.
    (line_into $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?
        ; niche ! $wrap:ident $(,)?) => {{
        $crate::test_size_of![%line_into [! $wrap] $buf; $ty = $bytes $(| $bits)?]
    }};
    // Printable buffer check; returns `(ok, line)`.
    (line_into $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?
        $(; niche $wrap:ident)? $(,)?) => {{
        $crate::test_size_of![%line_into [$($wrap)?] $buf; $ty = $bytes $(| $bits)?]
    }};
    // Probe mode; always panics with the measured size.
    (probe $ty:ty $(,)?) => {{
        let actual: usize = ::core::mem::size_of::<$ty>();
        ::core::panic!("size_of::<{}>() = {} bytes ({} bits)",
            ::core::any::type_name::<$ty>(), actual, actual.saturating_mul(8),
        );
    }};
    // Named generated test with an explicit negative niche-size check.
    ($name:ident : $ty:ty = $bytes:literal $(| $bits:literal)? ; niche ! $wrap:ident $(,)?) => {
        #[cfg(test)] #[test] #[allow(nonstandard_style)]
        fn $name() { $crate::test_size_of![assert $ty = $bytes $(| $bits)?; niche ! $wrap]; }
    };
    // Defines a named unit test for an expected size.
    ($name:ident : $ty:ty = $bytes:literal $(| $bits:literal)? $(; niche $wrap:ident)? $(,)?) => {
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
    /* private arms*/
    // Ends batch parsing.
    (%assert_batch) => {};
    // Parses one batch entry with a negative niche-size check.
    (%assert_batch
     $ty:ty = $bytes:literal $(| $bits:literal)? ; niche ! $wrap:ident ; $($rest:tt)*) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?; niche ! $wrap];
        $crate::test_size_of![%assert_batch $($rest)*];
    }};
    // Parses one batch entry with a niche check.
    (%assert_batch
        $ty:ty = $bytes:literal $(| $bits:literal)? ; niche $wrap:ident ; $($rest:tt)*) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?; niche $wrap];
        $crate::test_size_of![%assert_batch $($rest)*];
    }};
    // Parses one plain batch entry.
    (%assert_batch $ty:ty = $bytes:literal $(| $bits:literal)? ; $($rest:tt)*) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?];
        $crate::test_size_of![%assert_batch $($rest)*];
    }};
    // Panicking negative `Option<T>` niche-size check.
    (%assert_niche_not $ty:ty, Option) => {{
        let (plain, wrapped): (usize, usize) = (
            ::core::mem::size_of::<$ty>(),
            ::core::mem::size_of::<::core::option::Option<$ty>>(),
        );
        if wrapped == plain {
            ::core::panic!(
    "Option size-preservation mismatch for `{}`:\n  size_of::<{}>()         = \
{} bytes ({} bits)\n  size_of::<Option<{}>>() = {} bytes ({} bits)\n  expected: different stack sizes",
                ::core::any::type_name::<$ty>(),
                ::core::any::type_name::<$ty>(),
                plain, plain.saturating_mul(8),
                ::core::any::type_name::<$ty>(),
                wrapped, wrapped.saturating_mul(8),
            );
        }
    }};
    // Rejects unsupported negative niche wrappers.
    (%assert_niche_not $ty:ty, $wrap:ident) => {
        ::core::compile_error!(concat!(
            "unsupported `test_size_of!` niche wrapper `",
            stringify!($wrap),
            "`; currently supported: `Option`"
        ));
    };
    // Panicking niche-size check.
    (%assert_niche $ty:ty, Option) => {{
        let (plain, wrapped): (usize, usize) =
            (::core::mem::size_of::<$ty>(), ::core::mem::size_of::<::core::option::Option<$ty>>());
        if wrapped != plain {
            ::core::panic!(
    "Option size-preservation mismatch for `{}`:\n  size_of::<{}>()         = \
{} bytes ({} bits)\n  size_of::<Option<{}>>() = {} bytes ({} bits)\n  expected: same stack size",
                ::core::any::type_name::<$ty>(), ::core::any::type_name::<$ty>(),
                plain, plain.saturating_mul(8), ::core::any::type_name::<$ty>(),
                wrapped, wrapped.saturating_mul(8),
            );
        }
    }};
    // Rejects unsupported niche wrappers.
    (%assert_niche $ty:ty, $wrap:ident) => {
        ::core::compile_error!( concat!( "unsupported `test_size_of!` niche wrapper `",
                stringify!($wrap), "`; currently supported: `Option`"));
    };
    // Core fallible buffer check.
    (%check_into [$($niche:tt)*] $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?) => {{
        let __buf: &mut [u8] = $buf;
        let __ty: &'static str = ::core::any::type_name::<$ty>();
        let (__actual, __expected): (usize, usize) = (::core::mem::size_of::<$ty>(), $bytes);
        if __actual != __expected {
            (false, __ty,
                $crate::__test_size_of_report(__buf, __ty, "byte", __actual, __expected))
        } else {
            let (__ok, __msg) =
                $crate::test_size_of![%check_into_bits __buf; $ty, __expected $(, $bits)?];
            if !__ok { (__ok, __ty, __msg) }
            else {
                let (__ok, __msg) =
                    $crate::test_size_of![%check_into_niche [$($niche)*] __buf; $ty];
                if !__ok { (__ok, __ty, __msg) } else { (true, __ty, "") }
            }
        }
    }};
    // Core printable buffer check.
    (%line_into [$($niche:tt)*] $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?) => {{
        let __buf: &mut [u8] = $buf;
        let __bytes: usize = $bytes;
        let (__ok, __ty, __msg) =
            $crate::test_size_of![%check_into [$($niche)*] __buf; $ty = $bytes $(| $bits)?];
        if __ok {
            (true, $crate::format_buf![? __buf, "ok: size_of::<{}>() = {} bytes ({} bits)",
                __ty, __bytes, __bytes.saturating_mul(8)])
        } else {
            (false, __msg)
        }
    }};
    // No bit check requested.
    (%check_into_bits $buf:expr; $ty:ty, $bytes:expr) => {{ (true, "") }};
    // Fallible bit-count check.
    (%check_into_bits $buf:expr; $ty:ty, $bytes:expr, $bits:literal) => {{
        let __actual_bits = $bytes.saturating_mul(8);
        let __expected_bits: usize = $bits;
        if __actual_bits != __expected_bits {
            (false, $crate::__test_size_of_report(
                $buf, ::core::any::type_name::<$ty>(), "bit", __actual_bits, __expected_bits))
        } else { (true, "") }
    }};
    // No niche-size check requested.
    (%check_into_niche [] $buf:expr; $ty:ty) => {{
        (true, "")
    }};
    // Fallible positive `Option<T>` niche-size check.
    (%check_into_niche [Option] $buf:expr; $ty:ty) => {{
        let (__plain, __wrapped): (usize, usize) = (
            ::core::mem::size_of::<$ty>(), ::core::mem::size_of::<::core::option::Option<$ty>>(),
        );
        if __wrapped != __plain {
            (
                false,
                $crate::format_buf![? $buf, "Option size-preservation mismatch for `{}`:\n  \
                    size_of::<{}>()         = {} bytes ({} bits)\n  \
                    size_of::<Option<{}>>() = {} bytes ({} bits)\n  \
                    expected: same stack size",
                    ::core::any::type_name::<$ty>(), ::core::any::type_name::<$ty>(),
                    __plain, __plain.saturating_mul(8), ::core::any::type_name::<$ty>(),
                    __wrapped, __wrapped.saturating_mul(8)
                ],
            )
        } else {
            (true, "")
        }
    }};
    // Fallible negative `Option<T>` niche-size check.
    (%check_into_niche [! Option] $buf:expr; $ty:ty) => {{
        let (__plain, __wrapped): (usize, usize) = (
            ::core::mem::size_of::<$ty>(), ::core::mem::size_of::<::core::option::Option<$ty>>(),
        );
        if __wrapped == __plain {
            (
                false,
                $crate::format_buf![? $buf, "Option size-preservation mismatch for `{}`:\n  \
                    size_of::<{}>()         = {} bytes ({} bits)\n  \
                    size_of::<Option<{}>>() = {} bytes ({} bits)\n  \
                    expected: different stack sizes",
                    ::core::any::type_name::<$ty>(), ::core::any::type_name::<$ty>(),
                    __plain, __plain.saturating_mul(8), ::core::any::type_name::<$ty>(),
                    __wrapped, __wrapped.saturating_mul(8)
                ],
            )
        } else {
            (true, "")
        }
    }};
    // Rejects unsupported positive niche wrappers.
    (%check_into_niche [$wrap:ident] $buf:expr; $ty:ty) => {
        ::core::compile_error!(concat!("unsupported `test_size_of!` niche wrapper `",
            stringify!($wrap), "`; currently supported: `Option`"));
    };
    // Rejects unsupported negative niche wrappers.
    (%check_into_niche [! $wrap:ident] $buf:expr; $ty:ty) => {
        ::core::compile_error!(concat!("unsupported `test_size_of!` niche wrapper `",
            stringify!($wrap), "`; currently supported: `Option`"));
    };
}
#[doc(inline)]
pub use test_size_of· as test_size_of;
