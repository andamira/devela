// devela::code::util::asserts
//
//! Additional assertion macros
//

/// Asserts the equality of a series of expressions.
///
/// Similar to [`assert_eq`] but supports more than 2 terms to test for equality.
///
/// # Panics
// TODO
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
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
#[doc(inline)]
pub use assert_eq_all;

/// Asserts the approximate equality of a series of expressions within `tolerance`.
///
/// This macro should work with any numeric type that supports comparison and
/// subtraction, including signed and unsigned integers and floating-point numbers.
///
/// The given `$tolerance` must be a non-negative number.
///
/// # Panics
// TODO
///
/// # Examples
/// The following examples compile:
/// ```
/// # use devela::assert_approx_eq_all;
/// assert_approx_eq_all![tolerance: 0.01, 1.0, 1.001, 0.999]; // up to 0.01 away from 1.0
/// assert_approx_eq_all![tolerance: 1_u32, 4, 3, 5]; // up to 1 away from 4
///
/// assert_approx_eq_all![tolerance: 0.01, -1.0, -1.001, -0.999];
/// assert_approx_eq_all![tolerance: 1_i32, -4, -3, -5];
/// assert_approx_eq_all![tolerance: 0_i32, 3, 3];
/// ```
/// The following examples panic:
/// ```should_panic
/// # use devela::assert_approx_eq_all;
/// assert_approx_eq_all![tolerance: 0.01, 1.0, 1.001, 0.989]; // |0.989 - 1.0| > |0.01|
/// ```
/// ```should_panic
/// # use devela::assert_approx_eq_all;
/// assert_approx_eq_all![tolerance: 1_u32, 4, 3, 5, 6]; // |6 - 4| > |1|
/// ```
/// ```should_panic
/// # use devela::assert_approx_eq_all;
/// assert_approx_eq_all![tolerance: 1_u32, 3, 4, 5]; // |5 - 3| > |1|
/// ```
/// ```should_panic
/// # use devela::assert_approx_eq_all;
/// assert_approx_eq_all![tolerance: -0.01, 1.0, 1.001, 0.999]; // tolerance: -0.01 < 0
/// ```
/// ```should_panic
/// # use devela::assert_approx_eq_all;
/// assert_approx_eq_all![tolerance: -1_i32, 4, 3, 5]; // tolerance: -1 < 0
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! assert_approx_eq_all {
    (tolerance: $tolerance:expr, $first:expr, $($rest:expr),+ $(,)?) => {{
        let first_val = $first;
        $(
            let rest_val = $rest;
            // Calculate the absolute difference without relying on `abs`:
            let difference =
                if first_val > rest_val { first_val - rest_val } else { rest_val - first_val };
            assert!(
                difference <= $tolerance,
                "Assertion failed: ({}) is not approximately equal to ({})
      left: {}\n     right: {}\n tolerance: {}",
                stringify!($first), stringify!($rest), first_val, rest_val, $tolerance,
            );
        )+
    }};
}
#[doc(inline)]
pub use assert_approx_eq_all;
