// devela::code::util::read
//
//! Defines the [`read_at!`] macro.
//

#[doc = crate::_tags!(code mem)]
/// Read elements from a buffer at the given offset.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// Efficiently reads fixed-width and dynamic-width sequences from a buffer using
/// direct indexing, avoiding slice conversion boilerplate while keeping the
/// access pattern explicit.
///
/// - Supports fixed-width unrolled reads with `@N` syntax.
/// - Supports dynamic-width reads into a destination sequence with `@dst`.
/// - Updates offset variables automatically with `+=offset`.
/// - Works with any indexable source and destination sequence.
///
/// # Panics
/// Panics if reading would exceed the source bounds, or if writing into a
/// destination sequence would exceed its bounds.
///
/// # Behavior
/// - With `@N`, returns an array containing `N` elements read from the source.
/// - With `@expr`, writes into `expr[0..dst.len()]` and returns the final offset.
/// - With `+= $offset`, the offset identifier is updated to the new position.
/// - With `$offset`, the offset expression is used as the starting position but
///   is not updated.
///
/// Supported fixed widths are `0..=8`, `16`, `32`, and `64`.
///
/// # Examples
/// ```
/// # use devela::read_at;
/// let bytes = *b"RIFF\x24\0\0\0WAVE";
/// let mut offset = 0;
///
/// // Fixed-width reads return arrays.
/// let id = read_at!(bytes, +=offset, @4);
/// let size = u32::from_le_bytes(read_at!(bytes, +=offset, @4));
/// let wave = read_at!(bytes, +=offset, @4);
///
/// assert_eq!(&id, b"RIFF");
/// assert_eq!(size, 36);
/// assert_eq!(&wave, b"WAVE");
/// assert_eq!(offset, 12);
///
/// // Plain offsets do not mutate external state.
/// let id = read_at!(bytes, 0, @4);
/// assert_eq!(&id, b"RIFF");
///
/// // Dynamic-width reads copy into a destination sequence.
/// let mut out = [0u8; 4];
/// let end = read_at!(bytes, 8, @out);
/// assert_eq!(end, 12);
/// assert_eq!(&out, b"WAVE");
/// ```
/// # See also
/// [`write_at!`] is the dual operation: it scatters sequence elements into a
/// buffer at an offset, while `read_at!` gathers sequence elements from one.
///
/// This pair is useful for small binary codecs, parsers, emitters…
/// where an explicit offset is clearer than introducing a cursor type.
///
/// [`write_at!`]: crate::write_at
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! read_at· {
    ($buf:ident, += $offset:ident, $($elem:tt)*) => {{
        let mut __offset = $offset;
        let __out = $crate::read_at!(% $buf, __offset, $($elem)*);
        $offset = __offset;
        __out
    }};
    ($buf:ident, $offset:expr, $($elem:tt)*) => {{
        let mut __offset = $offset;
        $crate::read_at!(% $buf, __offset, $($elem)*)
    }};
    /* private arms */
    // fixed-width sequence gather
    (% $buf:ident, $offset:ident, @0) => { [] };
    (% $buf:ident, $offset:ident, @1) => { $crate::read_at!(%unr $buf, $offset, 1) };
    (% $buf:ident, $offset:ident, @2) => { $crate::read_at!(%unr $buf, $offset, 2) };
    (% $buf:ident, $offset:ident, @3) => { $crate::read_at!(%unr $buf, $offset, 3) };
    (% $buf:ident, $offset:ident, @4) => { $crate::read_at!(%unr $buf, $offset, 4) };
    (% $buf:ident, $offset:ident, @5) => { $crate::read_at!(%unr $buf, $offset, 5) };
    (% $buf:ident, $offset:ident, @6) => { $crate::read_at!(%unr $buf, $offset, 6) };
    (% $buf:ident, $offset:ident, @7) => { $crate::read_at!(%unr $buf, $offset, 7) };
    (% $buf:ident, $offset:ident, @8) => { $crate::read_at!(%unr $buf, $offset, 8) };
    (% $buf:ident, $offset:ident, @16) => { $crate::read_at!(%unr $buf, $offset, 16) };
    (% $buf:ident, $offset:ident, @32) => { $crate::read_at!(%unr $buf, $offset, 32) };
    (% $buf:ident, $offset:ident, @64) => { $crate::read_at!(%unr $buf, $offset, 64) };
    (% $buf:ident, $off:ident, @$n:literal $($t:tt)*) => {
        compile_error!("unsupported fixed read width; expected 0..=8, 16, 32, or 64")
    };
    (% unr $buf:ident, $offset:ident, $n:tt) => {{
        let __out = $crate::punroll![$n [] |i| $buf[$offset + i]];
        $offset += $n;
        __out
    }};
    // dynamic-width sequence gather into destination
    (% $buf:ident, $offset:ident, @$dst:expr) => {{
        let __dst = &mut $dst;
        $crate::whilst! { i in 0..__dst.len(); { __dst[i] = $buf[$offset]; $offset += 1; }}
        $offset
    }};
}
pub use read_at· as read_at;

