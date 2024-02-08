// devela::code::asserts
//
//! Additional assertion macros
//

/// Asserts the equality of a series of expressions.
///
/// Similar to [`assert_eq`] but supports more than 2 terms to test for equality.
///
/// # Panics
#[macro_export]
macro_rules! assert_eq_all {
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let first_val = &$first;
        $(
            let rest_val = &$rest;
            assert!(
                first_val == rest_val,
                "Assertion failed: ({}) is not equal to ({})\n  left: {}\n right: {}",
                stringify!($first),
                stringify!($rest),
                first_val,
                rest_val,
            );
        )+
    }};
}
pub use assert_eq_all;

/// Asserts the approximate equality of a series of expressions within `tolerance`.
///
/// This macro should work with any numeric type that supports comparison and
/// subtraction, including signed and unsigned integers and floating-point numbers.
///
/// # Examples
/// ```
/// # use devela::code::assert_approx_eq_all;
/// assert_approx_eq_all![tolerance: 1_u32, 4, 3, 5];
/// assert_approx_eq_all![tolerance: 0.01, 1.0, 1.001, 0.999];
/// ```
/// ```should_panic
/// # use devela::code::assert_approx_eq_all;
/// assert_approx_eq_all![tolerance: 1_u32, 4, 3, 5, 6];
/// ```
#[macro_export]
macro_rules! assert_approx_eq_all {
    (tolerance: $tolerance:expr, $first:expr, $($rest:expr),+ $(,)?) => {{
        let first_val = $first;
        $(
            let rest_val = $rest;
            // Calculate the absolute difference without relying on `abs`:
            let difference = if first_val > rest_val {
                first_val - rest_val
            } else {
                rest_val - first_val
            };
            assert!(
                difference <= $tolerance,
                "Assertion failed: ({}) is not approximately equal to ({})
  left: {}\n right: {}\n toler: {}",
                stringify!($first), stringify!($rest), first_val, rest_val, $tolerance,
            );
        )+
    }};
}
pub use assert_approx_eq_all;
