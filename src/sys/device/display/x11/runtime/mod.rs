// devela/src/sys/device/display/x11/runtime/mod.rs

mod frontend; // (XBackend), (XFrameCtx), XFrontend
mod present; // XPresent, (XPresenter), XRasterRender

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            frontend::*,
            present::*,
        };
    }
}
