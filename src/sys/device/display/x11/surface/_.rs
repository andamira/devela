// devela/src/sys/device/display/x11/surface/_.rs

crate::mods_in! {
    mod _base; // (XSurface), (XSurfaceStorage)
    mod frame; // XSurfaceFrame
    mod cpu; // XCpuBuffer

    #[cfg(ffi_xcb_shm··)]
    mod shm; // XShmBuffer, (XShmCaps)
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            frame::XSurfaceFrame,
            cpu::XCpuBuffer,
        };
        #[cfg(ffi_xcb_shm··)]
        pub use super::shm::XShmBuffer;
    }
    _crate_internals {
        pub(crate) use super::{
            _base::{XSurface, XSurfaceStorage},
        };
        #[cfg(ffi_xcb_shm··)]
        pub(crate) use super::shm::XShmCaps;
    }
}
