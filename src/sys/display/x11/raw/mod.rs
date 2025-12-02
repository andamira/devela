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

mod consts;
mod fns;
mod lut; // LUT_SCANCODE_TO_KEY
mod types;

pub(crate) use {consts::*, fns::*, lut::*, types::*};
