// devela::sys::device::display::x11
//
#![doc = crate::_DOC_SYS_DEVICE_DISPLAY_X11!()] // public
#![doc = crate::_doc!(modules: crate::sys::device::display; x11)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
// - <https://www.x.org/releases/current/doc/>
// - [library functions](https://www.x.org/releases/current/doc/man/man3/)
//

mod _raw; //

mod atoms; // (XAtoms)
mod display; // XDisplay
mod error; // XError
mod event; // XEvent
mod image; // XImageMode, (XImageFormat), (XImageStore).
mod runtime; // XFrontend, XPresent, XRasterRender, (XBackend), (XFrameCtx), (XPresenter)
mod surface; // XCpuBuffer, XShmBuffer, (XShmCaps), (XSurface), (XSurfaceStorage)
mod window; // XWindow
mod xkb; // (KeyRepeatFilter), (XkbInfo), (XkbState)

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            display::*,
            error::*,
            event::*,
            image::*,
            runtime::*,
            surface::*,
            window::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _raw::*,
            atoms::*,
            xkb::*,
        };
    }
}
