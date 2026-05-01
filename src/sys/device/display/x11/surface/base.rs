// devela::sys::device::display::x11::surface::base
//
//! Defines `XSurfaceFrame`, (`XSurface`), (`XSurfaceStorage`).
//

#[cfg(ffi_xcb_shm··)]
use crate::XShmBuffer;
use crate::{XCpuBuffer, XDisplay, XError, XImageMode, XImageStore, XWindow};

#[doc = crate::_tags!(unix runtime)]
/// Pixel-backed X11 drawing surface.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// Owns raster pixels independently of any one rendering policy, and can present
/// them to an [`XWindow`] using either shared memory or plain client-side upload.
#[derive(Debug)]
pub(crate) struct XSurface {
    width: u16,
    height: u16,
    depth: u8,
    storage: XSurfaceStorage,
}
impl XSurface {
    fn _new(width: u16, height: u16, depth: u8, storage: XSurfaceStorage) -> Self {
        Self { width, height, depth, storage }
    }
    /// Creates the best available X11 pixel surface for the display.
    pub fn new(display: &XDisplay, width: u16, height: u16, depth: u8) -> Result<Self, XError> {
        #[cfg(not(ffi_xcb_shm··))]
        return Self::new_cpu(display, width, height, depth);
        #[cfg(ffi_xcb_shm··)]
        match XShmBuffer::new(display, width, height, depth) {
            Ok(shm) => Ok(Self::_new(width, height, depth, XSurfaceStorage::Shm(shm))),
            Err(XError::ExtensionUnavailable(_)) => Self::new_cpu(display, width, height, depth),
            Err(err) => Err(err),
        }
    }
    /// Creates a CPU-backed X11 pixel surface.
    pub fn new_cpu(display: &XDisplay, width: u16, height: u16, depth: u8) -> Result<Self, XError> {
        let cpu = XCpuBuffer::new(display, width, height, depth)?;
        Ok(Self::_new(width, height, depth, XSurfaceStorage::Cpu(cpu)))
    }
    /// Creates an SHM-backed X11 pixel surface.
    #[cfg(ffi_xcb_shm··)]
    pub fn new_shm(display: &XDisplay, width: u16, height: u16, depth: u8) -> Result<Self, XError> {
        let shm = XShmBuffer::new(display, width, height, depth)?;
        Ok(Self::_new(width, height, depth, XSurfaceStorage::Shm(shm)))
    }
    ///
    pub fn present(&self, display: &mut XDisplay, window: &XWindow) -> Result<(), XError> {
        self.storage.present(display, window, self.width, self.height, self.depth)
    }
    /// Returns the current X11 image presentation mode.
    pub const fn mode(&self) -> XImageMode {
        match self.storage {
            XSurfaceStorage::Cpu(_) => XImageMode::Cpu,
            #[cfg(ffi_xcb_shm··)]
            XSurfaceStorage::Shm(_) => XImageMode::Shm,
        }
    }
}
#[rustfmt::skip]
impl XImageStore for XSurface {
    fn size(&self) -> (u16, u16) { (self.width, self.height) }
    fn depth(&self) -> u8 { self.depth }
    fn bytes(&self) -> &[u8] { self.storage.bytes() }
    fn bytes_mut(&mut self) -> &mut [u8] { self.storage.bytes_mut() }
    fn resize(&mut self, display: &XDisplay, width: u16, height: u16, depth: u8)
        -> Result<(), XError> {
        self.storage.resize(display, width, height, depth)
    }
}

#[doc = crate::_tags!(unix runtime)]
/// Borrowed mutable X11 surface for direct frame rendering.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
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
}
#[rustfmt::skip]
impl<'a> XSurfaceFrame<'a> {
    pub(crate) const fn _new(surface: &'a mut XSurface, bytes_per_line: u32) -> Self {
        Self { surface, bytes_per_line }
    }
    /// Returns the surface width in pixels.
    #[must_use]
    pub const fn width(&self) -> u16 { self.surface.width }
    /// Returns the surface height in pixels.
    #[must_use]
    pub const fn height(&self) -> u16 { self.surface.height }
    /// Returns the surface pixel depth in bits.
    #[must_use]
    pub const fn depth(&self) -> u8 { self.surface.depth }
    /// Returns the byte stride between consecutive rows.
    #[must_use]
    pub const fn bytes_per_line(&self) -> u32 { self.bytes_per_line }
    /// Returns the active surface backing mode.
    pub const fn mode(&self) -> XImageMode { self.surface.mode() }
    /// Returns whether each row has no backend padding.
    #[must_use]
    pub const fn is_tight_rows(&self) -> bool {
        let bits = self.width() as u32 * self.depth() as u32;
        bits.div_ceil(8) == self.bytes_per_line
    }
    /// Returns the mutable surface bytes for direct rendering.
    pub fn bytes_mut(&mut self) -> &mut [u8] { self.surface.bytes_mut() }
}

/// Retained X11 image backing, either CPU-owned or MIT-SHM-backed.
#[derive(Debug)]
enum XSurfaceStorage {
    Cpu(XCpuBuffer),
    #[cfg(ffi_xcb_shm··)]
    Shm(XShmBuffer),
}
#[rustfmt::skip]
impl XSurfaceStorage {
    #[allow(unused_variables, reason = "display gated by ffi_xcb_shm··")]
    pub fn present(&self, display: &mut XDisplay, window: &XWindow,
        width: u16, height: u16, depth: u8) -> Result<(), XError> {
        match self {
            Self::Cpu(cpu) => {
                window.put_image_bytes(width, height, depth, cpu.bytes());
                Ok(())
            }
            #[cfg(ffi_xcb_shm··)]
            Self::Shm(shm) => shm.present(display, window),
        }
    }
}
impl XImageStore for XSurfaceStorage {
    fn size(&self) -> (u16, u16) {
        match self {
            Self::Cpu(cpu) => cpu.size(),
            #[cfg(ffi_xcb_shm··)]
            Self::Shm(shm) => shm.size(),
        }
    }
    fn depth(&self) -> u8 {
        match self {
            Self::Cpu(cpu) => cpu.depth(),
            #[cfg(ffi_xcb_shm··)]
            Self::Shm(shm) => shm.depth(),
        }
    }
    fn bytes(&self) -> &[u8] {
        match self {
            Self::Cpu(cpu) => cpu.bytes(),
            #[cfg(ffi_xcb_shm··)]
            Self::Shm(shm) => shm.bytes(),
        }
    }
    fn bytes_mut(&mut self) -> &mut [u8] {
        match self {
            Self::Cpu(cpu) => cpu.bytes_mut(),
            #[cfg(ffi_xcb_shm··)]
            Self::Shm(shm) => shm.bytes_mut(),
        }
    }
    fn resize(
        &mut self,
        display: &XDisplay,
        width: u16,
        height: u16,
        depth: u8,
    ) -> Result<(), XError> {
        match self {
            Self::Cpu(cpu) => cpu.resize(display, width, height, depth),
            #[cfg(ffi_xcb_shm··)]
            Self::Shm(shm) => shm.resize(display, width, height, depth),
        }
    }
}
