// devela_base_core::code::util::write
//
//! Defines the [write_at!] macro.
//

/// Write elements to a buffer at the given offset.
///
/// Efficiently writes a sequence of elements using individual assignments,
/// avoiding slice operations while maintaining clean syntax.
///
/// Works with any type that supports indexing using integer primitives.
///
/// # Panics
/// Panics if writing would exceed the buffer bounds.
///
/// # Behavior
/// - When `$offset` is an identifier, it gets updated to the new position
/// - When `$offset` is a literal, it's used as a starting position without updating
///
/// # Example
/// ```
/// # use devela_base_core::write_at;
/// let mut bytes = [0u8; 8];
/// let mut offset = 0;
///
/// // With mutable offset (identifier) - gets updated
/// write_at!(bytes, offset, b'@', b'h', b'e', b'l', b'l', b'o', b'!', b'\0');
/// assert_eq!(offset, 8);
/// assert_eq!(&bytes[0..offset], b"@hello!\0");
///
/// // With literal offset - doesn't update any variable
/// write_at!(bytes, 1, b'w', b'o', b'r', b'l', b'd', b'?');
/// assert_eq!(&bytes[1..7], b"world?");
///
/// assert_eq!(&bytes[0..8], b"@world?\0");
///
/// // Works with any type
/// let mut chars = ['_'; 8];
/// let mut offset = 2;
/// write_at!(chars, offset, 'c', 'h', 'a', 'r', 's');
/// assert_eq!(&chars[..], &['_', '_', 'c', 'h', 'a', 'r', 's', '_']);
/// assert_eq!(offset, 7);
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _write_at {
    ($buf:ident, $offset:literal, $($elem:expr),* $(,)?) => { #[allow(unused_assignments)] {
        let mut offset = $offset;
        $(
            $buf[offset] = $elem;
            offset += 1;
        )*
    }};
    ($buf:ident, $offset:ident, $($elem:expr),* $(,)?) => {#[allow(unused_assignments)] {
        let mut offset = $offset;
        $(
            $buf[offset] = $elem;
            offset += 1;
        )*
        $offset = offset; // when given an identifier, update it
    }};
}
pub use _write_at as write_at;

#[cfg(test)]
#[test]
fn write_at() {
    let mut buffer = [0u8; 20];
    let mut offset = 0;

    write_at!(buffer, offset, b'\x1b', b'P', b'q', b'"');
    write_at!(buffer, offset, b'h', b'e', b'l', b'l', b'o');
    write_at!(buffer, offset, b' ', b'w', b'o', b'r', b'l', b'd');

    assert_eq![offset, 15];
    assert_eq![&buffer[0..offset], b"\x1bPq\"hello world"];
}
#[test]
const fn const_write_at() {
    use crate::Slice;

    let mut buffer = [0u8; 20];
    let start = 3;
    let mut offset = start;

    write_at!(buffer, offset, b'\x1b', b'P', b'q', b'"');
    write_at!(buffer, offset, b'h', b'e', b'l', b'l', b'o');
    write_at!(buffer, offset, b' ', b'w', b'o', b'r', b'l', b'd');

    let result = Slice::range(&buffer, 3, offset);

    assert![offset == start + 15];
    assert![Slice::<u8>::eq(result, b"\x1bPq\"hello world")];
}
#[test]
#[should_panic(expected = "index out of bounds: the len is 4 but the index is 4")]
fn buffer_overflow() {
    let mut buffer = [0u8; 4];
    let mut offset = 0;
    write_at!(buffer, offset, 1, 2, 3, 4, 5);
}
