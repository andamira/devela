// devela_base_core::sys::mem::view::slice::namespace::chunk

use crate::{Slice, is, lets};

/// # `*chunk*` API methods for subslicing.
#[rustfmt::skip]
impl<T> Slice<T> {
    /// Returns the number of complete `N`-sized chunks in `slice` and the trailing remainder.
    ///
    /// This is the fixed-step primitive for const-friendly traversal.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// let bytes = b"abcdefg";
    /// let (count, rem) = Slice::chunks_exact::<3>(bytes);
    /// assert_eq!(count, 2); // "abc", "def"
    /// assert_eq!(rem, b"g");
    /// ```
    pub const fn chunks_exact<const N: usize>(slice: &[T]) -> (usize, &[T]) {
        lets![full=slice.len()/N, rem_start=full*N];
        (full, Slice::range_from(slice, rem_start))
    }

    /// Mutable counterpart to [`chunks_exact`].
    ///
    /// Returns the number of complete `N`-sized chunks and a mutable remainder slice.
    /// The caller performs any iteration or stepping logic.
    pub const fn chunks_exact_mut<const N: usize>(slice: &mut [T]) -> (usize, &mut [T]) {
        lets![full=slice.len()/N, rem_start=full*N];
        (full, Slice::range_from_mut(slice, rem_start))
    }

    /// Returns the `idx`-th complete chunk of size `N`, or `None` if incomplete.
    ///
    /// This offers direct, index-based access to fixed-width records.
    /// Bounds are checked; no iteration policy is imposed.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// let b = b"abcdefgh";
    /// assert_eq!(Slice::chunk::<3>(b, 0), Some(&b"abc"[..]));
    /// assert_eq!(Slice::chunk::<3>(b, 1), Some(&b"def"[..]));
    /// assert_eq!(Slice::chunk::<3>(b, 2), None); // only "gh" remains, not enough for a full chunk
    /// ```
    pub const fn chunk<const N: usize>(slice: &[T], idx: usize) -> Option<&[T]> {
        lets![start=idx*N, end=start+N];
        is![end <= slice.len(), Some(Slice::range(slice, start, end)), None]
    }

    /// Mutable counterpart to [`chunk`].
    ///
    /// Returns the `idx`-th complete chunk of size `N` as a mutable subslice,
    /// or `None` if the chunk is incomplete.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Slice;
    /// let mut b = *b"abcdef";
    /// if let Some(ch) = Slice::chunk_mut::<3>(&mut b, 1) {
    ///     ch[0] = b'X';
    /// }
    /// assert_eq!(&b, b"abcXef");
    /// ```
    pub const fn chunk_mut<const N: usize>(slice: &mut [T], idx: usize) -> Option<&mut [T]> {
        lets![start=idx*N, end=start+N];
        is![end <= slice.len(), Some(Slice::range_mut(slice, start, end)), None]
    }
}

#[test]
#[cfg(test)]
fn test_chunks() {
    let mut buf = [10, 11, 12, 13, 14, 15, 16];

    // chunks_exact
    let (count, rem) = Slice::chunks_exact::<3>(&buf);
    assert_eq!(count, 2); // [10,11,12] , [13,14,15]
    assert_eq!(rem, &[16]);

    // chunks_exact_mut
    let (count_mut, rem_mut) = Slice::chunks_exact_mut::<3>(&mut buf);
    assert_eq!(count_mut, 2);
    assert_eq!(rem_mut, &mut [16]);

    // chunk
    let c0 = Slice::chunk::<3>(&buf, 0).unwrap();
    let c1 = Slice::chunk::<3>(&buf, 1).unwrap();
    assert_eq!(c0, &[10, 11, 12]);
    assert_eq!(c1, &[13, 14, 15]);
    assert!(Slice::chunk::<3>(&buf, 2).is_none());

    // chunk_mut
    is![let Some(c0m) = Slice::chunk_mut::<3>(&mut buf, 0), c0m[0] = 99];
    assert_eq!(buf[0], 99);
}
