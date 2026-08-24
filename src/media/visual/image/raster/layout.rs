// devela/src/media/visual/image/raster/layout.rs
//
//! Defines [`RasterLayout`].
//
// > How are pixels represented in memory?

use crate::{Boundary1d, Extent2, Position2, is, unwrap};

#[doc = crate::_tags!(image layout)]
/// Describes the extent and memory stepping of raster storage.
#[doc = crate::_doc_meta!{
    location("media/visual/image/raster"),
    #[cfg(target_pointer_width = "32")]
    test_size_of(RasterLayout = 16|128),
    #[cfg(target_pointer_width = "64")]
    test_size_of(RasterLayout = 16|128),
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
    pub bytes_per_line: u32,
}
impl RasterLayout {
    /// Creates an interleaved raster layout.
    pub const fn interleaved(
        extent: Extent2<u32>,
        bytes_per_pixel: u8,
        bytes_per_line: u32,
        row_start: Boundary1d,
    ) -> Self {
        Self { extent, row_start, bytes_per_pixel, bytes_per_line }
    }
    /// Creates a dense top-down interleaved raster layout.
    pub const fn dense_interleaved(extent: Extent2<u32>, bytes_per_pixel: u8) -> Option<Self> {
        let [width, _height] = extent.dim;
        let Some(bytes_per_line) = width.checked_mul(bytes_per_pixel as u32) else {
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
            width.checked_mul(self.bytes_per_pixel as u32),
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
        match width.checked_mul(self.bytes_per_pixel as u32) {
            Some(row_used) => self.bytes_per_line >= row_used,
            None => false,
        }
    }
    /// Returns the minimum byte length required by this layout.
    pub const fn min_len_bytes(self) -> Option<usize> {
        is! { !self.is_valid(), return None }
        let [width, height] = self.extent.dim;
        is! { width == 0 || height == 0, return Some(0) }
        let row_used = width as u64 * self.bytes_per_pixel as u64;
        let prior_rows = (height as u64 - 1) * self.bytes_per_line as u64;
        let len = unwrap![some? prior_rows.checked_add(row_used)];
        is! { len > usize::MAX as u64, None, Some(len as usize) }
    }

    /// Returns the byte offset of logical row `y` in physical storage.
    ///
    /// The offset is measured from the beginning of the backing byte storage.
    /// [`row_start`][Self::row_start] determines whether logical row `0`
    /// is stored first or last.
    ///
    /// Returns `None` when:
    /// - the layout is invalid;
    /// - `y` lies outside the raster;
    /// - or the resulting offset is not representable as a `usize`.
    #[must_use]
    pub const fn row_offset_bytes(self, y: u32) -> Option<usize> {
        is! { !self.is_valid() || y >= self.extent.dim[1], return None }
        let height = self.extent.dim[1];
        let stored_y = match self.row_start {
            Boundary1d::Upper => y,
            Boundary1d::Lower => height - 1 - y,
        };
        let offset = stored_y as u64 * self.bytes_per_line as u64;
        is! { offset > usize::MAX as u64, None, Some(offset as usize) }
    }
    /// Returns the byte offset of the first stored byte of `coord`.
    ///
    /// The input coordinate is always expressed in canonical logical raster
    /// space. Row orientation and padding are resolved by this layout.
    ///
    /// Returns `None` for an invalid layout, an external coordinate,
    /// or an offset that cannot be represented as a `usize`.
    #[must_use]
    pub const fn pixel_offset_bytes(self, coord: Position2<u32>) -> Option<usize> {
        let [x, y] = coord.dim;
        let [width, height] = self.extent.dim;
        is! { x >= width || y >= height, return None }
        let row = unwrap![some? self.row_offset_bytes(y)] as u64;
        let pixel = x as u64 * self.bytes_per_pixel as u64;
        let offset = unwrap![some? row.checked_add(pixel)];
        is! { offset > usize::MAX as u64, None, Some(offset as usize) }
    }
}
