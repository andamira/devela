// devela/src/sys/device/display/x11/surface/_.rs

crate::mods_in! {
    mod base; // XSurfaceFrame, (XSurface), (XSurfaceStorage)
    mod cpu; // XCpuBuffer

    #[cfg(ffi_xcb_shm··)]
    mod shm; // XShmBuffer, (XShmCaps)
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            base::*,
            cpu::*,
        };
        #[cfg(ffi_xcb_shm··)]
        pub use super::shm::*;
    }
}
