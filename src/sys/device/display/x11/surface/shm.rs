// devela::sys::device::display::x11::surface::shm
//
//! Defines [`XShmCaps`], [`XShmBuffer`].

use super::super::_raw;
use crate::{Libc, XDisplay, XError, XImageFormat, XImageStore, XWindow};

#[doc = crate::_tags!(unix runtime)]
/// MIT-SHM capabilities reported by the X server.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// This is an explicit capability snapshot gathered once from the display.
///
/// It answers whether MIT-SHM is present and which shared-pixmap properties
/// the server reports, without committing yet to a specific client storage path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct XShmCaps {
    pub major_version: u16,
    pub minor_version: u16,
    pub shared_pixmaps: bool,
    pub pixmap_format: u8,
}

#[doc = crate::_tags!(unix runtime)]
/// MIT-SHM-backed pixel buffer for X11 image upload.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// Owns one shared-memory mapping together with the X11 SHM segment attachment
/// used to upload image bytes efficiently.
//
// This first implementation uses `xcb_shm_create_segment`.
// A future variant may support `xcb_shm_attach_fd` for caller-managed backing.
#[derive(Debug)]
pub struct XShmBuffer {
    pub(super) conn: *mut _raw::xcb_connection_t,
    pub(super) shmseg: _raw::xcb_shm_seg_t,

    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) format: XImageFormat,
    pub(super) bytes_per_line: u32,

    /// Local mapped base pointer.
    pub(super) bytes: *mut u8,
    /// Total mapped byte length.
    pub(super) len_bytes: usize,
    /// Owned fd returned by `xcb_shm_create_segment_reply_fds`.
    pub(super) fd: i32,
}
impl XShmBuffer {
    /// Creates a new fd-backed MIT-SHM pixel buffer compatible with the display.
    pub fn new(display: &XDisplay, width: u16, height: u16, depth: u8) -> Result<Self, XError> {
        let Some(_caps) = display.shm_caps else {
            return Err(XError::ExtensionUnavailable("MIT-SHM"));
        };
        let format = display.image_format;
        if depth != format.depth {
            return Err(XError::Other("requested depth does not match display image format"));
        }
        let bytes_per_line = format.bytes_per_line(width);
        let len_bytes = format.len_bytes(width, height);
        if len_bytes == 0 {
            return Err(XError::Other("zero-sized SHM buffer"));
        }
        if len_bytes > u32::MAX as usize {
            return Err(XError::Other("SHM buffer too large for xcb_shm_create_segment"));
        }
        unsafe {
            let conn = display.conn;
            let shmseg = _raw::xcb_generate_id(conn);

            let cookie = _raw::xcb_shm_create_segment(conn, shmseg, len_bytes as u32, 0);
            let mut err = core::ptr::null_mut();
            let reply = _raw::xcb_shm_create_segment_reply(conn, cookie, &mut err);

            if !err.is_null() {
                let code = (*err).error_code;
                Libc::free(err.cast());
                return Err(XError::ProtocolError(code));
            }
            if reply.is_null() {
                return Err(XError::Other("xcb_shm_create_segment returned no reply"));
            }
            if (*reply).nfd < 1 {
                Libc::free(reply.cast());
                return Err(XError::Other("xcb_shm_create_segment returned no file descriptor"));
            }

            let fds = _raw::xcb_shm_create_segment_reply_fds(conn, reply);
            if fds.is_null() {
                Libc::free(reply.cast());
                return Err(XError::Other("xcb_shm_create_segment_reply_fds returned null"));
            }
            let fd = *fds;
            Libc::free(reply.cast());
            let map = Libc::mmap(
                core::ptr::null_mut(),
                len_bytes,
                Libc::PROT_READ | Libc::PROT_WRITE,
                Libc::MAP_SHARED,
                fd,
                0,
            );
            if Libc::is_map_failed(map) {
                Libc::close(fd);
                let _ = _raw::xcb_shm_detach(conn, shmseg);
                return Err(XError::Other("mmap failed for X11 SHM segment"));
            }
            // the mapping stays valid after close()
            Libc::close(fd);
            Ok(Self {
                conn,
                shmseg,
                width,
                height,
                format,
                bytes_per_line,
                bytes: map.cast::<u8>(),
                len_bytes,
                fd: -1,
            })
        }
    }

    /// Returns the bits per pixel associated with the buffer.
    pub const fn bits_per_pixel(&self) -> u8 {
        self.format.bits_per_pixel
    }
    /// Returns the bytes per line associated with the buffer.
    pub const fn bytes_per_line(&self) -> u32 {
        self.bytes_per_line
    }

    /// Presents the current pixels to `window`.
    pub fn present(&self, _display: &mut XDisplay, window: &XWindow) -> Result<(), XError> {
        unsafe {
            _raw::xcb_shm_put_image(
                self.conn,
                window.win,
                window.gc,
                self.width,  // total_width
                self.height, // total_height
                0,           // src_x
                0,           // src_y
                self.width,  // src_width
                self.height, // src_height
                0,           // dst_x
                0,           // dst_y
                self.format.depth,
                _raw::XCB_IMAGE_FORMAT_Z_PIXMAP,
                0, // send_event
                self.shmseg,
                0, // offset
            );
        }
        Ok(())
    }
}
#[rustfmt::skip]
impl XImageStore for XShmBuffer {
    fn size(&self) -> (u16, u16) { (self.width, self.height) }
    fn depth(&self) -> u8 { self.format.depth }
    fn bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.bytes, self.len_bytes) }
    }
    fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.bytes, self.len_bytes) }
    }
    fn resize(&mut self, display: &XDisplay, width: u16, height: u16, depth: u8)
        -> Result<(), XError> {
        if self.width == width && self.height == height && self.format.depth == depth {
            return Ok(());
        }
        let new = Self::new(display, width, height, depth)?;
        *self = new;
        Ok(())
    }
}
impl Drop for XShmBuffer {
    fn drop(&mut self) {
        unsafe {
            let _ = _raw::xcb_shm_detach(self.conn, self.shmseg);
            if !self.bytes.is_null() && self.len_bytes != 0 {
                let _ = Libc::munmap(self.bytes.cast(), self.len_bytes);
            }
            if self.fd >= 0 {
                let _ = Libc::close(self.fd);
            }
        }
    }
}
