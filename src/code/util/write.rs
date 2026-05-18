// devela::code::util::write
//
//! Defines the [write_at!] macro.
//

#[doc = crate::_tags!(code mem)]
/// Write elements to a buffer at the given offset.
#[doc = crate::_doc_location!("code/util")]
///
/// Efficiently writes individual elements and sequences to a buffer using direct
/// assignments, avoiding slice operations while maintaining clean syntax.
///
/// Returns the final offset after writing.
///
/// - Updates offset variables automatically when an identifier is provided.
/// - Works with any indexable type (arrays, vectors, etc.).
/// - Supports spreading sequences with `@expr` syntax.
///
/// # Panics
/// Panics if writing would exceed the buffer bounds.
///
/// # Behavior
/// - With `+= $offset` syntax, the offset identifier is updated to the new position,
///   and that final position is returned.
/// - With `$offset` syntax, the offset expression is used as the starting position
///   but is not updated. The final position is still returned.
/// - The return value can be ignored by using the macro as a statement.
///
/// # Examples
/// ```
/// # use devela::write_at;
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
/// // Works with any element type.
/// let mut chars = ['_'; 8];
/// let mut offset = 2;
/// let end = write_at!(chars, +=offset, 'c', 'h', 'a', 'r', 's');
/// assert_eq!(end, 7);
/// assert_eq!(offset, 7);
/// assert_eq!(&chars[..], &['_', '_', 'c', 'h', 'a', 'r', 's', '_']);
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! write_at· {
    ($buf:ident, += $offset:ident, $($elem:tt)*) => {{
        let mut __offset = $offset;
        #[allow(unused_assignments)]
        {
            $crate::write_at!(% $buf, __offset, $($elem)*);
            $offset = __offset;
        }
        __offset
    }};
    ($buf:ident, $offset:expr, $($elem:tt)*) => {{
        let mut __offset = $offset;
        $crate::write_at!(% $buf, __offset, $($elem)*);
        __offset
    }};
    /* private arms */
    (% $buf:ident, $offset:ident, @$seq:expr, $($rest:tt)*) => {{ // spread sequence + rest
        let seq = $seq;
        $crate::whilst! { i in 0..seq.len(); { $buf[$offset] = seq[i]; $offset += 1; }}
        $crate::write_at!(% $buf, $offset, $($rest)*);
    }};
    (% $buf:ident, $offset:ident, @$seq:expr) => {{ // last spread sequence
        let seq = $seq;
        $crate::whilst! { i in 0..seq.len(); { $buf[$offset] = seq[i]; $offset += 1; }}
    }};
    (% $buf:ident, $offset:ident, $elem:expr, $($rest:tt)*) => {{ // element + rest
        $buf[$offset] = $elem;
        $offset += 1;
        $crate::write_at!(% $buf, $offset, $($rest)*);
    }};
    (% $buf:ident, $offset:ident, $elem:expr) => {{ // last element
        $buf[$offset] = $elem;
        $offset += 1;
    }};
    (% $buf:ident, $offset:ident $(,)?) => {};
}
pub use write_at· as write_at;

#[cfg(test)]
mod tests {
    use super::write_at;
    use crate::{Slice, Str};

    #[test]
    fn test_write_at() {
        let mut buffer = [0u8; 20];
        let mut offset = 0;

        write_at!(buffer, +=offset, b'\x1b', b'P', b'q', b'"');
        write_at!(buffer, +=offset, b'h', b'e', b'l', b'l', b'o');
        write_at!(buffer, +=offset, b' ', b'w', b'o', b'r', b'l', b'd');

        assert_eq![offset, 15];
        assert_eq![&buffer[0..offset], b"\x1bPq\"hello world"];
    }
    #[test]
    const fn test_const_write_at() {
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
    #[cfg(test)]
    #[test]
    fn test_sequence_spread() {
        let bytes = [6, 7, 8];
        let mut buffer = [0u8; 10];
        let mut offset = 0;
        write_at!(buffer, +=offset, 1, 2, 3, 4, 5, @bytes, 9);
        assert_eq![&buffer[0..offset], &[1, 2, 3, 4, 5, 6, 7, 8, 9]];
    }
    #[test]
    #[should_panic(expected = "index out of bounds: the len is 4 but the index is 4")]
    fn test_buffer_overflow() {
        let mut buffer = [0u8; 4];
        let mut offset = 0;
        write_at!(buffer, +=offset, 1, 2, 3, 4, 5);
    }
    #[test]
    fn test_unicode() {
        let mut buf = [0u8; 12];
        let mut offset = 0;
        write_at!(buf, +=offset, @"µ".as_bytes()); // current way using a loop
        write_at![buf, +=offset, 0xC2, 0xB5]; // manual way, ideal performance
        // WIP
        // write_at!(buf, +=offset, #'µ');
        // write_at!(buf, +=offset, @>"µ");
        // assert_eq![Str::from_utf8(Slice::range_to(&buf, offset)), Ok("µµµµ")];
        assert_eq![Str::from_utf8(Slice::range_to(&buf, offset)), Ok("µµ")];
    }
}
