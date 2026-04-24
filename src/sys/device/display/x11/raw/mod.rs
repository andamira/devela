// devela::sys::display::x11::raw
//
//! Raw XCB bindings.
//!
//! Provides extern functions and constants from the X11 XCB protocol,
//! mirroring the core XCB and XCB-SHM APIs. These are thin, unsafe
//! calls to the X server with no added abstraction.
//!
//! # DOCS
//! There are two different documentation sources, generated in different ways,
//! and neither is complete on its own:
//! 1. XCB Manual (<https://xcb.freedesktop.org/manual/modules.html>)
//! 2. X.Org manpages (<https://x.org/releases/current/doc/man/>)

#![allow(unused)]

mod xcb; // main items from: xcb.h + xcb_ext.h + xproto.h
mod xcb_flags; // protocol bit-masks
mod xcb_shm; // shm extension
mod xcb_values; // protocol const values
pub(crate) use {xcb::*, xcb_flags::*, xcb_shm::*, xcb_values::*};

mod xkb; // libxkbcommon core bindings
mod xkb_x11; // libxkbcommon-x11 extension
pub(crate) use {xkb::*, xkb_x11::*};

// WM
mod icccm; // ICCCM: XSizeHints, XSizeRatio, XWinGravity, …
// mod ewmh; // EWMH
pub(crate) use {
    // ewmh::*,
    icccm::*,
};

mod _helper; // change_property_*
pub(crate) use _helper::*;

mod lut; // LUT_SCANCODE_TO_KEY
pub(crate) use lut::*;
