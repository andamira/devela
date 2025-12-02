// devela::sys::display::x11::raw::xcb_shm
//
//! SHM related
//
// - xcb_shm_seg_t
// - xcb_shm_attach_fd
// - xcb_shm_put_image

#![allow(non_camel_case_types)]

use crate::{c_int, xcb_connection_t, xcb_gcontext_t, xcb_void_cookie_t, xcb_window_t};

/// XCB SHM segment separator (for shmseg 0).
pub(crate) const XCB_SHM_SEG_SEPARATOR: u32 = 0;

/// X11 shared-memory segment identifier.
pub(crate) type xcb_shm_seg_t = u32;

#[rustfmt::skip]
#[link(name = "xcb-shm")]
unsafe extern "C" {
    /// Attaches a shared-memory segment to the X server using a file descriptor.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_shm_attach.3.xhtml>
    pub(crate) fn xcb_shm_attach_fd(c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t, shm_fd: c_int, read_only: u8) -> xcb_void_cookie_t;

    /// Puts an image into a drawable using shared memory.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_shm_put_image.3.xhtml>
    pub(crate) fn xcb_shm_put_image(c: *mut xcb_connection_t, drawable: xcb_window_t,
        gc: xcb_gcontext_t, total_width: u16, total_height: u16, src_x: u16, src_y: u16,
        src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8,
        send_event: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_void_cookie_t;
}
