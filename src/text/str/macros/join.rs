// devela::text::str::macros::join
//
//! Defines the [`strjoin!`] macro.
//

#[doc = crate::TAG_TEXT!()]
/// Joins multiple string slices in compile-time.
///
/// # Example
/// ```
/// # use devela::{strjoin, const_assert};
/// const BASE: &str = "path/to";
/// const PART1: &str = "/foo";
/// const PART2: &str = "/bar";
/// const PATH: &str = strjoin!(BASE, PART1, PART2);
/// const_assert![eq_str PATH, "path/to/foo/bar"];
///
/// const REPEATED: &str = strjoin!(repeat: PART1, 3);
/// const_assert![eq_str REPEATED, "/foo/foo/foo"];
/// ```
/// # Features
/// Makes use of the `unsafe_str` feature if available.
//
// - source: https://users.rust-lang.org/t/concatenate-const-strings/51712/7
// - modifications:
//   - make unsafe optional.
//   - support trailing commas.
//   - support the trivial cases.
//   - support more than 2 arguments.
//   - simplify reassignments and loop.
//   - add a new arm to repeat a string.
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! strjoin {
    // trivial cases:
    () => { "" };
    ($A:expr $(,)?) => { $A };
    // variadic case: Reduce to two-argument case:
    ($A:expr, $B:expr, $($rest:expr),+ $(,)?) => {
        $crate::strjoin!($A, $crate::strjoin!($B $(, $rest)+))
    };
    ($A:expr, $B:expr $(,)?) => {{
        const fn combined() -> [u8; LEN] {
            let mut out = [0u8; LEN];
            out = $crate::Slice::<u8>::copy_into_array(out, A.as_bytes(), 0);
            $crate::Slice::<u8>::copy_into_array(out, B.as_bytes(), A.len())
        }
        const A: &str = $A;
        const B: &str = $B;
        const LEN: usize = A.len() + B.len();
        const RESULT: &[u8] = &combined();
        $crate::Str::__utf8_bytes_to_str(RESULT)
    }};
    // Repeat a string slice a constant number of times:
    (repeat: $A:expr, $count:expr $(,)?) => {{
        const fn repeated() -> [u8; LEN] {
            let mut out = [0u8; LEN];
            let mut i = 0;
            while i < COUNT {
                out = $crate::Slice::<u8>::copy_into_array(out, A.as_bytes(), i * A.len());
                i += 1;
            }
            out
        }
        const A: &str = $A;
        const COUNT: usize = $count;
        const LEN: usize = A.len() * COUNT;
        const RESULT: &[u8] = &repeated();
        $crate::Str::__utf8_bytes_to_str(RESULT)
    }};
}
#[doc(inline)]
pub use strjoin;
