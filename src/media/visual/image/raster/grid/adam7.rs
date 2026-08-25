// devela/src/media/visual/image/raster/grid/adam7.rs
//
//! Defines Adam7 interlaced raster traversal.
//!
//! Adam7 partitions a raster into seven sparse passes for progressive traversal.
//! This module models only that raster geometry; sample representation,
//! scanline filtering, compression, and image formats are separate concerns.
//

use crate::{Extent2, IteratorFused, StridedBlocks, is, unwrap, whilst};

#[doc = crate::_tags!(image)]
/// One of the seven canonical passes of Adam7 interlacing.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster/grid", struct Adam7Pass),
    test_size_of(Adam7Pass = 5|40; niche !Option),
}]
/// A pass describes a sparse rectangular sampling grid over an image.
/// Its cells begin at [`x_start`][Self::x_start], [`y_start`][Self::y_start]
/// and advance by fixed horizontal and vertical steps.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Adam7Pass {
    index: u8,
    x_start: u8,
    y_start: u8,
    x_step: u8,
    y_step: u8,
}
#[rustfmt::skip]
impl Adam7Pass {
    /// The seven canonical Adam7 passes, in traversal order.
    pub const PASSES: [Self; 7] = [
        Self { index: 1, x_start: 0, y_start: 0, x_step: 8, y_step: 8 },
        Self { index: 2, x_start: 4, y_start: 0, x_step: 8, y_step: 8 },
        Self { index: 3, x_start: 0, y_start: 4, x_step: 4, y_step: 8 },
        Self { index: 4, x_start: 2, y_start: 0, x_step: 4, y_step: 4 },
        Self { index: 5, x_start: 0, y_start: 2, x_step: 2, y_step: 4 },
        Self { index: 6, x_start: 1, y_start: 0, x_step: 2, y_step: 2 },
        Self { index: 7, x_start: 0, y_start: 1, x_step: 1, y_step: 2 },
    ];

    /// Returns the one-based pass index in `1..=7`.
    pub const fn index(self) -> u8 { self.index }
    /// Returns the first image-space x coordinate visited by this pass.
    pub const fn x_start(self) -> u32 { self.x_start as u32 }
    /// Returns the first image-space y coordinate visited by this pass.
    pub const fn y_start(self) -> u32 { self.y_start as u32 }
    /// Returns the image-space horizontal step between pass samples.
    pub const fn x_step(self) -> u32 { self.x_step as u32 }
    /// Returns the image-space vertical step between pass rows.
    pub const fn y_step(self) -> u32 { self.y_step as u32 }

    /// Returns the number of pass columns within `width`.
    pub const fn width(self, width: u32) -> u32 {
        Self::axis_len(width, self.x_start(), self.x_step())
    }
    /// Returns the number of pass rows within `height`.
    pub const fn height(self, height: u32) -> u32 {
        Self::axis_len(height, self.y_start(), self.y_step())
    }
    /// Returns this pass's compact extent within an image of `extent`.
    pub const fn extent(self, extent: Extent2<u32>) -> Extent2<u32> {
        Extent2::new([self.width(extent.dim[0]), self.height(extent.dim[1])])
    }
    const fn axis_len(total: u32, start: u32, step: u32) -> u32 {
        if total <= start { 0 } else { 1 + (total - 1 - start) / step }
    }
}

