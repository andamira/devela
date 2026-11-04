//
//! Defines [`fmtcat!`].
//

#[cfg(doc)]
use crate::{Digits, Slice};

#[doc = crate::_tags!(fmt)]
/// Writes concatenated formatted data into a byte buffer.
#[doc = crate::_doc_meta!{
    location("text/fmt", macro fmtcat),
}]
/// Expands into sequential write calls using a mutable offset, combining:
/// - String literals: `"text"` — written via [`Slice::copy_str_into`].
/// - String expressions: `@expr` — written via [`Slice::copy_utf8_into`].
/// - Integer expressions: `%expr` — decimal formatting via [`Digits`].
///
/// Decimal formatting supports all signed and unsigned primitive integers. It is
/// `const`-compatible, writes the sign of negative numbers, and writes nothing if
/// the complete number cannot fit. No precompiled number-formatter wrapper
/// is required.
///
/// To use a custom number formatter, prefix the invocation with `with Formatter;`:
/// `fmtcat!(with path::Formatter; buf, pos, ...);`.
/// Every `%expr` is then formatted as `Formatter(expr).write(&mut buf, pos)`.
/// The formatter must be a callable wrapper constructor with a `write` method
/// returning the number of bytes written (or `0` on failure); it should not
/// partially modify the destination on failure. This form also supports floating
/// point or other types when implemented by the chosen formatter.
///
/// # Examples
/// ```
/// use devela::{StringU8, fmtcat};
///
/// let one = "Alice";
/// let score = 42u8;
/// let two = StringU8::<8>::from_str_unchecked("Bob");
///
/// let mut buf = [0u8; 64];
/// let mut pos = 0;
///
/// fmtcat!(buf, pos, "Player ", @one, " scored ", %score, "! Now it's ", @two, "'s turn.\n");
/// assert_eq!(&buf[..pos], b"Player Alice scored 42! Now it's Bob's turn.\n");
/// ```
///
/// A custom formatter can supply its own representation:
/// ```
/// use devela::fmtcat;
/// struct Hex(u8);
/// impl Hex {
///     const fn write(self, buf: &mut [u8], pos: usize) -> usize {
///         if buf.len().saturating_sub(pos) < 4 { return 0; }
///         let hex = b"0123456789ABCDEF";
///         buf[pos] = b'0';
///         buf[pos + 1] = b'x';
///         buf[pos + 2] = hex[(self.0 >> 4) as usize];
///         buf[pos + 3] = hex[(self.0 & 15) as usize];
///         4
///     }
/// }
/// let mut buf = [0u8; 12];
/// let mut pos = 0;
/// fmtcat!(with Hex; buf, pos, "value=", %255u8);
/// assert_eq!(&buf[..pos], b"value=0xFF");
/// ```
///
/// # Notes
/// - When passed a literal offset (`fmtcat!(buf, 0, ...)`) the offset variable
///   is not updated. This also applies to the custom-formatter form.
/// - Numeric expressions are evaluated exactly once.
/// - All writes append sequentially; on insufficient space, literal and string
///   writes retain the behavior of their respective [`Slice`] helpers.
/// - The default `%expr` path supports integers only; use an explicit formatter
///   for floating-point numbers and other representations.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! fmtcat· {
    // Entry point: select a custom formatter for each %expression.
    (with $formatter:path; $buf:ident, $offset:ident, $($elem:tt)*) => {{
        let mut pos = $offset;
        $crate::fmtcat!(@proc [with $formatter] $buf, pos, $($elem)*);
        $offset = pos;
    }};
    (with $formatter:path; $buf:ident, $offset:literal, $($elem:tt)*) => {{
        let mut pos = $offset;
        $crate::fmtcat!(@proc [with $formatter] $buf, pos, $($elem)*);
    }};

    // Entry point: default unsigned-magnitude decimal formatting via Digits.
    ($buf:ident, $offset:ident, $($elem:tt)*) => {{
        let mut pos = $offset;
        $crate::fmtcat!(@proc [digits] $buf, pos, $($elem)*);
        $offset = pos;
    }};
    ($buf:ident, $offset:literal, $($elem:tt)*) => {{
        let mut pos = $offset;
        $crate::fmtcat!(@proc [digits] $buf, pos, $($elem)*);
        let _final_pos = pos;
    }};

    // Recursion terminator.
    (@proc $mode:tt $buf:ident, $pos:ident $(,)?) => {};

    // Literal string.
    (@proc $mode:tt $buf:ident, $pos:ident, $lit:literal , $($rest:tt)*) => {{
        $pos += $crate::Slice::copy_str_into(&mut $buf, $pos, $lit);
        $crate::fmtcat!(@proc $mode $buf, $pos, $($rest)*);
    }};
    (@proc $mode:tt $buf:ident, $pos:ident, $lit:literal) => {{
        $pos += $crate::Slice::copy_str_into(&mut $buf, $pos, $lit);
    }};

    // Number expression.
    (@proc $mode:tt $buf:ident, $pos:ident, %$num:expr , $($rest:tt)*) => {{
        $crate::fmtcat!(@number $mode $buf, $pos, $num);
        $crate::fmtcat!(@proc $mode $buf, $pos, $($rest)*);
    }};
    (@proc $mode:tt $buf:ident, $pos:ident, %$num:expr) => {{
        $crate::fmtcat!(@number $mode $buf, $pos, $num);
    }};

    // Default number writer. abs_diff(0) preserves each integer's width and
    // handles signed minima without overflowing. Preflight makes the write atomic.
    (@number [digits] $buf:ident, $pos:ident, $num:expr) => {{
        let number = $num;
        #[allow(unused_comparisons)] // unsigned types can never be negative
        let negative = number < 0;
        let digits = $crate::Digits(number.abs_diff(0));
        let needed = digits.count_digits10() as usize + negative as usize;
        if needed <= $buf.len().saturating_sub($pos) {
            if negative {
                $buf[$pos] = b'-';
                $pos += 1;
            }
            $pos += digits.write_digits10(&mut $buf, $pos);
        }
    }};

    // Explicit formatter with the conventional const write(buf, pos) contract.
    (@number [with $formatter:path] $buf:ident, $pos:ident, $num:expr) => {{
        $pos += $formatter($num).write(&mut $buf, $pos);
    }};

    // String expression.
    (@proc $mode:tt $buf:ident, $pos:ident, @$s:expr, $($rest:tt)*) => {{
        $pos += $crate::Slice::copy_utf8_into(&mut $buf, $pos, $s.as_bytes());
        $crate::fmtcat!(@proc $mode $buf, $pos, $($rest)*);
    }};
    (@proc $mode:tt $buf:ident, $pos:ident, @$s:expr) => {{
        $pos += $crate::Slice::copy_utf8_into(&mut $buf, $pos, $s.as_bytes());
    }};
}
pub use fmtcat· as fmtcat;

