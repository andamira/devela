// devela::sys::device::display::x11::image
//
//! Defines [`XImageMode`], (`XImageFormat`), (`XImageStore`).
//

use super::{XDisplay, XError};

#[doc = crate::_tags!(unix runtime)]
/// Presentation backing policy for X11 image upload.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
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

#[doc = crate::_tags!(unix runtime)]
/// X11 image layout selected for byte-backed pixel upload.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
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
}
impl XImageFormat {
    /// Builds an image layout for `width` from one X11 pixmap format.
    pub const fn new(depth: u8, bits_per_pixel: u8, scanline_pad_bits: u8) -> Self {
        Self { depth, bits_per_pixel, scanline_pad_bits }
    }

    /// Returns the stored bytes per scanline for `width`.
    pub const fn bytes_per_line(self, width: u16) -> u32 {
        let bits = width as u32 * self.bits_per_pixel as u32;
        let pad = self.scanline_pad_bits as u32;
        if pad == 0 { bits.div_ceil(8) } else { (bits.div_ceil(pad) * pad).div_ceil(8) }
    }

    /// Returns the total byte length for `width × height`.
    pub const fn len_bytes(self, width: u16, height: u16) -> usize {
        self.bytes_per_line(width) as usize * height as usize
    }
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
