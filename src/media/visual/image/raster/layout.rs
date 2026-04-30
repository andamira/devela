// devela::media::visual::image::raster::layout
//
//! Defines [`RasterLayout`].
//
/* layout terminology
   ---------------------------------------------------------------------------
   - `extent` is logical image size, not byte size.
   - `row_start` describes vertical row order only. It is not a full 2D origin.
   - `bytes_per_pixel` is stored pixel width and includes padding fields.
   - `bytes_per_line` is row stride in bytes and may include trailing row padding.
   - The minimum byte length is usually:
       (height - 1) * bytes_per_line + width * bytes_per_pixel
     not simply:
       height * bytes_per_line
*/

use crate::{Boundary1d, Extent2};

/// Describes the extent and memory stepping of raster storage.
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
        let [w, _h] = extent.dim;
        match (w as usize).checked_mul(bytes_per_pixel as usize) {
            Some(bytes_per_line) => Some(Self {
                extent,
                row_start: Boundary1d::Upper,
                bytes_per_pixel,
                bytes_per_line,
            }),
            None => None,
        }
    }

    /// Returns whether rows are tightly packed.
    pub const fn is_dense(self) -> bool {
        let [w, _h] = self.extent.dim;
        self.bytes_per_line == w as usize * self.bytes_per_pixel as usize
    }
    /// Returns the stored bytes per scanline.
    pub const fn bytes_per_line(self) -> Option<usize> {
        Some(self.bytes_per_line)
    }
    /// Returns the minimum byte length required by this layout.
    pub const fn min_len_bytes(self) -> Option<usize> {
        let [w, h] = self.extent.dim;
        if h == 0 {
            return Some(0);
        }
        let Some(row_used) = (w as usize).checked_mul(self.bytes_per_pixel as usize) else {
            return None;
        };
        let Some(prior_rows) = (h as usize - 1).checked_mul(self.bytes_per_line) else {
            return None;
        };
        prior_rows.checked_add(row_used)
    }
}
