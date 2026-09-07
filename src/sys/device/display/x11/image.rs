// devela/src/sys/device/display/x11/image.rs
//
//! Defines [`XImageMode`], (`XImageStore`), (`XImageFormat`), (`XVisualFormat`).
//

use crate::{XDisplay, XError, is, slice};

#[doc = crate::_tags!(unix runtime)]
/// Presentation backing policy for X11 image upload.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11", enum XImageMode),
}]
/// This selects whether image presentation should prefer plain CPU upload,
/// MIT-SHM upload, or automatically choose the best available path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use]
pub enum XImageMode {
    /// Chooses the best available presentation path at runtime.
    ///
    /// The resolved active mode can be queried later through
    /// [`XFrontend::active_mode`][crate::XFrontend::active_mode].
    Auto,
    /// Forces plain client-side CPU upload.
    Cpu,
    /// Forces MIT-SHM-backed upload.
    #[cfg(ffi_xcb_shm··)]
    Shm,
}

/// Retained X11 image storage used by surface backends.
///
/// This is a private backend-storage contract, not a public raster-view bridge.
pub(crate) trait XImageStore {
    fn size(&self) -> (u16, u16);
    fn depth(&self) -> u8;
    fn bytes(&self) -> &[u8];
    fn bytes_mut(&mut self) -> &mut [u8];
    #[allow(dead_code, reason = "WIP")]
    fn resize(
        &mut self,
        display: &XDisplay,
        width: u16,
        height: u16,
        depth: u8,
    ) -> Result<(), XError>;
}

crate::test_size_of!(XImageFormat = 4|32; niche !Option);
#[doc = crate::_tags!(unix runtime)]
/// X11 image layout selected for byte-backed pixel upload.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11", struct XImageFormat),
}]
/// Describes how one logical raster is stored in memory for upload to the X server.
///
/// It is explicit about storage width and scanline padding, since X11 image
/// formats are not defined by one universal typed pixel representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct XImageFormat {
    /// Logical image depth in bits.
    pub(crate) depth: u8,
    /// Stored bits per pixel.
    pub(crate) bits_per_pixel: u8,
    /// Required scanline padding in bits.
    pub(crate) scanline_pad_bits: u8,
    /// Image byte order.
    pub(crate) image_byte_order: u8,
}
impl XImageFormat {
    /// Builds an image layout for `width` from one X11 pixmap format.
    pub const fn new(
        depth: u8,
        bits_per_pixel: u8,
        scanline_pad_bits: u8,
        image_byte_order: u8,
    ) -> Self {
        Self {
            depth,
            bits_per_pixel,
            scanline_pad_bits,
            image_byte_order,
        }
    }
    /// Returns the stored bytes per scanline for `width`.
    pub const fn bytes_per_line(self, width: u16) -> u32 {
        let bits = width as u32 * self.bits_per_pixel as u32;
        let pad = self.scanline_pad_bits as u32;
        is! { pad == 0, bits.div_ceil(8), (bits.div_ceil(pad) * pad).div_ceil(8) }
    }
    /// Returns the total byte length for `width × height`.
    pub const fn len_bytes(self, width: u16, height: u16) -> usize {
        self.bytes_per_line(width) as usize * height as usize
    }
    /// Returns whether native pixel values can be written directly with this image format.
    pub(crate) const fn supports_native_pixel(self) -> bool {
        let bpp = self.bits_per_pixel;
        is! { !bpp.is_multiple_of(8), return false }
        let bytes = bpp / 8;
        matches!(bytes, 1..=4) && matches!(self.image_byte_order, 0 | 1)
    }
    /// Writes a native X11 pixel value using this image format's byte order.
    ///
    /// Returns `false` when the stored pixel width is unsupported,
    /// `dst` is too short, or the image byte order is invalid.
    pub(crate) const fn write_native_pixel(self, dst: &mut [u8], pixel: u32) -> bool {
        is! { !self.supports_native_pixel(), return false }
        let bytes = (self.bits_per_pixel / 8) as usize;
        is! { dst.len() < bytes, return false }
        let (src, start) = match self.image_byte_order {
            0 => (pixel.to_le_bytes(), 0),
            1 => (pixel.to_be_bytes(), 4 - bytes),
            _ => unreachable!(),
        };
        slice![mut dst, ..bytes].copy_from_slice(slice![&src, start, ..start + bytes]);
        true
    }
}

crate::test_size_of!(XVisualFormat = 20|160; niche !Option);
#[doc = crate::_tags!(unix runtime)]
/// X11 visual format.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11", struct XVisualFormat),
}]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct XVisualFormat {
    pub(crate) visual_id: u32,
    pub(crate) class: u8,
    pub(crate) bits_per_rgb: u8,
    pub(crate) red_mask: u32,
    pub(crate) green_mask: u32,
    pub(crate) blue_mask: u32,
}
impl XVisualFormat {
    const CLASS_TRUE_COLOR: u8 = 4;
    /// Returns whether this is an X11 TrueColor visual.
    pub(crate) const fn is_true_color(self) -> bool {
        self.class == Self::CLASS_TRUE_COLOR
    }
    /// Returns whether the RGB masks are nonzero, contiguous and disjoint.
    pub(crate) const fn has_valid_rgb_masks(self) -> bool {
        Self::mask_is_contiguous(self.red_mask)
            && Self::mask_is_contiguous(self.green_mask)
            && Self::mask_is_contiguous(self.blue_mask)
            && self.red_mask & self.green_mask == 0
            && self.red_mask & self.blue_mask == 0
            && self.green_mask & self.blue_mask == 0
    }
    /// Returns whether direct RGB8 encoding is supported for this visual.
    pub(crate) const fn supports_rgb8(self) -> bool {
        self.is_true_color() && self.has_valid_rgb_masks()
    }
    const fn mask_is_contiguous(mask: u32) -> bool {
        is! { mask == 0, return false }
        let m = mask >> mask.trailing_zeros();
        m & m.wrapping_add(1) == 0
    }
    /// Encodes an 8-bit RGB color into this TrueColor visual's pixel value.
    ///
    /// The visual must satisfy [`supports_rgb8`][Self::supports_rgb8].
    pub(crate) const fn encode_rgb8(self, [r, g, b]: [u8; 3]) -> u32 {
        Self::encode_component(r, self.red_mask)
            | Self::encode_component(g, self.green_mask)
            | Self::encode_component(b, self.blue_mask)
    }
    const fn encode_component(value: u8, mask: u32) -> u32 {
        is! { mask == 0, return 0 }
        let shift = mask.trailing_zeros();
        let max = mask >> shift;
        // Scale 0..=255 onto the mask field, rounded to the nearest value.
        let scaled = (value as u64 * max as u64 + 127) / 255;
        ((scaled as u32) << shift) & mask
    }
}