#[cfg(any(doctest, test))]
mod tests {
    use super::read_at;

    #[test]
    fn fixed_read() {
        let bytes = *b"RIFF\x24\0\0\0WAVE";
        let mut offset = 0;
        let id = read_at!(bytes, +=offset, @4);
        let size = u32::from_le_bytes(read_at!(bytes, +=offset, @4));
        let wave = read_at!(bytes, +=offset, @4);
        assert_eq!(&id, b"RIFF");
        assert_eq!(size, 36);
        assert_eq!(&wave, b"WAVE");
        assert_eq!(offset, 12);
    }
    #[test]
    fn fixed_read_without_updating_offset() {
        let bytes = *b"abcdef";
        let offset = 1;
        let a = read_at!(bytes, offset, @3);
        let b = read_at!(bytes, offset + 2, @2);
        assert_eq!(offset, 1);
        assert_eq!(&a, b"bcd");
        assert_eq!(&b, b"de");
    }
    #[test]
    const fn const_fixed_read() {
        let bytes = *b"abcdefgh";
        let mut offset = 2;
        let a = read_at!(bytes, +=offset, @4);
        assert![offset == 6];
        assert![a[0] == b'c'];
        assert![a[1] == b'd'];
        assert![a[2] == b'e'];
        assert![a[3] == b'f'];
    }
    #[test]
    fn dynamic_read() {
        let bytes = *b"hello world";
        let mut offset = 6;
        let mut out = [0u8; 5];
        let end = read_at!(bytes, +=offset, @out);
        assert_eq!(end, 11);
        assert_eq!(offset, 11);
        assert_eq!(&out, b"world");
    }
    #[test]
    fn dynamic_read_without_updating_offset() {
        let bytes = *b"hello world";
        let offset = 6;
        let mut out = [0u8; 5];
        let end = read_at!(bytes, offset, @out);
        assert_eq!(end, 11);
        assert_eq!(offset, 6);
        assert_eq!(&out, b"world");
    }
    #[test]
    fn generic_element_type() {
        let chars = ['a', 'b', 'c', 'd', 'e'];
        let mut offset = 1;
        let fixed = read_at!(chars, +=offset, @3);
        assert_eq!(fixed, ['b', 'c', 'd']);
        assert_eq!(offset, 4);
        let mut out = ['_'; 2];
        read_at!(chars, 3, @out);
        assert_eq!(out, ['d', 'e']);
    }
    #[test]
    fn zero_width_read() {
        let _bytes = [1u8, 2, 3];
        let mut offset = 1;
        let empty: [u8; 0] = read_at!(_bytes, +=offset, @0);
        assert_eq!(empty, []);
        assert_eq!(offset, 1);
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn fixed_read_overflow() {
        let bytes = [1u8, 2, 3];
        let _ = read_at!(bytes, 1, @4);
    }
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn dynamic_read_overflow() {
        let bytes = [1u8, 2, 3];
        let mut out = [0u8; 4];
        let _ = read_at!(bytes, 0, @out);
    }
    /**
    ```compile_fail
    let bytes = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let _ = devela::read_at!(bytes, 1, @9);
    ```
    **/
    #[allow(dead_code)]
    fn unsupported_fixed_arity() {}
}
