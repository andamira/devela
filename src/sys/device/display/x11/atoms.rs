// devela::sys::device::display::x11::atoms
//
//! Defines [`XAtoms`].
//

use super::raw;
use crate::{Libc, Ptr, is};

#[doc = crate::_tags!(linux uid runtime)]
/// Cached atoms required for high–level event interpretation.
#[doc = crate::_doc_location!("sys/device/display/x11")]
///
/// X11 atoms are integer identifiers allocated by the server and
/// used as symbolic names for properties and protocols.
///
/// This struct caches only the atoms that are relevant for the current minimal X11 backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct XAtoms {
    /// The `WM_NORMAL_HINTS atom.
    pub wm_normal_hints: u32,
    /// The `WM_PROTOCOLS` atom.
    pub wm_protocols: u32,
    /// The `WM_DELETE_WINDOW` protocol atom.
    pub wm_delete_window: u32,
}
impl XAtoms {
    /// Returns a new [`XAtoms`] by resolving and caching the relevant atoms.
    pub(crate) fn new(conn: *mut raw::xcb_connection_t) -> Self {
        let wm_normal_hints = XAtoms::get_atom(conn, b"WM_NORMAL_HINTS");
        let wm_protocols = XAtoms::get_atom(conn, b"WM_PROTOCOLS");
        let wm_delete_window = XAtoms::get_atom(conn, b"WM_DELETE_WINDOW");
        Self { wm_normal_hints, wm_protocols, wm_delete_window }
    }

    /// Resolves an X11 atom by name.
    ///
    /// Returns `0` if the atom does not exist or an error occurs.
    pub(crate) fn get_atom(conn: *mut raw::xcb_connection_t, name: &[u8]) -> u32 {
        unsafe {
            let cookie = raw::xcb_intern_atom(conn, 0, name.len() as u16, name.as_ptr());
            let mut err: *mut raw::xcb_generic_error_t = Ptr::null_mut();
            let reply = raw::xcb_intern_atom_reply(conn, cookie, &mut err);
            is![reply.is_null(), return 0];
            let atom = (*reply).atom;
            Libc::free(reply as *mut _);
            atom
        }
    }

    /// Sets a window property that stores a single atom value.
    ///
    /// This is mainly used to advertise support for a protocol such as `WM_DELETE_WINDOW`.
    pub(crate) fn set_property_atom(
        &self,
        conn: *mut raw::xcb_connection_t,
        window: u32,
        property: u32,
        atom_value: u32,
    ) {
        let data = atom_value.to_ne_bytes();
        unsafe {
            raw::xcb_change_property(
                conn,
                raw::xcb_prop_mode::XCB_PROP_MODE_REPLACE as u8,
                window,
                property,
                raw::xcb_atom_enum_t::XCB_ATOM_ATOM as u32,
                32,
                1,
                data.as_ptr(),
            );
        }
    }
}
