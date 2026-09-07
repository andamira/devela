// devela/src/sys/device/display/x11/surface/frame.rs
//
//! Defines [`XSurfaceFrame`].
//

use crate::{Boundary1d, RasterLayout, XImageMode, XImageStore, XSurface, ext, is, unwrap};

#[doc = crate::_tags!(unix runtime)]
/// Borrowed mutable X11 surface for direct frame rendering.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11", struct XSurfaceFrame),
    #[cfg(target_pointer_width = "64")]
    test_size_of(XSurfaceFrame<'_> = 16|128; niche Option),
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
    bytes_per_line: u32,
    bits_per_pixel: u8,
}
#[rustfmt::skip]
impl<'a> XSurfaceFrame<'a> {
    pub(crate) const fn _new(surface: &'a mut XSurface, bytes_per_line: u32, bits_per_pixel: u8)
        -> Self { Self { surface, bytes_per_line, bits_per_pixel } }

    /* */

    /// Returns the surface width in pixels.
    #[must_use]
    pub const fn width(&self) -> u16 { self.surface.width }

    /// Returns the surface height in pixels.
    #[must_use]
    pub const fn height(&self) -> u16 { self.surface.height }

    #[must_use]
    /// Returns the surface pixel depth in bits.
    pub const fn depth(&self) -> u8 { self.surface.depth }

    #[must_use]
    /// Returns the number of stored bits per pixel.
    pub const fn bits_per_pixel(&self) -> u8 { self.bits_per_pixel }

    #[must_use]
    /// Returns the number of stored bytes per pixel, when byte-aligned.
    pub const fn bytes_per_pixel(&self) -> Option<u8> {
        is! { self.bits_per_pixel.is_multiple_of(8), Some(self.bits_per_pixel / 8), None }
    }
    #[must_use]
    /// Returns the byte stride between consecutive rows.
    pub const fn bytes_per_line(&self) -> u32 { self.bytes_per_line }

    /// Returns the active surface backing mode.
    pub const fn mode(&self) -> XImageMode { self.surface.mode() }

    #[must_use]
    /// Returns whether each row has no backend padding.
    pub const fn is_tight_rows(&self) -> bool {
        let bits = self.width() as u32 * self.bits_per_pixel() as u32;
        bits.div_ceil(8) == self.bytes_per_line
    }
    /// Returns this surface's byte-addressable raster layout.
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
    /// Returns the mutable surface bytes for direct rendering.
    pub fn bytes_mut(&mut self) -> &mut [u8] { self.surface.bytes_mut() }
}
