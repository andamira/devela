//
//! Defines [`__test_size_of_report()`], [`__test_size_of!`].
//

/// Formats a byte/bit size mismatch into caller-provided storage.
#[doc(hidden)] #[rustfmt::skip]
pub fn __test_size_of_report<'a>(buf: &'a mut [u8],
    ty: &str, kind: &str, actual: usize, expected: usize,) -> &'a str {
    match kind {
        "bit" => crate::format_buf![? buf,
            "size_of::<{}>() bit mismatch:\n  actual:   {} bits\n  expected: {} bits",
            ty, actual, expected],
        _ => crate::format_buf![? buf,
"size_of::<{}>() byte mismatch:\n  actual:   {} bytes ({} bits)\n  expected: {} bytes ({} bits)",
            ty, actual, actual.saturating_mul(8), expected, expected.saturating_mul(8)],
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __test_size_of· {
    // Checks the byte size, and optionally the bit size.
    //
    // Array-length equality is intentional: on failure rustc reports both
    // the expected and actual values.
    (%const_size $ty:ty = $bytes:literal $(| $bits:literal)?) => {
        const _: [(); $bytes] = [(); ::core::mem::size_of::<$ty>()];
        $(
            const _: [(); $bits] = [(); ::core::mem::size_of::<$ty>() * 8];
        )?
    };
    // Checks that Option<T> preserves T's size.
    (%const_niche $ty:ty, Option) => {
        const _: [(); ::core::mem::size_of::<$ty>()] =
            [(); ::core::mem::size_of::<::core::option::Option<$ty>>()];
    };
    // Checks that Option<T> does NOT preserve T's size.
    (%const_niche_not $ty:ty, Option) => {
        const _: () = {
            if ::core::mem::size_of::<::core::option::Option<$ty>>()
                == ::core::mem::size_of::<$ty>()
            {
                ::core::panic!(concat!("test_size_of!: Option<", stringify!($ty),
                    "> unexpectedly has the same size as ", stringify!($ty)));
            }
        };
    };
    // Reject unsupported positive niche wrappers.
    (%const_niche $ty:ty, $wrap:ident) => {
        ::core::compile_error!(concat!("unsupported `test_size_of!` const niche wrapper `",
            stringify!($wrap), "`; currently supported: `Option`"));
    };
    // Reject unsupported negative niche wrappers.
    (%const_niche_not $ty:ty, $wrap:ident) => {
        ::core::compile_error!(concat!("unsupported `test_size_of!` const niche wrapper `",
            stringify!($wrap), "`; currently supported: `Option`"));
    };
    // Ends batch parsing.
    (%assert_batch) => {};
    // Parses one batch entry with a negative niche-size check.
    (%assert_batch
     $ty:ty = $bytes:literal $(| $bits:literal)? ; niche ! $wrap:ident ; $($rest:tt)*) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?; niche ! $wrap];
        $crate::__test_size_of![%assert_batch $($rest)*];
    }};
    // Parses one batch entry with a niche check.
    (%assert_batch
        $ty:ty = $bytes:literal $(| $bits:literal)? ; niche $wrap:ident ; $($rest:tt)*) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?; niche $wrap];
        $crate::__test_size_of![%assert_batch $($rest)*];
    }};
    // Parses one plain batch entry.
    (%assert_batch $ty:ty = $bytes:literal $(| $bits:literal)? ; $($rest:tt)*) => {{
        $crate::test_size_of![assert $ty = $bytes $(| $bits)?];
        $crate::__test_size_of![%assert_batch $($rest)*];
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
                $crate::__test_size_of![%check_into_bits __buf; $ty, __expected $(, $bits)?];
            if !__ok { (__ok, __ty, __msg) }
            else {
                let (__ok, __msg) =
                    $crate::__test_size_of![%check_into_niche [$($niche)*] __buf; $ty];
                if !__ok { (__ok, __ty, __msg) } else { (true, __ty, "") }
            }
        }
    }};
    // Core printable buffer check.
    (%line_into [$($niche:tt)*] $buf:expr; $ty:ty = $bytes:literal $(| $bits:literal)?) => {{
        let __buf: &mut [u8] = $buf;
        let __bytes: usize = $bytes;
        let (__ok, __ty, __msg) =
            $crate::__test_size_of![%check_into [$($niche)*] __buf; $ty = $bytes $(| $bits)?];
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
#[doc(hidden)]
pub use __test_size_of· as __test_size_of;
