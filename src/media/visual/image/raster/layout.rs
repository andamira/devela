// devela/src/media/visual/image/raster/layout.rs
//
//! Defines [`RasterLayout`].
//
// > How are pixels represented in memory?

use crate::{Boundary1d, Extent2, is, unwrap};

#[doc = crate::_tags!(image layout)]
/// Describes the extent and memory stepping of raster storage.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RasterLayout = 16|128),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RasterLayout = 24|192),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterLayout {
    /// Logical width and height of the raster, in pixels.
    pub extent: Extent2<u32>,

    /// Boundary corresponding to the first stored row.
    ///
    /// `Upper` is top-down storage. `Lower` is bottom-up storage.
    pub row_start: Boundary1d,

    /// Stored bytes occupied by one pixel.
    ///
    /// This is the storage width, not necessarily the meaningful color depth.
    /// For example, an XRGB8888 pixel stores 4 bytes while carrying 24 color bits.
    pub bytes_per_pixel: u8,

    /// Stored bytes between the start of one row and the start of the next.
    ///
    /// This may be larger than `extent.w() * bytes_per_pixel` when rows include
    /// padding for alignment or backend requirements.
    pub bytes_per_line: usize,
}
impl RasterLayout {
    /// Creates an interleaved raster layout.
    pub const fn interleaved(
        extent: Extent2<u32>,
        bytes_per_pixel: u8,
        bytes_per_line: usize,
        row_start: Boundary1d,
    ) -> Self {
        Self { extent, row_start, bytes_per_pixel, bytes_per_line }
    }
    /// Creates a dense top-down interleaved raster layout.
    pub const fn dense_interleaved(extent: Extent2<u32>, bytes_per_pixel: u8) -> Option<Self> {
        let [width, _height] = extent.dim;
        let Some(bytes_per_line) = (width as usize).checked_mul(bytes_per_pixel as usize) else {
            return None;
        };
        let layout = Self {
            extent,
            row_start: Boundary1d::Upper,
            bytes_per_pixel,
            bytes_per_line,
        };
        is! { layout.is_valid(), Some(layout), None }
    }

    /// Returns whether rows are tightly packed.
    pub const fn is_dense(self) -> bool {
        let [width, _] = self.extent.dim;
        matches!(
            (width as usize).checked_mul(self.bytes_per_pixel as usize),
            Some(bytes) if bytes == self.bytes_per_line
        )
    }
    /// Returns whether this layout describes non-overlapping complete rows.
    ///
    /// Empty rasters are valid. A non-empty raster must have a non-zero stored pixel width,
    /// and its row stride must be at least the number of bytes occupied by one logical row.
    pub const fn is_valid(self) -> bool {
        let [width, height] = self.extent.dim;
        is! { width == 0 || height == 0, return true }
        is! { self.bytes_per_pixel == 0, return false }
        match (width as usize).checked_mul(self.bytes_per_pixel as usize) {
            Some(row_used) => self.bytes_per_line >= row_used,
            None => false,
        }
    }

    /// Returns the stored bytes per scanline.
    pub const fn bytes_per_line(self) -> Option<usize> {
        Some(self.bytes_per_line)
    }
    /// Returns the minimum byte length required by this layout.
    pub const fn min_len_bytes(self) -> Option<usize> {
        is! { !self.is_valid(), return None }
        let [width, height] = self.extent.dim;
        is! { width == 0 || height == 0, return Some(0) }
        let row_used = unwrap![some?(width as usize).checked_mul(self.bytes_per_pixel as usize)];
        let prior_rows = unwrap![some?(height as usize - 1).checked_mul(self.bytes_per_line)];
        prior_rows.checked_add(row_used)
    }
}
