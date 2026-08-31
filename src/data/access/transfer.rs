// devela/src/data/access/transfer.rs
//
//! Defines caller-buffered data transfer operations.
//

use crate::{is, unwrap, whilst};

#[doc = crate::_tags!(data)]
/// Describes a sequence of equal-width blocks separated by a fixed stride.
#[doc = crate::_doc_meta!{
    location("data/access", struct StridedBlocks),
    #[cfg(target_pointer_width = "32")]
    test_size_of(StridedBlocks = 16|128; niche !Option),
    #[cfg(target_pointer_width = "64")]
    test_size_of(StridedBlocks = 32|256; niche !Option),
}]
/// Block `k` occupies:
///
/// `start + k * stride .. start + k * stride + block_len`
///
/// for `k` in `0..count`.
///
/// Non-empty blocks do not overlap: `stride >= block_len`.
///
/// This geometry can be used to [`gather_into`][Self::gather_into] contiguous
/// storage or [`scatter_into`][Self::scatter_into] from contiguous storage.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StridedBlocks {
    start: usize,
    stride: usize,
    block_len: usize,
    count: usize,
}
#[rustfmt::skip]
impl StridedBlocks {
    /* constructors */

    /// Creates a validated strided-block transfer geometry.
    ///
    /// An empty geometry (`count == 0`) is always valid.
    ///
    /// Returns `None` if non-empty blocks have zero width, overlap,
    /// or any derived length or offset would overflow `usize`.
    pub const fn new(start: usize, stride: usize, block_len: usize, count: usize) -> Option<Self> {
        is! { count == 0, return Some(Self { start, stride, block_len, count }) }
        is! { block_len == 0 || stride < block_len, return None }
        // Validate both contiguous and strided extents.
        is! { block_len.checked_mul(count).is_none(), return None }
        let last_start = unwrap![some? unwrap![some?
            (count - 1).checked_mul(stride)].checked_add(start)];
        is! { last_start.checked_add(block_len).is_none(), return None }
        Some(Self { start, stride, block_len, count })
    }

    /* queries */

    /// Returns the starting index of the first block.
    pub const fn start(self) -> usize { self.start }

    /// Returns the distance between consecutive block starts.
    pub const fn stride(self) -> usize { self.stride }

    /// Returns the number of elements in each block.
    pub const fn block_len(self) -> usize { self.block_len }

    /// Returns the number of blocks.
    pub const fn count(self) -> usize { self.count }

    /// Returns whether this geometry contains no blocks.
    pub const fn is_empty(self) -> bool { self.count == 0 }

    /// Returns the total number of elements transferred.
    pub const fn transfer_len(self) -> usize {
        if self.count == 0 { 0 } else { self.block_len * self.count }
    }
    /// Returns the minimum length required by the strided slice.
    pub const fn required_len(self) -> usize {
        is! { self.count == 0, 0, self.start + (self.count - 1) * self.stride + self.block_len }
    }
    /// Returns the starting index of block `index`.
    pub const fn block_start(self, index: usize) -> Option<usize> {
        is! { index < self.count, Some(self.start + index * self.stride), None }
    }

    /* ops */

