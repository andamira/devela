// devela::sys::display::x11::raw::iccm
//
//! ICCCM window-manager hint structures (`Xutil.h`).
//!
//! Provides raw data layouts and constants required to set
//! `WM_NORMAL_HINTS` via XCB. These definitions match the Xlib
//! `XSizeHints` struct exactly, enabling clients to express preferred
//! position, size, aspect ratio, gravity, and resize increments.
//!
//! The ICCCM is not part of the XCB protocol; it is a convention that
//! window managers are expected to follow. This module offers no
//! abstraction beyond the layout itself.
//

use super::super::raw;

/* constants */

pub(crate) const US_POSITION: i64 = 1 << 0; // user-specified x,y
pub(crate) const US_SIZE: i64 = 1 << 1; // user-specified width,height

pub(crate) const P_POSITION: i64 = 1 << 2; // program-specified position
pub(crate) const P_SIZE: i64 = 1 << 3; // program-specified size
pub(crate) const P_MIN_SIZE: i64 = 1 << 4; // program min size
pub(crate) const P_MAX_SIZE: i64 = 1 << 5; // program max size
pub(crate) const P_RESIZE_INC: i64 = 1 << 6; // width_inc, height_inc
pub(crate) const P_ASPECT: i64 = 1 << 7; // min/max aspect
pub(crate) const P_BASE_SIZE: i64 = 1 << 8; // base_width, base_height
pub(crate) const P_WIN_GRAVITY: i64 = 1 << 9;

/* types */

/// ICCCM WM_NORMAL_HINTS structure.
/// Must match XSizeHints layout exactly.
/// - <https://tronche.com/gui/x/xlib/ICC/client-to-window-manager/wm-normal-hints.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct XSizeHints {
    /// Bitmask specifying which fields are valid.
    pub flags: i64,

    // Obsolete for modern WMs but must be filled for compatibility.
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,

    /// Minimum usable window size.
    pub min_width: i32,
    pub min_height: i32,

    /// Maximum permitted window size.
    pub max_width: i32,
    pub max_height: i32,

    /// Preferred resize increments (width, height).
    pub width_inc: i32,
    pub height_inc: i32,

    /// Preferred aspect ratio range.
    pub aspect_min: XSizeRatio,
    pub aspect_max: XSizeRatio,

    /// Base size for incremental sizing (ICCCM version 1).
    pub base_width: i32,
    pub base_height: i32,

    /// Window gravity (ICCCM version 1).
    pub win_gravity: i32,
}

/// Simple integer ratio for ICCCM aspect hints.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct XSizeRatio {
    pub num: i32,
    pub den: i32,
}

/// Window gravity hint (ICCCCM).
///
/// Controls how a window’s position should be interpreted when the
/// window is resized or when its parent frame changes size.
/// The gravity defines which point of the outer rectangle remains
/// fixed relative to the parent.
///
/// This hint does not move the window by itself; it only influences
/// how the window manager interprets the window’s geometry.
///
/// Source: ICCCM §4.1.2.3 (XSizeHints.win_gravity)
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum XWinGravity {
    Forget = 0,
    NorthWest = 1,
    North = 2,
    NorthEast = 3,
    West = 4,
    Center = 5,
    East = 6,
    SouthWest = 7,
    South = 8,
    SouthEast = 9,
    Static = 10,
}

/* impls */

#[rustfmt::skip]
impl XSizeHints {
    /// Creates a new empty struct.
    pub fn new() -> Self {
        Self::default()
    }
    /// Marks a user-specified window position.
    pub fn set_position(mut self, x: i16, y: i16) -> Self {
        self.flags |= US_POSITION; self.x = x as i32; self.y = y as i32; self
    }
    /// Marks a user-specified window size.
    pub fn set_size(mut self, w: u16, h: u16) -> Self {
        self.flags |= US_SIZE; self.width = w as i32; self.height = h as i32; self
    }
    /// Sets the minimum accepted window size.
    pub fn set_min_size(mut self, w: u16, h: u16) -> Self {
        self.flags |= P_MIN_SIZE; self.min_width = w as i32; self.min_height = h as i32; self
    }
    /// Sets the maximum accepted window size.
    pub fn set_max_size(mut self, w: u16, h: u16) -> Self {
        self.flags |= P_MAX_SIZE; self.max_width = w as i32; self.max_height = h as i32; self
    }
    /// Sets the base size used for incremental resizing.
    pub fn set_resize_inc(mut self, w: i32, h: i32) -> Self {
        self.flags |= P_RESIZE_INC; self.width_inc = w; self.height_inc = h; self
    }
    /// Sets the preferred minimum and maximum aspect ratios.
    pub fn set_base_size(mut self, w: i32, h: i32) -> Self {
        self.flags |= P_BASE_SIZE; self.base_width = w; self.base_height = h; self
    }
    /// Sets the preferred minimum and maximum aspect ratios.
    pub fn set_aspect(mut self, min_num: i32, min_den: i32, max_num: i32, max_den: i32) -> Self {
        self.flags |= P_ASPECT;
        self.aspect_min = XSizeRatio { num: min_num, den: min_den };
        self.aspect_max = XSizeRatio { num: max_num, den: max_den };
        self
    }
    /// Sets the window gravity hint.
    pub fn set_gravity(mut self, g: XWinGravity) -> Self {
        self.flags |= P_WIN_GRAVITY;
        self.win_gravity = g as i32;
        self
    }

    /// Pushes `WM_NORMAL_HINTS` to the server.
    pub fn set_on(&self, conn: *mut raw::xcb_connection_t, win: u32, atom_normal_hints: u32) {
        let arr = self.as_u32_array();
        raw::change_property_u32(conn, win, atom_normal_hints,
            raw::xcb_atom_enum_t::XCB_ATOM_WM_SIZE_HINTS as u32, &arr);
    }
    /// Converts the ICCCM struct into a 20-long array of 32-bit units.
    pub fn as_u32_array(&self) -> [u32; 20] {
        [
            self.flags as u32, // orig i64
            self.x as u32,
            self.y as u32,
            self.width as u32,
            self.height as u32,
            self.min_width as u32,
            self.min_height as u32,
            self.max_width as u32,
            self.max_height as u32,
            self.width_inc as u32,
            self.height_inc as u32,
            self.aspect_min.num as u32,
            self.aspect_min.den as u32,
            self.aspect_max.num as u32,
            self.aspect_max.den as u32,
            self.base_width as u32,
            self.base_height as u32,
            self.win_gravity as u32,
            0, // padding (ICCCM defines 18–19 unused)
            0,
        ]
    }
}
