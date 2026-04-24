// devela::sys::device::display::x11::surface

mod base; // XSurfaceFrame, (XSurface), (XSurfaceStorage)
mod cpu; // XCpuBuffer
mod shm; // XShmBuffer, (XShmCaps)

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            base::*,
            cpu::*,
            shm::*,
        };
    }
}