#[cfg(test)]
mod _test {
    use super::fmtcat;

    #[test]
    fn fmtargs() {
        let name = "Alice";
        let score = 42u8;
        let mut buf = [0u8; 64];
        let mut pos = 0;
        fmtcat!(buf, pos,
            "Hello ", @name, ", score=", %score, "!\n"
        );
        assert_eq!(&buf[..pos], b"Hello Alice, score=42!\n");
        // can continue writing
        fmtcat!(buf, pos, "Bye ", @name, "!");
        assert_eq!(&buf[..pos], b"Hello Alice, score=42!\nBye Alice!");
    }
    #[test]
    fn primitive_integers_and_extremes() {
        macro_rules! check {
            ($number:expr, $expected:expr) => {{
                let mut buf = [0u8; 40];
                let mut pos = 0;
                fmtcat!(buf, pos, %$number);
                assert_eq!(&buf[..pos], $expected);
            }};
        }
        check!(i8::MIN, b"-128");
        check!(i16::MIN, b"-32768");
        check!(i32::MIN, b"-2147483648");
        check!(i64::MIN, b"-9223372036854775808");
        check!(i128::MIN, b"-170141183460469231731687303715884105728");
        check!(u8::MAX, b"255");
        check!(u16::MAX, b"65535");
        check!(u32::MAX, b"4294967295");
        check!(u64::MAX, b"18446744073709551615");
        check!(u128::MAX, b"340282366920938463463374607431768211455");
        let signed_expected: &[u8] = match core::mem::size_of::<isize>() {
            2 => b"-32768",
            4 => b"-2147483648",
            8 => b"-9223372036854775808",
            _ => unreachable!(),
        };
        let unsigned_expected: &[u8] = match core::mem::size_of::<usize>() {
            2 => b"65535",
            4 => b"4294967295",
            8 => b"18446744073709551615",
            _ => unreachable!(),
        };
        check!(isize::MIN, signed_expected);
        check!(usize::MAX, unsigned_expected);
    }
    #[test]
    fn integer_write_is_atomic_and_offset_safe() {
        let mut buf = [b'_'; 8];
        let mut pos = 3;
        fmtcat!(buf, pos, %-42i8);
        assert_eq!(pos, 6);
        assert_eq!(&buf, b"___-42__");

        // The signed number does not fit. No bytes or offset change.
        fmtcat!(buf, pos, %-99i8);
        assert_eq!(pos, 6);
        assert_eq!(&buf, b"___-42__");

        // An out-of-range offset must not cause indexing or overflow.
        fmtcat!(buf, 100, %1u8);
        assert_eq!(&buf, b"___-42__");
    }

    #[test]
    fn numeric_expression_evaluated_once() {
        let mut evals = 0;
        let mut buf = [0u8; 8];
        let mut pos = 0;
        fmtcat!(buf, pos, %{ evals += 1; -5i16 });
        assert_eq!(evals, 1);
        assert_eq!(&buf[..pos], b"-5");
    }

    mod custom {
        pub struct Hex(pub u8);
        impl Hex {
            pub const fn write(self, buf: &mut [u8], pos: usize) -> usize {
                if buf.len().saturating_sub(pos) < 4 {
                    return 0;
                }
                let hex = b"0123456789ABCDEF";
                buf[pos] = b'0';
                buf[pos + 1] = b'x';
                buf[pos + 2] = hex[(self.0 >> 4) as usize];
                buf[pos + 3] = hex[(self.0 & 15) as usize];
                4
            }
        }
    }

    #[test]
    fn custom_formatter_with_path() {
        let mut buf = [0u8; 16];
        let mut pos = 0;
        fmtcat!(with custom::Hex; buf, pos, "n=", %255u8, ",", %1u8);
        assert_eq!(&buf[..pos], b"n=0xFF,0x01");
    }
    #[test]
    fn const_compatible_for_both_modes() {
        const DEFAULT: ([u8; 8], usize) = {
            let mut buf = [0u8; 8];
            let mut pos = 0;
            fmtcat!(buf, pos, %i8::MIN);
            (buf, pos)
        };
        const CUSTOM: ([u8; 8], usize) = {
            let mut buf = [0u8; 8];
            let mut pos = 0;
            fmtcat!(with custom::Hex; buf, pos, %0xA5u8);
            (buf, pos)
        };
        assert_eq!(&DEFAULT.0[..DEFAULT.1], b"-128");
        assert_eq!(&CUSTOM.0[..CUSTOM.1], b"0xA5");
    }
}
