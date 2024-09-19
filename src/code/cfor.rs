// devela::code::cfor
//
// Original source code by Joachim Enggård Nebel, licensed as MIT,
// https://crates.io/crates/const_for/0.1.4
//
// WAIT: [for-loops in constants](https://github.com/rust-lang/rust/issues/87575)
// IMPROVE: doesn't work in certain circumstances.

/// A for loop that is usable in *compile-time* contexts.
///
/// It aims to work exactly like a normal for loop over a standard exclusive range,
/// eg. `0..10` or `-5..5`. Unfortunately it doesn't support other types of ranges
/// like `..10` or `2..=10`. So generally just use it like a regular for loop.
///
/// `.rev()` and `.step_by(x)` are implemented via macros instead of the
/// non-const iter trait, and makes the loop behave as expected.
///
/// # Examples
/// ```
/// # use devela::code::cfor;
/// let mut a = 0;
/// cfor!(i in 0..5 => {
///     a += i
/// });
/// assert!(a == 10)
/// ```
///
/// This is equivalent to the following regular for loop, except it is usable in const context.
/// ```
/// let mut a = 0;
/// for i in 0..5 {
///     a += i
/// }
/// assert!(a == 10)
/// ```
///
/// ## Custom step size
/// A custom step size can be set:
/// ```
/// # use devela::code::cfor;
/// let mut v = Vec::new();
/// cfor!(i in (0..5).step_by(2) => {
///     v.push(i)
/// });
/// assert!(v == vec![0, 2, 4])
/// ```
/// The loop behaves as if the function was called on the range,
/// including requiring a usize, but it is implemented by a macro.
///
/// ## Reversed
/// Iteration can be reversed:
/// ```
/// # use devela::code::cfor;
/// let mut v = Vec::new();
/// cfor!(i in (0..5).rev() => {
///     v.push(i)
/// });
/// assert!(v == vec![4, 3, 2, 1, 0])
/// ```
/// The loop behaves as if the function was called on the range, but it is implemented by a macro.
///
/// ## Reversed and custom step size
/// It is possible to combine rev and step_by, but each can only be appended once.
/// So the following two examples are the only legal combinations.
/// ```
/// # use devela::code::cfor;
/// // Reverse, then change step size
/// let mut v = Vec::new();
/// cfor!(i in (0..10).rev().step_by(4) => {
///     v.push(i)
/// });
/// assert!(v == vec![9, 5, 1]);
///
/// // Change step size, then reverse
/// let mut v = Vec::new();
/// cfor!(i in (0..10).step_by(4).rev() => {
///     v.push(i)
/// });
/// assert!(v == vec![8, 4, 0])
/// ```
///
/// ## Notes
/// You can use mutable and wildcard variables as the loop variable, and they act as expected.
///
/// ```
/// // Mutable variable
/// # use devela::code::cfor;
/// let mut v = Vec::new();
/// cfor!(mut i in (0..4) => {
///     i *= 2;
///     v.push(i)
/// });
/// assert!(v == vec![0, 2, 4, 6]);
///
/// // Wildcard variable
/// let mut a = 0;
/// cfor!(_ in 0..5 =>
///    a += 1
/// );
/// assert!(a == 5)
/// ```
///
/// The body of the loop can be any statement. This means that the following is legal,
/// even though it is not in a regular for loop.
/// ```
/// # use devela::code::cfor;
/// let mut a = 0;
/// cfor!(_ in 0..5 => a += 1);
///
/// unsafe fn unsafe_function() {}
/// cfor!(_ in 0..5 => unsafe {
///    unsafe_function()
/// });
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! cfor {
    ($var:pat_param in ($range:expr).step_by($step:expr) => $body:stmt) => {
        {
            let _: usize = $step;
            let mut __ite = $range.start;
            let __end = $range.end;
            let mut __is_first = true;
            let __step = $step;

            loop {
                if !__is_first {
                    __ite += __step
                }
                __is_first = false;

                let $var = __ite;

                if __ite >= __end {
                    break
                }

                $body
            }
        }
    };

    ($var:pat_param in ($range:expr).rev().step_by($step:expr) => $body:stmt) => {
        {
            let _: usize = $step;
            let mut __ite = $range.end;
            let __start = $range.start;
            let mut __is_first = true;
            let __step = $step;

            loop {
                if !__is_first {
                    if __step + __start >= __ite {
                        break
                    }
                    __ite -= __step
                }
                __is_first = false;

                if __ite <= __start {
                    break
                }

                // cannot underflow as __ite > __start
                let $var = __ite - 1;

                $body
            }
        }
    };

    ($var:pat_param in ($range:expr).rev() => $body:stmt) => {
        cfor!($var in ($range).rev().step_by(1) => $body)
    };

    ($var:pat_param in ($range:expr).step_by($step:expr).rev() => $body:stmt) => {
        cfor!($var in ($range.start..$range.end - ($range.end - $range.start - 1) % $step)
            .rev().step_by($step) => $body)
    };

    ($var:pat_param in $range:expr => $body:stmt) => {
        cfor!($var in ($range).step_by(1) => $body)
    };
}
#[doc(inline)]
pub use cfor;

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::cfor;
    use crate::data::{vec, Vec};

    macro_rules! validate_loop {
        (@impl $($loop:tt)*) => {
            let mut c_values_hit = Vec::new();
            cfor!(i in $($loop)* => {
                c_values_hit.push(i);
            });

            let mut r_values_hit = Vec::new();
            for i in $($loop)* {
                r_values_hit.push(i);
            };

            assert!(c_values_hit == r_values_hit);
        };

        ($step: expr, $($loop:tt)*) => {
            validate_loop!(@impl ($($loop)*).step_by(1));
            validate_loop!(@impl ($($loop)*).step_by(1).rev());
            validate_loop!(@impl ($($loop)*).rev().step_by(1));
        };

        ($($loop:tt)*) => {
            validate_loop!(@impl $($loop)*);
            validate_loop!(@impl ($($loop)*).rev());

            validate_loop!(1, $($loop)*);
            validate_loop!(2, $($loop)*);
            validate_loop!(3, $($loop)*);
            validate_loop!(8, $($loop)*);
            validate_loop!(15, $($loop)*);
            validate_loop!(17, $($loop)*);
            validate_loop!(45, $($loop)*);
            validate_loop!(150, $($loop)*);
        };
    }

    #[test]
    #[allow(unused_parens, reason = "(0..10)")]
    fn equivalent_to_regular_for() {
        validate_loop!(-10..10);
        validate_loop!(0..10);
        validate_loop!(-10..10);
        validate_loop!((0..10));
        validate_loop!(50..10);
        validate_loop!(-15..-12);
        validate_loop!(-14..0);
        validate_loop!(-100..-50);
        validate_loop!(-14..80);
        validate_loop!(1..80);
    }

    #[test]
    fn capture_range_at_beginning() {
        let mut a = 113;
        cfor!(i in 0..a-100 => {
            a += i;
        });
        let mut b = 113;
        for i in 0..b - 100 {
            b += i;
        }
        assert_eq!(a, b);

        let mut a = 0;
        let mut step = 1;
        cfor!(_ in (0..10).step_by(step) => {
            a += step;
            step += 1;
        });
        let mut b = 0;
        let mut step = 1;
        for _ in (0..10).step_by(step) {
            b += step;
            step += 1;
        }
        assert_eq!(a, b);
    }

    #[test]
    const fn available_in_const() {
        let mut a = 0;

        cfor!(_ in 0..25 => {
            a += 1
        });
        cfor!(_ in (0..25).rev() => {
            a += 1
        });
        cfor!(_ in (0..100).step_by(2) =>
            a += 1
        );

        cfor!(mut i in (0..3) => {
            i += 1;
            a += i
        });

        cfor!(_ in (0..7).rev() => {
            a += 1
        });

        assert!(a == 25 + 25 + 50 + 6 + 7);
    }

    #[test]
    fn no_underflow() {
        cfor!(_ in (0u64..1).rev() => {});
        let mut iterations: u64 = 0;
        cfor!(_ in (i8::MIN..0).rev() => iterations += 1);
        assert_eq!(iterations, 128);
    }

    #[test]
    fn signed_can_go_negative() {
        let mut actual = Vec::new();
        cfor!(i in (-10..11).rev().step_by(5) => actual.push(i));
        assert_eq!(actual, vec![10, 5, 0, -5, -10]);
    }
}
