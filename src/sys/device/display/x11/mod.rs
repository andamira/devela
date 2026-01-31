// devela::sys::device::display::x11
//
#![doc = crate::_DOC_SYS_DEVICE_DISPLAY_X11!()] // public
#![doc = crate::_doc!(modules: crate::sys::device::display; x11)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
// - <https://www.x.org/releases/current/doc/>
// - [library functions](https://www.x.org/releases/current/doc/man/man3/)
//

// private items
mod atoms; // (XAtoms)
mod raw;
mod xkb; // (KeyRepeatFilter), (XkbInfo), (XkbState)

// public items
mod display; // XDisplay
mod error; // XError
mod event; // XEvent
// mod shm; // XShm
// mod surface; // XSurface
mod window; // XWindow

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            display::*,
            error::*,
            event::*,
            // shm::*,
            // surface::*,
            window::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            atoms::*,
            raw::*,
            xkb::*,
        };
    }
}
