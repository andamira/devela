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
//! 1. XCB Manual (freedesktop.org/manual)
//! 2. X.Org manpages (x.org/releases/current/doc/man/…)

#![allow(unused)]

mod xcb; // main xcb.h + xproto.h structs, fns
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

mod helpers; // change_property_*
pub(crate) use helpers::*;

mod lut; // LUT_SCANCODE_TO_KEY
pub(crate) use lut::*;
