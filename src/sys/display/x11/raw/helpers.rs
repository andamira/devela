// devela::sys::display::x11::raw::helpers
//
//! Defines convenience helpers.
//

use super::{xcb_change_property, xcb_connection_t, xcb_prop_mode};
use crate::is;

/// Convenience wrapper for setting a 32-bit property.
///
/// `data` is interpreted as an array of 32-bit values (`format = 32`).
/// Common use cases include: `_NET_WM_STATE`, `_NET_WM_WINDOW_TYPE`,
/// `WM_NORMAL_HINTS`, and other ICCCM/EWMH integer properties.
#[inline(always)]
pub(crate) fn change_property_u32(
    conn: *mut xcb_connection_t,
    win: u32,
    property: u32,
    type_atom: u32,
    data: &[u32],
) {
    let bytes =
        unsafe { ::core::slice::from_raw_parts(data.as_ptr().cast::<u8>(), data.len() * 4) };
    change_property_u8(conn, win, property, type_atom, 32, bytes);
}

/// Convenience wrapper for setting UTF-8 string properties.
///
/// Used by EWMH (`_NET_WM_NAME`, `_NET_WM_ICON_NAME`) and ICCCM (`WM_NAME`)
/// when providing a UTF-8 encoded window title.
#[inline(always)]
pub(crate) fn change_property_str(
    conn: *mut xcb_connection_t,
    win: u32,
    property: u32,
    type_atom: u32,
    s: &str,
) {
    change_property_u8(conn, win, property, type_atom, 8, s.as_bytes());
}

/// Thin safe wrapper over `xcb_change_property` for all formats.
/// `format` must be 8, 16, or 32. `data` is raw bytes.
///
/// `data_len_units` is automatically derived from format.
#[inline(always)]
pub(crate) fn change_property_u8(
    conn: *mut xcb_connection_t,
    win: u32,
    property: u32,
    type_atom: u32,
    format: u8,
    data: &[u8],
) {
    debug_assert!(format == 8 || format == 16 || format == 32);

    let unit = (format as usize) / 8;
    let len_units = is![unit == 0; 0; data.len() as u32 / unit as u32];

    unsafe {
        xcb_change_property(
            conn,
            xcb_prop_mode::XCB_PROP_MODE_REPLACE as u8,
            win,
            property,
            type_atom,
            format,
            len_units,
            data.as_ptr().cast(),
        );
    }
}
