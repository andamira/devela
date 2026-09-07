// devela/src/sys/device/display/x11/surface/frame.rs
//
//! Defines [`XSurfaceFrame`].
//

use crate::{Boundary1d, Position2, RasterLayout, ext, is, unwrap};
use crate::{XImageFormat, XImageMode, XImageStore, XSurface, XVisualFormat};

#[doc = crate::_tags!(unix runtime)]
/// Borrowed mutable X11 surface for direct frame rendering.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11", struct XSurfaceFrame),
    #[cfg(target_pointer_width = "64")]
    test_size_of(XSurfaceFrame<'_> = 32|256; niche Option),
}]
/// This exposes the retained X11 presentation surface for one frame.
///
/// Drawing into this surface avoids the intermediate scene-to-surface copy
/// used by [`XRasterRenderer`][crate::XRasterRenderer].
///
/// `XSurfaceFrame` is the X11 direct-surface path.
///
/// It exposes the retained X11 surface for one frame, allowing callers to render
/// directly into the CPU/SHM presentation buffer.
///
/// Use it when X11-specific performance or surface control matters.
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

    /// Returns this surface's byte-addressable raster layout.
    ///
    ///
    /// The layout describes its pixel extent, stored bytes per pixel,
    /// row stride, and upper-first row orientation.
    /// It does not describe the pixel color or channel encoding.
    ///
    /// Returns `None` when the stored pixel width is not byte-aligned.
    #[must_use]
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
        self.supports_native_pixel() && self.visual_format.supports_rgb8()
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

    /// Returns whether native pixel values can be written directly to this surface.
    pub const fn supports_native_pixel(&self) -> bool {
        self.image_format.supports_native_pixel()
    }
    /// Writes an already encoded native X11 pixel at `coord`.
    ///
    /// Returns whether the pixel was written.
    pub fn write_native_pixel(&mut self, coord: Position2<u32>, pixel: u32) -> bool {
        let image_format = self.image_format;
        let Some(layout) = self.raster_layout() else { return false };
        let Some(offset) = layout.pixel_offset_bytes(coord) else { return false };
        let Some(dst) = self.bytes_mut().get_mut(offset..) else { return false };
        image_format.write_native_pixel(dst, pixel)
    }

    /// Returns the mutable surface bytes for direct rendering.
    pub fn bytes_mut(&mut self) -> &mut [u8] { self.surface.bytes_mut() }
}
