// devela/src/code/util/assert/test_size_of/define.rs
//
//! Defines `__test_size_of_report`.
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
