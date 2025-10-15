// devela_base_core::code::util::write
//
//! Defines the [write_bytes!] macro.
//

/// Write bytes to a buffer with manual assignments for optimal performance.
///
/// Efficiently writes a sequence of bytes to a buffer using individual assignments,
/// avoiding slice operations while maintaining clean syntax.
///
/// # Panics
/// Panics if writing would exceed the buffer bounds.
///
/// # Example
/// ```
/// # use devela_base_core::write_bytes;
/// let mut buffer = [0u8; 8];
/// let mut offset = 0;
///
/// // It supports a mutable offset to update
/// write_bytes!(buffer, offset, b'@', b'h', b'e', b'l', b'l', b'o', b'!', b'\0');
/// assert_eq!(offset, 8);
/// assert_eq!(&buffer[0..offset], b"@hello!\0");
///
/// // It also supports a literal as the starting offset
/// write_bytes!(buffer, 1, b'w', b'o', b'r', b'l', b'd', b'?');
/// assert_eq!(&buffer[1..7], b"world?");
///
/// assert_eq!(&buffer[0..8], b"@world?\0");
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _write_bytes {
    ($buf:ident, $offset:literal, $($byte:expr),* $(,)?) => { #[allow(unused_assignments)] {
        let mut offset = $offset;
        $(
            $buf[offset] = $byte;
            offset += 1;
        )*
    }};
    ($buf:ident, $offset:ident, $($byte:expr),* $(,)?) => {#[allow(unused_assignments)] {
        let mut offset = $offset;
        $(
            $buf[offset] = $byte;
            offset += 1;
        )*
        $offset = offset; // when given an identifier, update it
    }};
}
pub use _write_bytes as write_bytes;

#[cfg(test)]
#[test]
fn write_bytes() {
    let mut buffer = [0u8; 20];
    let mut offset = 0;

    write_bytes!(buffer, offset, b'\x1b', b'P', b'q', b'"');
    write_bytes!(buffer, offset, b'h', b'e', b'l', b'l', b'o');
    write_bytes!(buffer, offset, b' ', b'w', b'o', b'r', b'l', b'd');

    assert_eq![offset, 15];
    assert_eq![&buffer[0..offset], b"\x1bPq\"hello world"];
}
#[test]
const fn const_write_bytes() {
    use crate::Slice;

    let mut buffer = [0u8; 20];
    let start = 3;
    let mut offset = start;

    write_bytes!(buffer, offset, b'\x1b', b'P', b'q', b'"');
    write_bytes!(buffer, offset, b'h', b'e', b'l', b'l', b'o');
    write_bytes!(buffer, offset, b' ', b'w', b'o', b'r', b'l', b'd');

    let result = Slice::range(&buffer, 3, offset);

    assert![offset == start + 15];
    assert![Slice::<u8>::eq(result, b"\x1bPq\"hello world")];
}
#[test]
#[should_panic(expected = "index out of bounds: the len is 4 but the index is 4")]
fn buffer_overflow() {
    let mut buffer = [0u8; 4];
    let mut offset = 0;
    write_bytes!(buffer, offset, 1, 2, 3, 4, 5);
}
