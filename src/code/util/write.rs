// devela::code::util::write
//
//! Defines the [write_at!] macro.
//

#[doc = crate::_tags!(code mem)]
/// Write elements into a buffer at an explicit offset.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// Writes individual values, sequences, and UTF-8 scalar bytes using direct indexing.
/// The buffer expression is evaluated once before the write sequence is expanded.
///
/// Returns the final offset after writing.
///
/// # Forms
/// - `write_at!(buf, offset, ...)` writes from `offset` and returns the final offset.
/// - `write_at!(buf, += offset, ...)` also stores the final offset back into `offset`.
/// - `value` writes one value.
/// - `@seq` writes all indexed elements of `seq`, using `seq.len()`.
/// - `@N seq` writes the first `N` indexed elements of `seq` with an unrolled expansion.
/// - `#scalar` encodes a Unicode scalar value as UTF-8 bytes.
///
/// Supported fixed widths are `0..=8`, `16`, `32`, and `64`.
///
/// # Panics
/// Panics if writing exceeds the buffer bounds.
///
/// # Notes
/// - The buffer must be a mutable indexable place.
/// - The `+=` offset must be an assignable place expression.
///
/// # Examples
/// ```
/// # use devela::{Slice, Str, write_at};
/// let mut bytes = [0u8; 8];
/// let mut offset = 0;
///
/// // Passing a mutable offset identifier with += prefix updates it.
/// let end = write_at!(bytes, +=offset, b'>', b'h', b'e', b'l', b'l', b'o', b'!', b'\0');
/// assert_eq!(end, 8);
/// assert_eq!(offset, 8);
/// assert_eq!(&bytes[0..offset], b">hello!\0");
///
/// // Passing a literal offset does not update any external offset,
/// // but the final position is still returned.
/// let end = write_at!(bytes, 1, b'w', b'o', b'r', b'l', b'd', b'?');
/// assert_eq!(end, 7);
/// assert_eq!(&bytes[1..end], b"world?");
///
/// assert_eq!(&bytes[0..8], b">world?\0");
///
/// // Spread sequences with `@`.
/// let mut bytes = [0u8; 12];
/// let mut offset = 0;
/// let hello = "hello";
/// write_at!(bytes, +=offset, @hello.as_bytes(), b' ', @b"world", b'!');
/// assert_eq!(offset, 12);
/// assert_eq!(&bytes[0..offset], b"hello world!");
///
/// // Spread a fixed number of elements with unrolled assignments.
/// let mut header = [0u8; 8];
/// let mut offset = 0;
/// let id = *b"RIFF";
/// let size = 36u32.to_le_bytes();
/// let end = write_at!(header, +=offset, @4 id, @4 size);
/// assert_eq!(end, 8);
/// assert_eq!(offset, 8);
/// assert_eq!(&header[..4], b"RIFF");
/// assert_eq!(&header[4..], &size);
///
/// // Encode Unicode scalar values as UTF-8 bytes with `#`.
/// let mut bytes = [0u8; 6];
/// let mut offset = 0;
/// let ch = 'µ';
/// write_at!(bytes, +=offset, #'µ', #ch, #0x00B5);
/// assert_eq!(offset, 6);
/// assert_eq!(Str::from_utf8(Slice::range_to(&bytes, offset)), Ok("µµµ"));
///
/// // Works with any element type.
/// let mut chars = ['_'; 8];
/// let mut offset = 2;
/// let end = write_at!(chars, +=offset, 'c', 'h', 'a', 'r', 's');
/// assert_eq!(end, 7);
/// assert_eq!(offset, 7);
/// assert_eq!(&chars[..], &['_', '_', 'c', 'h', 'a', 'r', 's', '_']);
/// ```
/// # See also
/// [`read_at!`] gathers values from a buffer using the same explicit-offset
/// pattern that `write_at!` uses to scatter values into one.
///
/// Together they are useful for small binary codecs, parsers, and emitters
/// where fixed offsets are clearer than cursor movement.
///
/// [`ByteCursor`] provides cursor-based access over ordered byte regions.
/// Prefer it when several byte operations form a sequential traversal.
///
/// [`read_at!`]: crate::read_at
/// [`ByteCursor`]: crate::ByteCursor
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! write_at· {
    ($buf:ident, += $offset:expr, $($elem:tt)*) => {{
        #[allow(unused_mut, unused_assignments)]
        {
            let mut __offset = $offset;
            $crate::write_at!(% $buf, __offset, $($elem)*);
            $offset = __offset;
            __offset
        }
    }};
    ($buf:ident, $offset:expr, $($elem:tt)*) => {{
        #[allow(unused_mut, unused_assignments)]
        {
            let mut __offset = $offset;
            $crate::write_at!(% $buf, __offset, $($elem)*);
            __offset
        }
    }};
    ($buf:expr, += $offset:expr, $($elem:tt)*) => {{
        #[allow(unused_mut, unused_assignments)]
        {
            let __buf = &mut $buf;
            let mut __offset = $offset;
            $crate::write_at!(% __buf, __offset, $($elem)*);
            $offset = __offset;
            __offset
        }
    }};
    ($buf:expr, $offset:expr, $($elem:tt)*) => {{
        #[allow(unused_mut, unused_assignments)]
        {
            let __buf = &mut $buf;
            let mut __offset = $offset;
            $crate::write_at!(% __buf, __offset, $($elem)*);
            __offset
        }
    }};
    /* private arms */
    // fixed-width sequence spread
    (% $buf:ident, $off:ident, @0 $seq:expr $(, $($t:tt)*)? ) => {{
        $( $crate::write_at!(% $buf, $off, $($t)*); )?
    }};
    (% $buf:ident, $off:ident, @1 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 1 $($t)*); };
    (% $buf:ident, $off:ident, @2 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 2 $($t)*); };
    (% $buf:ident, $off:ident, @3 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 3 $($t)*); };
    (% $buf:ident, $off:ident, @4 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 4 $($t)*); };
    (% $buf:ident, $off:ident, @5 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 5 $($t)*); };
    (% $buf:ident, $off:ident, @6 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 6 $($t)*); };
    (% $buf:ident, $off:ident, @7 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 7 $($t)*); };
    (% $buf:ident, $off:ident, @8 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 8 $($t)*); };
    (% $buf:ident, $off:ident, @16 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 16 $($t)*); };
    (% $buf:ident, $off:ident, @32 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 32 $($t)*); };
    (% $buf:ident, $off:ident, @64 $($t:tt)*) => { $crate::write_at!(%unr $buf, $off, 64 $($t)*); };
    // this conflicts with @"str".as_bytes() syntax.
    // (% $buf:ident, $off:ident, @$n:literal $($t:tt)*) => {
    //     compile_error!("unsupported fixed write width; expected 0..=8, 16, 32, or 64")
    // };
    (% unr $buf:ident, $off:ident, $n:tt $seq:expr $(, $($rest:tt)*)? ) => {{
        let __seq = &$seq;
        $crate::punroll![$n |i| { $buf[$off] = __seq[i]; $off += 1; }];
        $( $crate::write_at!(% $buf, $off, $($rest)*); )?
    }};
    // dynamic-width sequence spread
    (% $buf:ident, $off:ident, @$seq:expr, $($rest:tt)*) => {{
        let __seq = &$seq;
        $crate::whilst! { i in 0..__seq.len(); { $buf[$off] = __seq[i]; $off += 1; }}
        $crate::write_at!(% $buf, $off, $($rest)*);
    }};
    (% $buf:ident, $off:ident, @$seq:expr) => {{
        let __seq = &$seq;
        $crate::whilst! { i in 0..__seq.len(); { $buf[$off] = __seq[i]; $off += 1; }}
    }};
    // UTF-8 scalar
    (% $buf:ident, $off:ident, #$ch:expr, $($rest:tt)*) => {{
        $crate::write_at!(%utf8_char $buf, $off, $ch);
        $crate::write_at!(% $buf, $off, $($rest)*);
    }};
    (% $buf:ident, $off:ident, #$ch:expr) => {{
        $crate::write_at!(%utf8_char $buf, $off, $ch);
    }};
    (% utf8_char $buf:ident, $off:ident, $ch:expr) => {{
        $off += $crate::__unicode_scalar_write_utf8_at![$buf, $off, $ch];
    }};
    // generic element
    (% $buf:ident, $off:ident, $elem:expr, $($rest:tt)*) => {{
        $buf[$off] = $elem;
        $off += 1;
        $crate::write_at!(% $buf, $off, $($rest)*);
    }};
    (% $buf:ident, $off:ident, $elem:expr) => {{
        $buf[$off] = $elem;
        $off += 1;
    }};
    (% $buf:ident, $off:ident $(,)?) => {};
}
pub use write_at· as write_at;

