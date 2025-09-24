// devela_base_core::sys::mem::slice::namespace::bytes

#[cfg(any(doc, unsafe··))]
use crate::Ptr;
use crate::{Cmp, Slice};

/// # Methods for byte slices.
#[rustfmt::skip]
impl Slice<u8> {
    /* array */

    /// Copies the `src` byte array into `dst` in compile-time.
    ///
    /// # Features
    /// - Uses `Ptr::copy_nonoverlapping` when unsafe operations are allowed.
    pub const fn copy_array<const N: usize>(dst: &mut [u8; N], src: &[u8; N]) {
        #[cfg(any(base_safe_mem, not(unsafe··)))]
        { let mut i = 0; while i < N { dst[i] = src[i]; i += 1; } }

        #[cfg(all(not(base_safe_mem), unsafe··))]
        unsafe { Ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), N); }
    }

    /// Copies all elements from `src` into a fixed-size array starting at `offset`.
    ///
    /// # Panics
    /// Panics if `src.len() + offset > LEN`.
    ///
    /// # Features
    /// - Uses `Ptr::copy_nonoverlapping` when unsafe operations are allowed.
    /// - Falls back to safe element-wise copy otherwise.
    pub const fn copy_array_at<const LEN: usize>(dst: &mut [u8; LEN], src: &[u8], offset: usize) {
        assert!(src.len() + offset <= LEN, "source slice does not fit in destination array");

        #[cfg(any(base_safe_mem, not(unsafe··)))]
        { let mut i = 0; while i < src.len() { dst[offset + i] = src[i]; i += 1; } }

        #[cfg(all(not(base_safe_mem), unsafe··))]
        // SAFETY: Length checked via assert, u8 is Copy, offset + src.len() is bounds-checked
        unsafe { Ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(offset), src.len()); }
    }

    /// A convenience wrapper over [`Slice::copy_array_at`].
    // #[inline(always)] // MAYBE
    // // Auto-select best version
    // macro_rules! copy_array { // IDEA
    //     ($dst:expr, $src:expr, $at:expr) => {
    //         if $dst.len() <= 32 { copied_array_at($dst, $src, $at) }
    //         else { let mut tmp = $dst; copy_array_at(&mut tmp, $src, $at); tmp }
    //     }
    // }
    pub const fn copied_array_at<const LEN: usize>(base: [u8; LEN], src: &[u8], offset: usize)
        -> [u8; LEN] {
        let mut new = base;
        Slice::copy_array_at(&mut new, src, offset);
        new
    }

    /// Copies all elements from `src` into a new fixed-size array.
    ///
    /// # Features
    /// - Uses `Ptr::copy_nonoverlapping` when unsafe operations are allowed
    /// - Falls back to safe element-wise copy otherwise
    ///
    /// # Panics
    /// Panics if `src.len() != LEN`.
    pub const fn to_array<const LEN: usize>(src: &[u8]) -> [u8; LEN] {
        assert!(src.len() == LEN, "source slice length must match destination array length");
        let mut buf = [0; LEN];

        #[cfg(any(base_safe_mem, not(unsafe··)))]
        { let mut i = 0; while i < src.len() { buf[i] = src[i]; i += 1; } }

        #[cfg(all(not(base_safe_mem), unsafe··))]
        // SAFETY: Lengths are equal (checked by assert), u8 is Copy, entire range is bounds-checked
        unsafe { Ptr::copy_nonoverlapping(src.as_ptr(), buf.as_mut_ptr(), src.len()); }

        buf
    }

    /* replace */

    /// Replaces the `old` leading byte with a `new` byte.
    pub const fn replace_leading(slice: &mut [u8], old: u8, new: u8) {
        let mut start = 0;
        while start < slice.len() && slice[start] == old { slice[start] = new; start += 1; }
    }

    /* trim leading */

    /// Returns a subslice without the given leading `byte`s.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_leading(b"000123", b'0'), b"123");
    /// ```
    #[must_use]
    pub const fn trim_leading(slice: &[u8], byte: u8) -> &[u8] {
        let mut first = 0;
        while first < slice.len() && slice[first] == byte { first += 1; }
        Slice::range_from(slice, first)
    }

    /// Returns a subslice without the given leading `byte`s, except of `keep` number of them.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_leading_keep(b"000123", b'0', 0), b"123");
    /// assert_eq!(Slice::trim_leading_keep(b"000123", b'0', 2), b"00123");
    /// assert_eq!(Slice::trim_leading_keep(b"000123", b'0', 10), b"000123");
    /// ```
    #[must_use]
    pub const fn trim_leading_keep(slice: &[u8], byte: u8, keep: usize) -> &[u8] {
        let mut first = 0;
        while first < slice.len() && slice[first] == byte { first += 1; }
        let first = first.saturating_sub(keep);
        Slice::range_from(slice, first)
    }

    /// Returns a subslice without the given leading `byte`s,
    /// ensuring the result has at least `min_len` bytes (if possible).
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_leading_min_len(b"000000", b'0', 1), b"0");
    /// assert_eq!(Slice::trim_leading_min_len(b"000123", b'0', 0), b"123");
    /// assert_eq!(Slice::trim_leading_min_len(b"000123", b'0', 4), b"0123");
    /// assert_eq!(Slice::trim_leading_min_len(b"000123", b'0', 10), b"000123");
    /// ```
    pub const fn trim_leading_min_len(slice: &[u8], byte: u8, min_len: usize) -> &[u8] {
        let mut first = 0;
        while first < slice.len() && slice[first] == byte { first += 1; }
        let first = Cmp(first).min(slice.len().saturating_sub(min_len));
        Slice::range_from(slice, first)
    }

    /* trim trailing*/

    /// Returns a subslice without the given trailing `byte`s.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_trailing(b"123000", b'0'), b"123");
    /// ```
    #[must_use]
    pub const fn trim_trailing(slice: &[u8], byte: u8) -> &[u8] {
        let mut last = slice.len();
        while last > 0 && slice[last - 1] == byte { last -= 1; }
        Slice::range_to(slice, last)
    }

    /// Returns a subslice without the given trailing `byte`s, except of `keep` number of them.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_trailing_keep(b"123000", b'0', 0), b"123");
    /// assert_eq!(Slice::trim_trailing_keep(b"123000", b'0', 2), b"12300");
    /// assert_eq!(Slice::trim_trailing_keep(b"123000", b'0', 10), b"123000");
    /// ```
    #[must_use]
    pub const fn trim_trailing_keep(slice: &[u8], byte: u8, keep: usize) -> &[u8] {
        let mut last = slice.len();
        while last > 0 && slice[last - 1] == byte { last -= 1; }
        let last = slice.len() - (slice.len() - last).saturating_sub(keep);
        Slice::range_to(slice, last)
    }

    /// Returns a subslice without the given trailing `byte`s,
    /// ensuring the result has at least `min_len` bytes (if possible).
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_trailing_min_len(b"000000", b'0', 1), b"0");
    /// assert_eq!(Slice::trim_trailing_min_len(b"123000", b'0', 0), b"123");
    /// assert_eq!(Slice::trim_trailing_min_len(b"123000", b'0', 4), b"1230");
    /// assert_eq!(Slice::trim_trailing_min_len(b"123000", b'0', 10), b"123000");
    /// ```
    pub const fn trim_trailing_min_len(slice: &[u8], byte: u8, min_len: usize) -> &[u8] {
        let mut last = slice.len();
        while last > 0 && slice[last - 1] == byte { last -= 1; }
        let last = slice.len() - Cmp(slice.len() - last).min(slice.len().saturating_sub(min_len));
        Slice::range_to(slice, last)
    }

    /* trim edges */

    /// Returns a subslice without the given leading and trailing `byte`s.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_edges(b"000123000", b'0'), b"123");
    /// ```
    #[must_use] #[inline(always)]
    pub const fn trim_edges(slice: &[u8], byte: u8) -> &[u8] {
        Slice::trim_trailing(Slice::trim_leading(slice, byte), byte)
    }

    /// Returns a subslice without the given leading and trailing `byte`s,
    /// except for `keep` number of them on each side.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_edges_keep(b"000123000", b'0', 0), b"123");
    /// assert_eq!(Slice::trim_edges_keep(b"000123000", b'0', 1), b"01230");
    /// assert_eq!(Slice::trim_edges_keep(b"000123000", b'0', 2), b"0012300");
    /// assert_eq!(Slice::trim_edges_keep(b"000123000", b'0', 10), b"000123000");
    /// ```
    #[must_use] #[inline(always)]
    pub const fn trim_edges_keep(slice: &[u8], byte: u8, keep: usize) -> &[u8] {
        Slice::trim_trailing_keep(Slice::trim_leading_keep(slice, byte, keep), byte, keep)
    }

    /// Returns a subslice without the given leading and edges `byte`s,
    /// ensuring the result has at least `min_len` bytes (if possible), with a left bias.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_edges_min_len_left(b"000123000", b'0', 0), b"123");
    /// assert_eq!(Slice::trim_edges_min_len_left(b"000123000", b'0', 4), b"0123");
    /// assert_eq!(Slice::trim_edges_min_len_left(b"000123000", b'0', 5), b"01230");
    /// assert_eq!(Slice::trim_edges_min_len_left(b"000123000", b'0', 6), b"001230");
    /// assert_eq!(Slice::trim_edges_min_len_left(b"000123000", b'0', 7), b"0012300");
    /// assert_eq!(Slice::trim_edges_min_len_left(b"000123000", b'0', 20), b"000123000");
    /// ```
    pub const fn trim_edges_min_len_left(slice: &[u8], byte: u8, min_len: usize) -> &[u8] {
        let leading_trimmed = Slice::trim_leading(slice, byte);
        let leading_kept = slice.len() - leading_trimmed.len();

        let trailing_trimmed = Slice::trim_trailing(leading_trimmed, byte);
        let trailing_kept = leading_trimmed.len() - trailing_trimmed.len();

        if trailing_trimmed.len() >= min_len { return trailing_trimmed; }
        let bytes_needed = min_len - trailing_trimmed.len();

        // Left bias: give the extra byte to the left side
        let restore_leading = Cmp(leading_kept).min(bytes_needed / 2 + bytes_needed % 2);
        let restore_trailing = Cmp(trailing_kept).min(bytes_needed - restore_leading);
        let start = leading_kept.saturating_sub(restore_leading);
        let end = slice.len() - trailing_kept + restore_trailing;
        Slice::range(slice, start, end)
    }

    /// Returns a subslice without the given leading and edges `byte`s,
    /// ensuring the result has at least `min_len` bytes (if possible), with a right bias.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// assert_eq!(Slice::trim_edges_min_len_right(b"000123000", b'0', 0), b"123");
    /// assert_eq!(Slice::trim_edges_min_len_right(b"000123000", b'0', 4), b"1230");
    /// assert_eq!(Slice::trim_edges_min_len_right(b"000123000", b'0', 5), b"01230");
    /// assert_eq!(Slice::trim_edges_min_len_right(b"000123000", b'0', 6), b"012300");
    /// assert_eq!(Slice::trim_edges_min_len_right(b"000123000", b'0', 7), b"0012300");
    /// assert_eq!(Slice::trim_edges_min_len_right(b"000123000", b'0', 20), b"000123000");
    /// ```
    pub const fn trim_edges_min_len_right(slice: &[u8], byte: u8, min_len: usize) -> &[u8] {
        let leading_trimmed = Slice::trim_leading(slice, byte);
        let leading_kept = slice.len() - leading_trimmed.len();

        let trailing_trimmed = Slice::trim_trailing(leading_trimmed, byte);
        let trailing_kept = leading_trimmed.len() - trailing_trimmed.len();

        if trailing_trimmed.len() >= min_len { return trailing_trimmed; }
        let bytes_needed = min_len - trailing_trimmed.len();

        // Right bias: give the extra byte to the right side
        let restore_trailing = Cmp(trailing_kept).min(bytes_needed / 2 + bytes_needed % 2);
        let restore_leading = Cmp(leading_kept).min(bytes_needed - restore_trailing);
        let start = leading_kept.saturating_sub(restore_leading);
        let end = slice.len() - trailing_kept + restore_trailing;
        Slice::range(slice, start, end)
    }
}
