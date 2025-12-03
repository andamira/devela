// devela::sys::display::x11::window
//
//!
//

use crate::{
    Libc, Ptr, xcb_connection_t, xcb_generic_error_t, xcb_intern_atom, xcb_intern_atom_reply,
};

/// Helper for getting an atom name.
pub(crate) fn x11_intern_atom(conn: *mut xcb_connection_t, name: &[u8]) -> u32 {
    unsafe {
        let cookie = xcb_intern_atom(conn, 0, name.len() as u16, name.as_ptr());
        let mut err: *mut xcb_generic_error_t = Ptr::null_mut();
        let reply = xcb_intern_atom_reply(conn, cookie, &mut err);
        if reply.is_null() {
            0
        } else {
            let atom = (*reply).atom;
            Libc::free(reply as *mut _);
            atom
        }
    }
}
