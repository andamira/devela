// devela::sys::device::display::x11::surface

mod base; // XSurfaceFrame, (XSurface), (XSurfaceStorage)
mod cpu; // XCpuBuffer

#[cfg(ffi_xcb_shm··)]
mod shm; // XShmBuffer, (XShmCaps)

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            base::*,
            cpu::*,
        };
        #[cfg(ffi_xcb_shm··)]
        pub use super::shm::*;
    }
}
