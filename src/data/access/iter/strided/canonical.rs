// devela/src/data/access/iter/strided/canonical.rs
//
//! Defines [`StridedIter`], [`StridedIterMut`].
//

crate::iter_strided! {
    #[doc = crate::_tags!(iterator)]
    /// Iterates over a slice using an affine index progression.
    #[doc = crate::_doc_meta!{
        location("data/access/iter", struct StridedIter),
        #[cfg(target_pointer_width = "32")]
        test_size_of(__: StridedIter<u32> = 20|160; niche Option),
        #[cfg(target_pointer_width = "64")]
        test_size_of(__: StridedIter<u32> = 40|320; niche Option),
    }]
    /// This is the immutable counterpart of [`StridedIterMut`].
    ///
    /// Elements are accessed according to:
    ///
    /// `index_k = front + k * stride`
    ///
    /// for increasing `k`, until the inclusive bound `back` is reached.
    ///
    /// The iterator supports forward and backward traversal.
    ///
    /// This type is dimension-agnostic and suitable for:
    /// - Traversing rows, columns, or diagonals of a 2D layout.
    /// - Projecting channels from interleaved buffers (e.g. RGBRGB…).
    /// - Downsampling by stepping every `stride` elements.
    /// - Iterating a collapsed axis of an n-dimensional layout.
    ///
    /// # Invariants
    /// - `stride > 0`.
    /// - `front <= back`, or the iterator is empty.
    /// - All generated indices lie within `storage`.
    ///
    /// If these conditions are violated, advancing or peeking may panic.
    /// No unsafe code is used.
    pub struct StridedIter: ref (usize)
}

crate::iter_strided! {
    #[doc = crate::_tags!(iterator)]
    /// Iterates mutably over a slice using an affine index progression.
    #[doc = crate::_doc_meta!{
        location("data/access/iter", struct StridedIterMut),
        #[cfg(target_pointer_width = "32")]
        test_size_of(__: StridedIterMut<u32> = 20|160; niche Option),
        #[cfg(target_pointer_width = "64")]
        test_size_of(__: StridedIterMut<u32> = 32|256; niche Option),
    }]
    /// This is the mutable counterpart of [`StridedIter`].
    ///
    /// Elements follow:
    ///
    /// `index_k = front + k * stride`
    ///
    /// until the inclusive bound `back` is reached.
    ///
    /// Each call yields an exclusive reference tied to the borrow
    /// of the iterator itself. Forward and backward traversal
    /// are both supported.
    ///
    /// # Invariants
    /// - `stride > 0`.
    /// - `front <= back`, or the iterator is empty.
    /// - All generated indices lie within `storage`.
    ///
    /// # Aliasing
    ///
    /// Even if indices repeat due to improper construction,
    /// simultaneous mutable aliases are prevented because each
    /// yielded reference is tied to `&mut self`.
    ///
    /// If bounds conditions are violated, advancing or peeking may panic.
    /// No unsafe code is used.
    pub struct StridedIterMut: mut (u32)
}