    /// Gathers the strided blocks from `src` contiguously into `dst`.
    ///
    /// Returns the number of elements written, or `None` if either slice
    /// is too short. On failure, `dst` is left unchanged.
    pub const fn gather_into<T: Copy>(self, src: &[T], dst: &mut [T]) -> Option<usize> {
        let transfer_len = self.transfer_len();
        is! { self.required_len() > src.len(), return None }
        is! { transfer_len > dst.len(), return None }
        whilst! { block in 0..self.count; {
            let src_start = self.start + block * self.stride;
            let dst_start = block * self.block_len;
            whilst! { i in 0..self.block_len; {
                dst[dst_start + i] = src[src_start + i];
            }}
        }}
        Some(transfer_len)
    }
    /// Scatters contiguous elements from `src` into the strided blocks of `dst`.
    ///
    /// Returns the number of elements read, or `None` if either slice
    /// is too short. On failure, `dst` is left unchanged.
    pub const fn scatter_into<T: Copy>(self, src: &[T], dst: &mut [T]) -> Option<usize> {
        let transfer_len = self.transfer_len();
        is! { transfer_len > src.len(), return None }
        is! { self.required_len() > dst.len(), return None }
        whilst! { block in 0..self.count; {
            let src_start = block * self.block_len;
            let dst_start = self.start + block * self.stride;
            whilst! { i in 0..self.block_len; {
                dst[dst_start + i] = src[src_start + i];
            }}
        }}
        Some(transfer_len)
    }
}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn geometry() {
        let blocks = StridedBlocks::new(4, 6, 2, 3).unwrap();
        assert_eq!(blocks.start(), 4);
        assert_eq!(blocks.stride(), 6);
        assert_eq!(blocks.block_len(), 2);
        assert_eq!(blocks.count(), 3);
        assert_eq!(blocks.transfer_len(), 6);
        assert_eq!(blocks.required_len(), 18);
        assert_eq!(blocks.block_start(0), Some(4));
        assert_eq!(blocks.block_start(1), Some(10));
        assert_eq!(blocks.block_start(2), Some(16));
        assert_eq!(blocks.block_start(3), None);
    }
    #[test]
    fn rejects_overflowing_geometry() {
        // Contiguous transfer length.
        assert_eq!(StridedBlocks::new(0, usize::MAX, usize::MAX, 2), None);
        // Strided extent.
        assert_eq!(StridedBlocks::new(usize::MAX, 1, 1, 1), None);
        assert_eq!(StridedBlocks::new(0, usize::MAX, 1, 3), None);
        assert_eq!(StridedBlocks::new(0, 1, 0, 1), None); // zero-width
        assert_eq!(StridedBlocks::new(0, 1, 2, 2), None); // overlapping
    }
    #[test]
    fn gather_blocks() {
        let src = [10, 11, 99, 20, 21, 99, 30, 31];
        let mut dst = [0; 6];
        let blocks = StridedBlocks::new(0, 3, 2, 3).unwrap();
        assert_eq!(blocks.gather_into(&src, &mut dst), Some(6));
        assert_eq!(dst, [10, 11, 20, 21, 30, 31]);
    }
    #[test]
    fn gather_elements() {
        let src = [10, 11, 20, 21, 30, 31, 40];
        let mut dst = [0; 4];
        let blocks = StridedBlocks::new(0, 2, 1, 4).unwrap();
        assert_eq!(blocks.gather_into(&src, &mut dst), Some(4),);
        assert_eq!(dst, [10, 20, 30, 40]);
    }
    #[test]
    fn scatter_blocks() {
        let src = [10, 11, 20, 21, 30, 31];
        let mut dst = [99; 8];
        let blocks = StridedBlocks::new(0, 3, 2, 3).unwrap();
        assert_eq!(blocks.scatter_into(&src, &mut dst), Some(6),);
        assert_eq!(dst, [10, 11, 99, 20, 21, 99, 30, 31,]);
    }
    #[test]
    fn gather_scatter_roundtrip() {
        let src = [10, 11, 99, 20, 21, 99, 30, 31];
        let mut packed = [0; 6];
        let mut dst = [99; 8];
        let blocks = StridedBlocks::new(0, 3, 2, 3).unwrap();
        assert_eq!(blocks.gather_into(&src, &mut packed), Some(6),);
        assert_eq!(blocks.scatter_into(&packed, &mut dst), Some(6),);
        assert_eq!(src, dst);
    }
    #[test]
    fn empty_transfer() {
        let mut dst = [7, 8, 9];
        let blocks = StridedBlocks::new(usize::MAX, 0, 0, 0).unwrap();
        assert_eq!(blocks.gather_into::<u8>(&[], &mut dst), Some(0),);
        assert_eq!(dst, [7, 8, 9]);
    }
    #[test]
    fn failure_is_atomic() {
        let src = [1, 2, 3, 4];
        let mut dst = [9; 3];
        let blocks = StridedBlocks::new(0, 2, 1, 4).unwrap();
        assert_eq!(blocks.gather_into(&src, &mut dst), None,);
        assert_eq!(dst, [9; 3]);
    }
}
