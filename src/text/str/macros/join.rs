// devela::text::str::macros::join
//
//! Defines the [`join_str!`] macro.
//

#[doc = crate::TAG_TEXT!()]
/// Joins multiple byte slices or string slices in compile-time.
///
/// # Example
/// ```
/// # use devela::{const_assert, join};
/// /* string slices */
///
/// const SBASE: &str = "path/to";
/// const SPART1: &str = "/foo";
/// const SPART2: &str = "/bar";
/// const SPATH: &str = join!(str: SBASE, SPART1, SPART2);
/// const_assert![eq_str SPATH, "path/to/foo/bar"];
///
/// const SREPEATED: &str = join!(str_repeat: SPART1, 3);
/// const_assert![eq_str SREPEATED, "/foo/foo/foo"];
///
/// const SPARTS: &str = join!(str_separated: "/"; "path", "to", "file");
/// const_assert!(eq_str SPARTS, "path/to/file");
///
/// /* byte slices */
///
/// const BBASE: &[u8] = b"path/to";
/// const BPART1: &[u8] = b"/foo";
/// const BPART2: &[u8] = b"/bar";
/// const BPATH: &[u8] = join!(bytes: BBASE, BPART1, BPART2);
/// const_assert![eq_buf BPATH, b"path/to/foo/bar"];
///
/// const BREPEATED: &[u8] = join!(bytes_repeat: BPART1, 3);
/// const_assert![eq_buf BREPEATED, b"/foo/foo/foo"];
///
/// const BPARTS: &[u8] = join!(bytes_separated: b"/"; b"path", b"to", b"file");
/// const_assert!(eq_buf BPARTS, b"path/to/file");

/// ```
///
/// # Features
/// Makes use of the `unsafe_str` feature if available.
//
// - based on: https://users.rust-lang.org/t/concatenate-const-strings/51712/7
// - modifications:
//   - make unsafe optional.
//   - support trailing commas.
//   - support the trivial cases.
//   - support more than 2 arguments.
//   - support repeating and separators.
//   - support concatenating byte slices.
//   - simplify the reassignments and loops.
#[doc(hidden)]
#[macro_export]
#[rustfmt::skip]
macro_rules! _join {
    /* bytes */

    // trivial cases:
    (bytes: ) => { &[] };
    (bytes: $A:expr $(,)?) => { $A };
    // variadic case (recursively reduce to pairs)
    (bytes: $A:expr, $B:expr, $($rest:expr),+ $(,)?) => {
        $crate::join!(bytes: $A, $crate::join!(bytes: $B, $($rest),+))
    };
    // two args (base case)
    (bytes: $A:expr, $B:expr $(,)?) => {{
        const fn concat() -> [u8; LEN] {
            let mut out = [0u8; LEN];
            out = $crate::Slice::<u8>::copy_into_array(out, A, 0);
            $crate::Slice::<u8>::copy_into_array(out, B, A.len())
        }
        const A: &[u8] = $A;
        const B: &[u8] = $B;
        const LEN: usize = A.len() + B.len();
        &concat()
    }};
    // Repeat a byte slice a constant number of times:
    (bytes_repeat: $A:expr, $count:expr $(,)?) => {{
        const fn repeated() -> [u8; LEN] {
            let mut out = [0u8; LEN];
            let mut i = 0;
            while i < COUNT {
                out = $crate::Slice::<u8>::copy_into_array(out, A, i * A.len());
                i += 1;
            }
            out
        }
        const A: &[u8] = $A;
        const COUNT: usize = $count;
        const LEN: usize = A.len() * COUNT;
        &repeated()
    }};
    (bytes_separated: $sep:expr; $first:expr, $($rest:expr),+ $(,)?) => {{
        const fn join() -> [u8; LEN] {
            let (mut out, mut pos, mut i) = ([0u8; LEN], 0, 0);
            // First part (no preceding separator)
            while i < $first.len() { out[pos] = $first[i]; pos += 1; i += 1; }
            // Subsequent parts with separators
            $(
                let mut j = 0; // Separator
                while j < $sep.len() { out[pos] = $sep[j]; pos += 1; j += 1; }
                let mut k = 0; // Part
                while k < $rest.len() { out[pos] = $rest[k]; pos += 1; k += 1; }
            )+
            out
        }
        const SEP: &[u8] = $sep;
        const LEN: usize = $first.len() $(+ SEP.len() + $rest.len())+;
        &join()
    }};

    /* strings */

    // trivial cases:
    (str: ) => { "" };
    (str: $A:expr $(,)?) => { $A };
    // variadic case: Reduce to two-argument case:
    (str: $A:expr, $B:expr, $($rest:expr),+ $(,)?) => {
        $crate::join!(str: $A, $crate::join!(str: $B $(, $rest)+))
    };
    // two args (base case)
    (str: $A:expr, $B:expr $(,)?) => {{
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
    (str_repeat: $A:expr, $count:expr $(,)?) => {{
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
        const REPEATED: &[u8] = &repeated();
        $crate::Str::__utf8_bytes_to_str(REPEATED)
    }};
    // Join strings with separator
    (str_separated: $sep:expr; $first:expr, $($rest:expr),+ $(,)?) => {{
        const fn join() -> [u8; LEN] {
            let (mut out, mut pos, mut i) = ([0u8; LEN], 0, 0);
            // First part
            let first = $first.as_bytes();
            while i < first.len() { out[pos] = first[i]; pos += 1; i += 1; }
            // Subsequent parts
            $(
                let mut j = 0; // Separator
                let sep = $sep.as_bytes();
                while j < sep.len() { out[pos] = sep[j]; pos += 1; j += 1; }
                let mut k = 0; // Part
                let rest = $rest.as_bytes();
                while k < rest.len() { out[pos] = rest[k]; pos += 1; k += 1; }
            )+
            out
        }
        const SEP: &str = $sep;
        const LEN: usize = $first.len() $(+ SEP.len() + $rest.len())+;
        const RESULT: &[u8] = &join();
        $crate::Str::__utf8_bytes_to_str(RESULT)
    }};
}
#[doc(inline)]
pub use _join as join;

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use crate::{const_assert, join};

    #[test]
    const fn join_bytes() {
        const BASE: &[u8] = b"path/to";
        const PART1: &[u8] = b"/foo";
        const PART2: &[u8] = b"/bar";
        const PATH: &[u8] = join!(bytes: BASE, PART1, PART2);
        const_assert![eq_buf PATH, b"path/to/foo/bar"];

        const REPEATED: &[u8] = join!(bytes_repeat: PART1, 3);
        const_assert![eq_buf REPEATED, b"/foo/foo/foo"];

        const PARTS: &[u8] = join!(bytes_separated: b"/"; b"path", b"to", b"file");
        const_assert!(eq_buf PARTS, b"path/to/file");
    }
    #[test]
    const fn join_str() {
        const BASE: &str = "path/to";
        const PART1: &str = "/foo";
        const PART2: &str = "/bar";
        const PATH: &str = join!(str: BASE, PART1, PART2);
        const_assert![eq_str PATH, "path/to/foo/bar"];

        const REPEATED: &str = join!(str_repeat: PART1, 3);
        const_assert![eq_str REPEATED, "/foo/foo/foo"];

        const PARTS: &str = join!(str_separated: "/"; "path", "to", "file");
        const_assert!(eq_str PARTS, "path/to/file");
    }
}
