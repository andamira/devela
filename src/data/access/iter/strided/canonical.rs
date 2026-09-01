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
        test_size_of(StridedIter<u32> = 20|160; niche Option),
        #[cfg(target_pointer_width = "64")]
        test_size_of(StridedIter<u32> = 40|320; niche Option),
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
    /// Safe construction establishes these invariants.
    /// No unsafe code is used.
    pub struct StridedIter: ref (usize)
}

crate::iter_strided! {
    #[doc = crate::_tags!(iterator)]
    /// Iterates mutably over a slice using an affine index progression.
    #[doc = crate::_doc_meta!{
        location("data/access/iter", struct StridedIterMut),
        #[cfg(target_pointer_width = "32")]
        test_size_of(StridedIterMut<u32> = 20|160; niche Option),
        #[cfg(target_pointer_width = "64")]
        test_size_of(StridedIterMut<u32> = 40|320; niche Option),
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
    /// # Lending
    ///
    /// Each yielded exclusive reference is tied to the current mutable
    /// borrow of the iterator. Consequently this type implements the
    /// lending-iterator traits rather than [`Iterator`].
    ///
    /// A yielded reference must cease to be borrowed
    /// before the iterator can advance again.
    ///
    /// No unsafe code is used.
    pub struct StridedIterMut: mut (usize)
}