#[cfg(any(doctest, test))]
mod tests {
    use super::write_at;
    use crate::{Slice, Str};

    #[test]
    fn write_at() {
        let mut buffer = [0u8; 20];
        let mut offset = 0;

        write_at!(buffer, +=offset, b'\x1b', b'P', b'q', b'"');
        write_at!(buffer, +=offset, b'h', b'e', b'l', b'l', b'o');
        write_at!(buffer, +=offset, b' ', b'w', b'o', b'r', b'l', b'd');

        assert_eq![offset, 15];
        assert_eq![&buffer[0..offset], b"\x1bPq\"hello world"];
    }
    #[test]
    const fn const_write_at() {
        let mut buffer = [0u8; 20];
        let start = 3;
        let mut offset = start;

        write_at!(buffer, +=offset, b'\x1b', b'P', b'q', b'"');
        write_at!(buffer, +=offset, b'h', b'e', b'l', b'l', b'o');
        write_at!(buffer, +=offset, b' ', b'w', b'o', b'r', b'l', b'd');

        let result = Slice::range(&buffer, 3, offset);

        assert![offset == start + 15];
        assert![Slice::<u8>::eq(result, b"\x1bPq\"hello world")];
    }
    #[test]
    fn sequence_spread() {
        let bytes = [6, 7, 8];
        let mut buffer = [0u8; 10];
        let mut offset = 0;
        write_at!(buffer, +=offset, 1, 2, 3, 4, 5, @bytes, 9);
        assert_eq![&buffer[0..offset], &[1, 2, 3, 4, 5, 6, 7, 8, 9]];
    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 4 but the index is 4")]
    fn buffer_overflow() {
        let mut buffer = [0u8; 4];
        let mut offset = 0;
        write_at!(buffer, +=offset, 1, 2, 3, 4, 5);
    }
    #[test]
    fn unicode() {
        let mut buf = [0u8; 12];
        let mut offset = 0;
        write_at![buf, +=offset, 0xC2, 0xB5]; // manual
        write_at!(buf, +=offset, @"µ".as_bytes()); // byte loop
        write_at!(buf, +=offset, #'µ'); // UTF-8 scalar
        assert_eq![Str::from_utf8(Slice::range_to(&buf, offset)), Ok("µµµ")];
    }
    #[test]
    fn fixed_unrolled_sequence_spread() {
        let id = *b"RIFF";
        let mut dst = [0u8; 8];
        let end = write_at!(dst, 0, @4 id);
        assert_eq![end, 4];
        assert_eq![&dst[..4], b"RIFF"];
    }
    #[test]
    fn fixed_unrolled_sequence_spread_mut_offset() {
        let id = *b"fmt ";
        let mut dst = [0u8; 8];
        let mut pos = 2;
        let end = write_at!(dst, +=pos, @4 id);
        assert_eq![end, 6];
        assert_eq![pos, 6];
        assert_eq![&dst[2..6], b"fmt "];
    }
    /**
    ```compile_fail
    let src = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut dst = [0u8; 9];
    let _ = devela::write_at!(dst, 0, @9 src);
    ```
    **/
    #[allow(dead_code)]
    fn unsupported_fixed_arity() {}
}