#[doc = crate::_tags!(image)]
/// One non-empty scanline of an Adam7 pass mapped into image space.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster/grid", struct Adam7Row),
    test_size_of(Adam7Row = 16|128),
}]
/// The row contains [`len`][Self::len] samples beginning at
/// [`x_start`][Self::x_start], separated by [`x_step`][Self::x_step]
/// image cells, all lying on image row [`y`][Self::y].
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Adam7Row {
    pass: Adam7Pass,
    pass_y: u32,
    len: u32,
}
#[rustfmt::skip]
impl Adam7Row {
    /// Returns the Adam7 pass containing this row.
    pub const fn pass(self) -> Adam7Pass { self.pass }
    /// Returns the one-based Adam7 pass index.
    pub const fn pass_index(self) -> u8 { self.pass.index() }
    /// Returns this row's zero-based index within its pass.
    pub const fn pass_y(self) -> u32 { self.pass_y }
    /// Returns the corresponding image-space y coordinate.
    pub const fn y(self) -> u32 { self.pass.y_start() + self.pass_y * self.pass.y_step() }
    /// Returns the image-space x coordinate of pass-local sample `pass_x`.
    ///
    /// Returns `None` when `pass_x` lies outside this row.
    pub const fn x(self, pass_x: u32) -> Option<u32> {
        is! { pass_x < self.len, Some(self.x_start() + pass_x * self.x_step()), None }
    }
    /// Returns the image-space x coordinate of the first sample.
    pub const fn x_start(self) -> u32 { self.pass.x_start() }
    /// Returns the image-space horizontal step between samples.
    pub const fn x_step(self) -> u32 { self.pass.x_step() }
    /// Returns the number of samples in this compact pass row.
    #[allow(clippy::len_without_is_empty)]
    pub const fn len(self) -> u32 { self.len }

    /// Returns this row's samples as strided fixed-width storage blocks.
    ///
    /// Each image-space sample occupies `block_len` consecutive storage elements.
    /// The row's x origin and step are scaled accordingly.
    ///
    /// Use a block length of `1` when each raster cell occupies one typed element.
    ///
    /// Returns `None` if `block_len` is zero
    /// or the scaled storage geometry overflows `usize`.
    pub const fn strided_blocks(self, block_len: usize) -> Option<StridedBlocks> {
        let start = unwrap![some? (self.x_start() as usize).checked_mul(block_len)];
        let stride = unwrap![some? (self.x_step() as usize).checked_mul(block_len)];
        StridedBlocks::new(start, stride, block_len, self.len() as usize)
    }
}

#[doc = crate::_tags!(image iterator)]
/// Iterator over the non-empty scanlines of an Adam7-interlaced image.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster/grid", struct Adam7Rows),
    test_size_of(Adam7Rows = 16|128),
}]
/// Rows are emitted pass by pass, from pass 1 through pass 7,
/// and top-to-bottom within each pass.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Adam7Rows {
    extent: Extent2<u32>,
    pass_idx: u8,
    pass_y: u32,
}
#[rustfmt::skip]
impl Adam7Rows {
    /* constructors */

    /// Creates an Adam7 row iterator over an image of `extent`.
    pub const fn new(extent: Extent2<u32>) -> Self {
        Self { extent, pass_idx: 0, pass_y: 0 }
    }

    /* queries */

    /// Returns the image extent being traversed.
    pub const fn extent(&self) -> Extent2<u32> { self.extent }
    /// Returns whether all Adam7 passes have been traversed.
    #[must_use]
    pub const fn is_finished(&self) -> bool { self.pass_idx >= 7 }
    /// Returns whether no pass rows remain.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.remaining() == 0 }
    /// Returns the exact number of pass rows not yet yielded.
    #[must_use]
    pub const fn remaining(&self) -> u64 {
        let mut remaining = 0_u64;
        whilst! { pass_idx in self.pass_idx,..7; {
            let pass = Adam7Pass::PASSES[pass_idx as usize];
            // A pass with no columns contains no scanlines.
            if pass.width(self.extent.dim[0]) != 0 {
                let height = pass.height(self.extent.dim[1]);
                if pass_idx == self.pass_idx {
                    remaining += height.saturating_sub(self.pass_y) as u64;
                } else {
                    remaining += height as u64;
                }
            }
        }}
        remaining
    }

    /* iteration */

