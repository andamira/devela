// devela::sys::display::x11::_raw::xcb_shm
//
//! from `shm.h`
//
// DOC LINKS:
// - https://www.x.org/releases/current/doc/man/man3/
// - https://xcb.freedesktop.org/manual/group__XCB__Shm__API.html
//
// TOC
// - xcb_shm_seg_t
// - xcb_shm_query_version_cookie_t
// - xcb_shm_create_segment_cookie_t
// - xcb_shm_query_version_reply_t
// - xcb_shm_create_segment_reply_t
//
// - xcb_shm_id
// - xcb_shm_query_version()
// - xcb_shm_query_version_reply()
// - xcb_shm_create_segment()
// - xcb_shm_create_segment_reply()
// - xcb_shm_create_segment_reply_fds()
// - xcb_shm_attach_fd()
// - xcb_shm_detach()
// - xcb_shm_put_image()

#![allow(non_camel_case_types)]

use crate::{
    c_int, c_void, xcb_connection_t, xcb_drawable_t, xcb_extension_t, xcb_gcontext_t,
    xcb_generic_error_t, xcb_void_cookie_t, xcb_window_t,
};

/// X11 shared-memory segment identifier.
pub(crate) type xcb_shm_seg_t = u32;

// /// XCB SHM segment separator (for shmseg 0).
// pub(crate) const XCB_SHM_SEG_SEPARATOR: u32 = 0;

#[doc = crate::_tags!(unix ffi)]
/// - <https://xcb.freedesktop.org/manual/structxcb__shm__query__version__cookie__t.html>
#[repr(C)]
pub(crate) struct xcb_shm_query_version_cookie_t {
    pub(crate) sequence: u32,
}

#[doc = crate::_tags!(unix ffi)]
/// - <https://xcb.freedesktop.org/manual/structxcb__shm__create__segment__cookie__t.html>
#[repr(C)]
pub(crate) struct xcb_shm_create_segment_cookie_t {
    pub(crate) sequence: u32,
}

#[doc = crate::_tags!(unix ffi)]
/// - <https://xcb.freedesktop.org/manual/structxcb__shm__query__version__reply__t.html>
#[repr(C)]
pub(crate) struct xcb_shm_query_version_reply_t {
    pub(crate) response_type: u8,
    pub(crate) shared_pixmaps: u8,
    pub(crate) sequence: u16,
    pub(crate) length: u32,
    pub(crate) major_version: u16,
    pub(crate) minor_version: u16,
    pub(crate) uid: u16,
    pub(crate) gid: u16,
    pub(crate) pixmap_format: u8,
    pub(crate) pad0: [u8; 15],
}

#[doc = crate::_tags!(unix ffi)]
/// - <https://xcb.freedesktop.org/manual/structxcb__shm__create__segment__reply__t.html>
#[repr(C)]
pub(crate) struct xcb_shm_create_segment_reply_t {
    pub(crate) response_type: u8,
    pub(crate) nfd: u8,
    pub(crate) sequence: u16,
    pub(crate) length: u32,
    pub(crate) pad0: [u8; 24],
}

#[rustfmt::skip]
#[link(name = "xcb-shm")]
unsafe extern "C" {
    ///
    pub(crate) static mut xcb_shm_id: xcb_extension_t;

    ///
    pub(crate) fn xcb_shm_query_version(c: *mut xcb_connection_t) -> xcb_shm_query_version_cookie_t;

    ///
    pub(crate) fn xcb_shm_query_version_reply(c: *mut xcb_connection_t,
        cookie: xcb_shm_query_version_cookie_t, e: *mut *mut xcb_generic_error_t)
        -> *mut xcb_shm_query_version_reply_t;

    ///
    pub(crate) fn xcb_shm_create_segment(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t,
        size: u32, read_only: u8) -> xcb_shm_create_segment_cookie_t;

    ///
    pub(crate) fn xcb_shm_create_segment_reply(c: *mut xcb_connection_t,
        cookie: xcb_shm_create_segment_cookie_t, e: *mut *mut xcb_generic_error_t)
        -> *mut xcb_shm_create_segment_reply_t;

    ///
    pub(crate) fn xcb_shm_create_segment_reply_fds(c: *mut xcb_connection_t,
        reply: *mut xcb_shm_create_segment_reply_t) -> *mut c_int;

    // FUTURE
    /// Attaches a shared-memory segment to the X server using a file descriptor.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_shm_attach.3.xhtml>
    pub(crate) fn xcb_shm_attach_fd(c: *mut xcb_connection_t,
        shmseg: xcb_shm_seg_t, shm_fd: c_int, read_only: u8) -> xcb_void_cookie_t;

    ///
    pub(crate) fn xcb_shm_detach(c: *mut xcb_connection_t, shmseg: xcb_shm_seg_t)
        -> xcb_void_cookie_t;

    /// Puts an image into a drawable using shared memory.
    /// - <https://www.x.org/releases/current/doc/man/man3/xcb_shm_put_image.3.xhtml>
    pub(crate) fn xcb_shm_put_image(c: *mut xcb_connection_t, drawable: xcb_drawable_t,
        gc: xcb_gcontext_t, total_width: u16, total_height: u16, src_x: u16, src_y: u16,
        src_width: u16, src_height: u16, dst_x: i16, dst_y: i16, depth: u8, format: u8,
        send_event: u8, shmseg: xcb_shm_seg_t, offset: u32) -> xcb_void_cookie_t;
}
