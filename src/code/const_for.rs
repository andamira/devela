// devela::code::const_for
//
// Original source code by Joachim Enggård Nebel, licensed as MIT,
// https://crates.io/crates/const_for/0.1.2

/// A for loop that is usable in const expressions.
///
/// It aims to work exactly like a normal for loop over a standard exclusive range,
/// eg. `0..10` or `-5..5`.\
/// Unfortunately it doesn't support other types of ranges like `..10` or `2..=10`.
/// So generally just use it like a regular for loop.
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
///
/// A custom step size can be set:
/// ```
/// # use devela::code::cfor;
/// let mut v = Vec::new();
/// cfor!(i in (0..5).step_by(2) {
///     v.push(i)
/// });
/// assert!(v == vec![0, 2, 4])
/// ```
/// The loop behaves as if the function was called on the range, including requiring a usize, but it is implemented by a macro.
///
/// ## Reversed
///
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
///
/// It is possible to combine rev and step_by, but each can only be appended once.
/// So the following two examples are the only legal combinations.
/// ```
/// # use devela::code::cfor;
/// // Reverse, then change step size
/// let mut v = Vec::new();
/// cfor!(i in (0..10).rev().step_by(4) {
///     v.push(i)
/// });
/// assert!(v == vec![9, 5, 1]);
///
/// // Change step size, then reverse
/// let mut v = Vec::new();
/// cfor!(i in (0..10).step_by(4).rev() {
///     v.push(i)
/// });
/// assert!(v == vec![8, 4, 0])
/// ```
#[cfg_attr(feature = "nightly", doc(cfg(feature = "code")))]
#[macro_export]
macro_rules! cfor {
    ($var:ident in ($range:expr).step_by($step:expr) $body:block) => {
        {
            let _: usize = $step;
            let mut $var = $range.start;
            let mut __is_first = true;

            loop {
                if !__is_first {
                    $var += $step
                }
                __is_first = false;

                let $var = $var;

                if $var >= $range.end {
                    break
                }

                $body
            }
        }
    };

    ($var:ident in ($range:expr).rev().step_by($step:expr) $body:block) => {
        {
            let _: usize = $step;
            let mut $var = $range.end - 1;
            let mut __is_first = true;

            loop {
                if !__is_first {
                    $var -= $step
                }
                __is_first = false;

                if $var < $range.start {
                    break
                }

                $body
            }
        }
    };

    ($var:ident in ($range:expr).rev() => $body:block) => {
        cfor!($var in ($range).rev().step_by(1) $body)
    };

    ($var:ident in ($range:expr).step_by($step:expr).rev() $body:block) => {
        // A little janky, but imitates the chained functions
        cfor!($var in ($range.start..$range.end - ($range.end - $range.start - 1) % $step).rev().step_by($step) $body)
    };

    ($var:ident in $range:expr => $body:block) => {
        cfor!($var in ($range).step_by(1) $body)
    };
}
pub use cfor;
