// devela/src/sys/device/display/x11/surface/_base.rs
//
//! Defines `XSurface`, `XSurfaceStorage`.
//

#[cfg(ffi_xcb_shm··)]
use crate::XShmBuffer;
use crate::{XCpuBuffer, XDisplay, XError, XImageMode, XImageStore, XWindow};

#[doc = crate::_tags!(unix runtime)]
/// Pixel-backed X11 drawing surface.
#[doc = crate::_doc_meta!{
    location("sys/device/display/x11", struct XSurface),
}]
/// Owns raster pixels independently of any one rendering policy, and can present
/// them to an [`XWindow`] using either shared memory or plain client-side upload.
#[derive(Debug)]
pub(crate) struct XSurface {
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) depth: u8,
    pub(crate) storage: XSurfaceStorage,
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

/// Retained X11 image backing, either CPU-owned or MIT-SHM-backed.
#[derive(Debug)]
pub(crate) enum XSurfaceStorage {
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