    /// Advances and returns the next non-empty Adam7 pass row.
    #[must_use]
    pub const fn next(&mut self) -> Option<Adam7Row> {
        loop {
            if self.pass_idx >= 7 { return None; }
            let pass = Adam7Pass::PASSES[self.pass_idx as usize];
            let [width, height] = pass.extent(self.extent).dim;
            // A pass with no columns has no scanlines to emit.
            // A completed pass advances to the next one.
            if width == 0 || self.pass_y >= height {
                self.pass_idx += 1;
                self.pass_y = 0;
                continue;
            }
            let row = Adam7Row { pass, pass_y: self.pass_y, len: width };
            self.pass_y += 1;
            return Some(row);
        }
    }
}
impl Iterator for Adam7Rows {
    type Item = Adam7Row;
    fn next(&mut self) -> Option<Self::Item> {
        Self::next(self)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        if remaining > usize::MAX as u64 {
            (usize::MAX, None)
        } else {
            let len = remaining as usize;
            (len, Some(len))
        }
    }
}
impl IteratorFused for Adam7Rows {}

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn rows_8x8() {
        let expected = [
            (1, 0, 0, 1), (2, 0, 0, 1), (3, 0, 4, 2), (4, 0, 0, 2), (4, 1, 4, 2),
            (5, 0, 2, 4), (5, 1, 6, 4), (6, 0, 0, 4), (6, 1, 2, 4), (6, 2, 4, 4),
            (6, 3, 6, 4), (7, 0, 1, 8), (7, 1, 3, 8), (7, 2, 5, 8), (7, 3, 7, 8),
        ];
        let mut rows = Adam7Rows::new(Extent2::new([8, 8]));
        for (pass, pass_y, y, len) in expected {
            let row = rows.next().unwrap();
            assert_eq!(row.pass_index(), pass);
            assert_eq!(row.pass_y(), pass_y);
            assert_eq!(row.y(), y);
            assert_eq!(row.len(), len);
        }
        assert_eq!(rows.next(), None);
        assert!(rows.is_finished());
    }
    #[test]
    fn rows_cover_every_cell_exactly_once() {
        const MAX: usize = 16;
        for height in 0..=MAX as u32 {
            for width in 0..=MAX as u32 {
                let mut seen = [false; MAX * MAX];
                let mut count = 0_u64;
                for row in Adam7Rows::new(Extent2::new([width, height])) {
                    assert!(row.len() > 0);
                    assert!(row.y() < height);
                    for pass_x in 0..row.len() {
                        let x = row.x(pass_x).unwrap();
                        assert!(x < width);
                        let index = row.y() as usize * MAX + x as usize;
                        assert!(!seen[index]);
                        seen[index] = true;
                        count += 1;
                    }
                }
                assert_eq!(count, width as u64 * height as u64);
                for y in 0..height as usize {
                    for x in 0..width as usize {
                        assert!(seen[y * MAX + x]);
                    }
                }
            }
        }
    }
    #[test]
    fn iterator_contract() {
        fn assert_fused<I: IteratorFused>(_iter: I) {}
        let mut rows = Adam7Rows::new(Extent2::new([8, 8]));
        assert_fused(rows);
        assert_eq!(rows.remaining(), 15);
        assert_eq!(rows.size_hint(), (15, Some(15)));
        assert!(rows.next().is_some());
        assert_eq!(rows.remaining(), 14);
        while rows.next().is_some() {}
        assert!(rows.is_empty());
        assert_eq!(rows.remaining(), 0);
        assert_eq!(rows.size_hint(), (0, Some(0)));
        assert_eq!(rows.next(), None);
        assert_eq!(rows.next(), None);
    }
    #[test]
    fn row_gathers_typed_elements_with_strided_blocks() {
        let row = Adam7Rows::new(Extent2::new([8, 8]))
            .find(|row| row.pass_index() == 6 && row.pass_y() == 0)
            .unwrap();
        assert_eq!(row.y(), 0);
        assert_eq!(row.x_start(), 1);
        assert_eq!(row.x_step(), 2);
        assert_eq!(row.len(), 4);
        let blocks = row.strided_blocks(1).unwrap();
        let src = [0, 1, 2, 3, 4, 5, 6, 7];
        let mut dst = [0; 4];
        assert_eq!(blocks.gather_into(&src, &mut dst), Some(4));
        assert_eq!(dst, [1, 3, 5, 7]);
        let bytes = row.strided_blocks(4).unwrap();
        assert_eq!(bytes.start(), 4);
        assert_eq!(bytes.stride(), 8);
        assert_eq!(bytes.block_len(), 4);
        assert_eq!(bytes.count(), 4);
        assert_eq!(bytes.transfer_len(), 16);
    }
}
