// devela/src/sys/device/display/x11/_raw/_.rs
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

crate::mods_in! {
    mod xcb; // main items from: xcb.h + xcb_ext.h + xproto.h
    mod xcb_flags; // protocol bit-masks
    mod xcb_shm; // shm extension
    mod xcb_values; // protocol const values

    mod xkb; // libxkbcommon core bindings
    mod xkb_x11; // libxkbcommon-x11 extension

    // WM
    mod icccm; // ICCCM: XSizeHints, XSizeRatio, XWinGravity, …
    // mod ewmh; // EWMH

    mod _helper; // change_property_*
    mod lut; // LUT_SCANCODE_TO_KEY
}
crate::mods_out! { // _crate_internals
    _crate_internals {
        pub(crate) use super::{
            xcb::*,
            xcb_flags::*,
            xcb_shm::*,
            xcb_values::*,
        };
        pub(crate) use super::{
            xkb::*,
            xkb_x11::*,
        };
        pub(crate) use super::{
            // ewmh::*,
            icccm::*,
        };
        pub(crate) use super::_helper::*;
        pub(crate) use super::lut::*;
    }
}
