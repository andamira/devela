// devela::sys::display::x11
//
//!
// - <https://www.x.org/releases/current/doc/>
// - [library functions](https://www.x.org/releases/current/doc/man/man3/)
//

// private items
mod raw;
// mod xkb;

// public items
mod display; // XDisplay
mod error; // XError
mod event; // XEvent
// mod shm; // XShm
// mod surface; // XSurface TODO
mod window; // XWindow

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
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
            // xkb::XkbState,
            raw::*,
        };
    }
}
