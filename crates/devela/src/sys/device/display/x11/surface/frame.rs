//
//! Defines [`XSurfaceFrame`].
//

#[cfg(feature = "image")]
use crate::{Boundary1d, RasterLayout, ext, unwrap};
use crate::{Position2, XImageFormat, XImageMode, XImageStore, XSurface, XVisualFormat, is};

#[doc = crate::_tags!(unix runtime)]
/// Borrowed mutable view of the retained X11 presentation surface.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11", struct XSurfaceFrame),
    #[cfg(target_pointer_width = "64")]
    test_size_of(XSurfaceFrame<'_> = 32|256; niche Option),
}]
/// Provides direct access to the pixel storage used for one frame,
/// backed by ordinary CPU memory or, when available, MIT-SHM.
///
/// Its geometry and storage accessors describe the native X11 image layout,
/// while its pixel methods provide format-aware direct writes.
///
/// This is the X11 direct-surface path:
/// rendering here avoids an intermediate scene-to-surface copy
/// and is useful when backend-specific performance or surface control matters.
///
/// # Features
///
/// With the `image` feature, `raster_layout` additionally exposes
/// the storage through devela's generic raster-layout vocabulary.
pub struct XSurfaceFrame<'a> {
    surface: &'a mut XSurface,
    image_format: XImageFormat,
    visual_format: XVisualFormat,
}
#[rustfmt::skip]
impl<'a> XSurfaceFrame<'a> {
    pub(crate) const fn _new(
        surface: &'a mut XSurface,
        image_format: XImageFormat,
        visual_format: XVisualFormat,
    ) -> Self {
        Self { surface, image_format, visual_format }
    }

    /* geometry / storage */

    /// Returns the surface width in pixels.
    #[must_use]
    pub const fn width(&self) -> u16 { self.surface.width }

    /// Returns the surface height in pixels.
    #[must_use]
    pub const fn height(&self) -> u16 { self.surface.height }

    /// Returns the surface pixel depth in bits.
    #[must_use]
    pub const fn depth(&self) -> u8 { self.surface.depth }

    /// Returns the number of stored bits per pixel.
    #[must_use]
    pub const fn bits_per_pixel(&self) -> u8 { self.image_format.bits_per_pixel }

    /// Returns the number of stored bytes per pixel, when byte-aligned.
    #[must_use]
    pub const fn bytes_per_pixel(&self) -> Option<u8> {
        let bpp = self.bits_per_pixel();
        is! { bpp.is_multiple_of(8), Some(bpp / 8), None }
    }

    /// Returns the byte stride between consecutive rows.
    #[must_use]
    pub const fn bytes_per_line(&self) -> u32 { self.image_format.bytes_per_line(self.width()) }

    /// Returns the active surface backing mode.
    pub const fn mode(&self) -> XImageMode { self.surface.mode() }

    #[must_use]
    /// Returns whether each row has no backend padding.
    pub const fn is_tight_rows(&self) -> bool {
        let bits = self.width() as u32 * self.bits_per_pixel() as u32;
        bits.div_ceil(8) == self.bytes_per_line()
    }

    /// Returns this surface's byte-addressable layout as a [`RasterLayout`].
    ///
    /// The layout describes the pixel extent, stored bytes per pixel,
    /// row stride, and upper-first row orientation.
    /// It does not describe the X11 visual or channel encoding.
    ///
    /// Returns `None` when the stored pixel width is not byte-aligned.
    #[must_use]
    #[cfg(feature = "image")]
    pub const fn raster_layout(&self) -> Option<RasterLayout> {
        let bytes_per_pixel = unwrap![some? self.bytes_per_pixel()];
        Some(RasterLayout::interleaved(
            ext![self.width() as u32, self.height() as u32],
            bytes_per_pixel,
            self.bytes_per_line(),
            Boundary1d::Upper,
        ))
    }

    /* pixels */

    /// Returns whether RGB8 colors can be encoded and written directly to this surface.
    pub const fn supports_rgb8(&self) -> bool {
        self.supports_native_pixels() && self.visual_format.supports_rgb8()
    }
    /// Returns whether native pixel values can be written directly to this surface.
    pub const fn supports_native_pixels(&self) -> bool {
        self.image_format.supports_native_pixels()
    }

    /// Encodes an RGB8 color as this X11 surface's native pixel value.
    ///
    /// Returns `None` when this surface does not support direct RGB8 encoding.
    pub const fn encode_rgb8(&self, rgb: [u8; 3]) -> Option<u32> {
        is! { self.supports_rgb8(), Some(self.visual_format.encode_rgb8(rgb)), None }
    }
    /// Encodes and writes an RGB8 color at `coord`.
    ///
    /// Returns whether the pixel was written.
    pub fn write_rgb8(&mut self, coord: Position2<u32>, rgb: [u8; 3]) -> bool {
        let Some(pixel) = self.encode_rgb8(rgb) else { return false };
        self.write_native_pixel(coord, pixel)
    }
    /// Writes an already encoded native X11 pixel at `coord`.
    ///
    /// Returns `false` when the format does not support direct native pixel writes,
    /// `coord` lies outside the surface, or the backing storage is too short.
    pub fn write_native_pixel(&mut self, coord: Position2<u32>, pixel: u32) -> bool {
        let image_format = self.image_format;
        let Some(offset) = self.pixel_offset_bytes(coord) else { return false };
        let Some(dst) = self.bytes_mut().get_mut(offset..) else { return false };
        image_format.write_native_pixel(dst, pixel)
    }
    fn pixel_offset_bytes(&self, coord: Position2<u32>) -> Option<usize> {
        let [x, y] = coord.dim;
        is! { x >= self.width() as u32 || y >= self.height() as u32, return None }
        let bytes_per_pixel = self.bytes_per_pixel()? as u64;
        let offset = y as u64 * self.bytes_per_line() as u64 + x as u64 * bytes_per_pixel;
        usize::try_from(offset).ok()
    }

    /// Returns the mutable surface bytes for direct rendering.
    pub fn bytes_mut(&mut self) -> &mut [u8] { self.surface.bytes_mut() }
}
