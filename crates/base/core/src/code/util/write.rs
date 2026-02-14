// devela_base_core::code::util::write
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
/// - Updates offset variables automatically when an identifier is provided.
/// - Works with any indexable type (arrays, vectors, etc.).
/// - Supports spreading sequences with `@expr` syntax.
///
/// # Panics
/// Panics if writing would exceed the buffer bounds.
///
/// /// # Behavior
/// - With `+= $offset` syntax, the offset identifier is updated to the new position.
/// - With `$offset` syntax, the offset expression is used as the starting position
///   and is not updated.
///
/// # Example
/// ```
/// # use devela_base_core::write_at;
/// let mut bytes = [0u8; 8];
/// let mut offset = 0;
///
/// // Passing a mutable offset identifier with += prefix, it gets updated
/// write_at!(bytes, +=offset, b'>', b'h', b'e', b'l', b'l', b'o', b'!', b'\0');
/// assert_eq!(offset, 8);
/// assert_eq!(&bytes[0..offset], b">hello!\0");
///
/// // Passing a literal offset, it doesn't get updated
/// write_at!(bytes, 1, b'w', b'o', b'r', b'l', b'd', b'?');
/// assert_eq!(&bytes[1..7], b"world?");
///
/// assert_eq!(&bytes[0..8], b">world?\0");
///
/// // Spread sequences with `@`
/// let mut bytes = [0u8; 12];
/// let mut offset = 0;
/// let hello = "hello";
/// write_at!(bytes, +=offset, @hello.as_bytes(), b' ', @b"world", b'!');
/// assert_eq!(&bytes[0..offset], b"hello world!");
///
/// // Works with any type
/// let mut chars = ['_'; 8];
/// let mut offset = 2;
/// write_at!(chars, +=offset, 'c', 'h', 'a', 'r', 's');
/// assert_eq!(&chars[..], &['_', '_', 'c', 'h', 'a', 'r', 's', '_']);
/// assert_eq!(offset, 7);
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _write_at {
    ($buf:ident, += $offset:ident, $($elem:tt)*) => { #[allow(unused_assignments)] {
        let mut __offset = $offset;
        $crate::write_at!(% $buf, __offset, $($elem)*);
        $offset = __offset;
    }};
    ($buf:ident, $offset:expr, $($elem:tt)*) => { #[allow(unused_assignments)] {
        let mut __offset = $offset;
        $crate::write_at!(% $buf, __offset, $($elem)*)
    }};
    /* private arms */
    (% $buf:ident, $offset:ident, @$seq:expr, $($rest:tt)*) => {{ // spread sequence + more
        let seq = $seq;
        let mut i = 0;
        while i < seq.len() {
            $buf[$offset] = seq[i];
            $offset += 1;
            i += 1;
        }
        $crate::write_at!(% $buf, $offset, $($rest)*);
    }};
    (% $buf:ident, $offset:ident, @$seq:expr) => {{ // last spread sequence
        let seq = $seq;
        let mut i = 0;
        while i < seq.len() {
            $buf[$offset] = seq[i];
            $offset += 1;
            i += 1;
        }
    }};
    (% $buf:ident, $offset:ident, $elem:expr, $($rest:tt)*) => {{ // element + more
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
pub use _write_at as write_at;

#[cfg(test)]
mod tests {
    use super::write_at;
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
        use crate::Slice;

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
}
