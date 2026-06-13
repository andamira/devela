// devela/src/code/util/read.rs
//
//! Defines the [`read_at!`] macro.
//

#[doc = crate::_tags!(code mem)]
/// Read values from a buffer at an explicit offset.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// Reads fixed-width arrays or gathers values into a destination sequence using
/// direct indexing. The source buffer expression is evaluated once before the
/// read operation is expanded.
///
/// Returns either the gathered array or the final offset, depending on the form.
///
/// # Forms
/// - `read_at!(buf, offset, @N)` reads `N` values and returns an array.
/// - `read_at!(buf, += offset, @N)` also advances `offset` by `N`.
/// - `read_at!(buf, offset, @dst)` fills `dst` from index `0..dst.len()` and returns the final offset.
/// - `read_at!(buf, += offset, @dst)` also stores the final offset back into `offset`.
///
/// Supported fixed widths are `0..=8`, `16`, `32`, and `64`.
///
/// # Panics
/// Panics if reading exceeds the source bounds, or if writing into a destination
/// sequence exceeds its bounds.
///
/// # Notes
/// - The source may be any indexable expression.
/// - Dynamic destination reads require `dst` to be a mutable indexable place.
/// - The `+=` offset must be an assignable **place** expression.
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
/// [`write_at!`] scatters values into a buffer using the same explicit-offset
/// pattern that `read_at!` uses to gather values from one.
///
/// Together they are useful for small binary codecs, parsers, and emitters
/// where fixed offsets are clearer than cursor movement.
///
/// [`ByteCursor`] provides cursor-based access over ordered byte regions.
/// Prefer it when several byte operations form a sequential traversal.
///
/// [`write_at!`]: crate::write_at
/// [`ByteCursor`]: crate::ByteCursor
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! read_at· {
    ($buf:ident, += $offset:expr, $($elem:tt)*) => {
        {
            let mut __offset = $offset;
            let __out = $crate::read_at!(% $buf, __offset, $($elem)*);
            $offset = __offset;
            __out
        }
    };
    ($buf:ident, $offset:expr, $($elem:tt)*) => {
        {
            let mut __offset = $offset;
            $crate::read_at!(% $buf, __offset, $($elem)*)
        }
    };
    ($buf:expr, += $offset:expr, $($elem:tt)*) => {
        {
            let __buf = &$buf;
            let mut __offset = $offset;
            let __out = $crate::read_at!(% __buf, __offset, $($elem)*);
            $offset = __offset;
            __out
        }
    };
    ($buf:expr, $offset:expr, $($elem:tt)*) => {
        {
            let __buf = &$buf;
            let mut __offset = $offset;
            $crate::read_at!(% __buf, __offset, $($elem)*)
        }
    };
    /* private arms */
    // fixed-width sequence gather
    (% $buf:ident, $off:ident, @0) => { [] };
    (% $buf:ident, $off:ident, @1) => { $crate::read_at!(%unr $buf, $off, 1) };
    (% $buf:ident, $off:ident, @2) => { $crate::read_at!(%unr $buf, $off, 2) };
    (% $buf:ident, $off:ident, @3) => { $crate::read_at!(%unr $buf, $off, 3) };
    (% $buf:ident, $off:ident, @4) => { $crate::read_at!(%unr $buf, $off, 4) };
    (% $buf:ident, $off:ident, @5) => { $crate::read_at!(%unr $buf, $off, 5) };
    (% $buf:ident, $off:ident, @6) => { $crate::read_at!(%unr $buf, $off, 6) };
    (% $buf:ident, $off:ident, @7) => { $crate::read_at!(%unr $buf, $off, 7) };
    (% $buf:ident, $off:ident, @8) => { $crate::read_at!(%unr $buf, $off, 8) };
    (% $buf:ident, $off:ident, @16) => { $crate::read_at!(%unr $buf, $off, 16) };
    (% $buf:ident, $off:ident, @32) => { $crate::read_at!(%unr $buf, $off, 32) };
    (% $buf:ident, $off:ident, @64) => { $crate::read_at!(%unr $buf, $off, 64) };
    //
    (% $buf:ident, $off:ident, @$n:literal $($t:tt)*) => {
        compile_error!("unsupported fixed read width; expected 0..=8, 16, 32, or 64")
    };
    (% unr $buf:ident, $off:ident, $n:tt) => {{
        let __out = $crate::punroll![$n [] |i| $buf[$off + i]];
        $off += $n;
        __out
    }};
    // dynamic-width sequence gather into destination
    (% $buf:ident, $off:ident, @$dst:expr) => {{
        let __dst = &mut $dst;
        $crate::whilst! { i in 0..__dst.len(); { __dst[i] = $buf[$off]; $off += 1; }}
        $off
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
