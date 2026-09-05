// devela/src/sys/device/display/x11/_.rs
//
#![doc = crate::_DOC_SYS_DEVICE_DISPLAY_X11!()] // public
#![doc = crate::_doc!(modules: crate::sys::device::display; x11)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! # Features
//! The `x11` feature automatically enables: `alloc`, `event`, and `unsafe_ffi`.
//
// - <https://www.x.org/releases/current/doc/>
// - [library functions](https://www.x.org/releases/current/doc/man/man3/)
//

crate::mods_in! {
    mod_ _raw; //

    mod atoms; // (XAtoms)
    mod display; // XDisplay
    mod error; // XError
    mod event; // XEvent
    mod image; // XImageMode, (XImageFormat), (XImageStore).
    mod_ runtime; // XFrontend, XPresent, XRasterRender, (XBackend), (XFrameCtx), (XPresenter)
    mod_ surface; // XCpuBuffer, XShmBuffer, (XShmCaps), (XSurface), (XSurfaceStorage)
    #[cfg(all(feature = "ui", feature = "font"))]
    mod_ ui; // UI realization for X11 pixel surfaces
    mod window; // XWindow
    mod xkb; // (KeyRepeatFilter), (XkbInfo), (XkbState)
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            display::*,
            error::*,
            event::*,
            image::*,
            runtime::_all::*,
            surface::_all::*,
            window::*,
        };
        #[cfg(all(feature = "ui", feature = "font"))]
        pub use super::ui::_all::*;
    }
    _crate_internals {
        pub(crate) use super::{
            _raw::_crate_internals::*,

            atoms::*,
            xkb::*,
        };
    }
}
